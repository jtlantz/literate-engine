use std::{collections::HashMap, path::PathBuf};

use crate::{
    account::Account,
    line_items::{InputLineItem, OutputLineItem},
};
use anyhow::Result;

/// typed AccountId for easier readability
type AccountId = u16;

/// A generic system to process the csv input and record keep accounts while processing
pub struct System {
    // since clients are given on dispute actions, we'll record keep transactions at the account
    // level instead of by the system
    // Using a dashmap as it's an ergonomic wrapper of RwLock<HashMap<K, V>>
    // probably don't need this since we're not doing anything async though
    // might remove later
    accounts: HashMap<AccountId, Account>,
}

impl System {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }

    /// Process an individual file of transactions against this sytem.
    /// Expects to take a mutable copy of self since it will perform mutating actions on the
    /// internal account mapping.
    pub fn process(&mut self, path: impl Into<PathBuf>) -> Result<()> {
        let mut csv_parser = csv::Reader::from_path(path.into())?;

        for record in csv_parser.deserialize() {
            let record: InputLineItem = record?;

            let _ = self.process_item(record);
        }

        Ok(())
    }

    /// exports the records to stdout
    pub fn export_records(&self) -> Result<()> {
        let stdout = std::io::stdout();
        let handle = stdout.lock();

        let mut wtr = csv::Writer::from_writer(handle);

        for (_cx, account) in self.accounts.iter() {
            let record = OutputLineItem::from(account);
            wtr.serialize(&record)?;
        }

        wtr.flush()?;

        // not super efficient as it buffers the whole string
        // Could be refactored easily to dump straight to a file instead of redirecting stdout to a
        // file when calling the bin.
        Ok(())
    }

    /// processes an individual line item, if there was an error reading the line item then we'll
    /// return an error, log, and continue to the next line item
    fn process_item(&mut self, line_item: InputLineItem) -> Result<()> {
        // If cx exists update their transaction history, else create a new customer record
        let cx = line_item.client;
        let customer_record = self.accounts.entry(cx).or_insert(Account::new(cx));

        let res = match line_item.r#type {
            crate::line_items::LineItemType::Deposit => customer_record.deposit(line_item),
            crate::line_items::LineItemType::Withdrawal => customer_record.withdrawal(line_item),
            crate::line_items::LineItemType::Dispute => customer_record.dispute(line_item),
            crate::line_items::LineItemType::Resolve => customer_record.resolve(line_item),
            crate::line_items::LineItemType::Chargeback => customer_record.charge_back(line_item),
            crate::line_items::LineItemType::Unknown => {
                Err(anyhow::anyhow!("unknown transaction type was passed"))
            }
        };

        Ok(res?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::line_items::{InputLineItem, LineItemType};

    fn make_item(
        tx_type: LineItemType,
        client: u16,
        tx: u32,
        amount: Option<f32>,
    ) -> InputLineItem {
        InputLineItem {
            r#type: tx_type,
            client,
            tx,
            amount,
        }
    }

    #[test_log::test]
    fn basic_deposit_withdrawl_test() {
        let mut test_system = System::new();
        // validates if the system can run and pull in this basic test
        let _ = test_system
            .process("src/test-resources/basic_test.csv")
            .unwrap();

        let _ = test_system.export_records().unwrap();

        assert_eq!(test_system.accounts.get(&1).unwrap().total(), 1.5);
        assert_eq!(test_system.accounts.get(&2).unwrap().total(), 2.0);
        assert_eq!(test_system.accounts.get(&2).unwrap().held(), 0.0);
    }

    /// Dispute moves funds from available → held; resolve should return them to available.
    #[test_log::test]
    fn dispute_then_resolve_returns_funds() {
        let mut system = System::new();
        system
            .process_item(make_item(LineItemType::Deposit, 1, 1, Some(100.0)))
            .unwrap();
        system
            .process_item(make_item(LineItemType::Dispute, 1, 1, None))
            .unwrap();

        let acct = system.accounts.get(&1).unwrap();
        assert_eq!(acct.held(), 100.0, "funds should be held after dispute");
        assert_eq!(acct.avaliable(), 0.0, "available should be 0 after dispute");

        system
            .process_item(make_item(LineItemType::Resolve, 1, 1, None))
            .unwrap();

        let acct = system.accounts.get(&1).unwrap();
        assert_eq!(acct.held(), 0.0, "held should be 0 after resolve");
        assert_eq!(
            acct.avaliable(),
            100.0,
            "available should be restored after resolve"
        );
        assert_eq!(acct.total(), 100.0);
        assert!(!acct.locked());
    }

    /// Chargeback after a dispute should freeze the account and zero out all funds.
    #[test_log::test]
    fn dispute_then_chargeback_freezes_account() {
        let mut system = System::new();
        system
            .process_item(make_item(LineItemType::Deposit, 1, 1, Some(100.0)))
            .unwrap();
        system
            .process_item(make_item(LineItemType::Dispute, 1, 1, None))
            .unwrap();
        system
            .process_item(make_item(LineItemType::Chargeback, 1, 1, None))
            .unwrap();

        let acct = system.accounts.get(&1).unwrap();
        assert!(acct.locked(), "account should be frozen after chargeback");
        assert_eq!(acct.held(), 0.0, "held should be cleared after chargeback");
        assert_eq!(
            acct.avaliable(),
            0.0,
            "available should remain 0 (was moved to held during dispute)"
        );
        assert_eq!(acct.total(), 0.0, "total should be 0 after chargeback");
    }

    /// A frozen account should reject further deposits and withdrawals.
    #[test_log::test]
    fn frozen_account_rejects_transactions() {
        let mut system = System::new();
        system
            .process_item(make_item(LineItemType::Deposit, 1, 1, Some(50.0)))
            .unwrap();
        system
            .process_item(make_item(LineItemType::Dispute, 1, 1, None))
            .unwrap();
        system
            .process_item(make_item(LineItemType::Chargeback, 1, 1, None))
            .unwrap();

        assert!(
            system
                .process_item(make_item(LineItemType::Deposit, 1, 2, Some(10.0)))
                .is_err()
        );
        assert!(
            system
                .process_item(make_item(LineItemType::Withdrawal, 1, 3, Some(10.0)))
                .is_err()
        );
    }

    /// A withdrawal that exceeds the available balance should fail and leave the balance unchanged.
    #[test_log::test]
    fn withdrawal_insufficient_funds_leaves_balance_unchanged() {
        let mut system = System::new();
        system
            .process_item(make_item(LineItemType::Deposit, 1, 1, Some(50.0)))
            .unwrap();

        let result = system.process_item(make_item(LineItemType::Withdrawal, 1, 2, Some(100.0)));
        assert!(result.is_err(), "withdrawal exceeding balance should error");

        let acct = system.accounts.get(&1).unwrap();
        assert_eq!(
            acct.avaliable(),
            50.0,
            "balance should be unchanged after failed withdrawal"
        );
        assert_eq!(acct.total(), 50.0);
    }

    /// Disputing a transaction ID that doesn't exist in the ledger should return an error.
    #[test_log::test]
    fn dispute_nonexistent_transaction_errors() {
        let mut system = System::new();
        system
            .process_item(make_item(LineItemType::Deposit, 1, 1, Some(50.0)))
            .unwrap();

        let result = system.process_item(make_item(LineItemType::Dispute, 1, 999, None));
        assert!(result.is_err(), "disputing a non-existent tx should error");
    }
}
