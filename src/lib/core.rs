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

    /// exports the records to a string
    /// this is pretty inefficient on memory, and would prefer streaming this to a file writer,
    /// but the requirements ask for it to be in std_out so will print in the bin fn where this is
    /// called.
    pub fn export_records(&self) -> Result<String> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        for (_cx, account) in self.accounts.iter() {
            let record = OutputLineItem::from(account.clone());
            wtr.serialize(&record)?;
        }

        // not super efficient as it buffers the whole string
        // Could be refactored easily to dump straight to a file instead of redirecting stdout to a
        // file when calling the bin.
        let data = String::from_utf8(wtr.into_inner()?)?;
        Ok(data)
    }

    /// processes an individual line item, if there was an error reading the line item then we'll
    /// return an error, log, and continue to the next line item
    fn process_item(&mut self, line_item: InputLineItem) -> Result<()> {
        // If cx exists update their transaction history, else create a new customer record
        let cx = line_item.client;
        let customer_record = self.accounts.entry(cx).or_insert(Account::new(cx));

        let res = match line_item.r#type {
            crate::line_items::LineItemType::Deposit => customer_record.deposit(line_item),
            crate::line_items::LineItemType::Withdrawl => customer_record.withdrawl(line_item),
            crate::line_items::LineItemType::Dispute => customer_record.dispute(line_item),
            crate::line_items::LineItemType::Resolve => customer_record.resolve(line_item),
            crate::line_items::LineItemType::Chargeback => customer_record.chargeback(line_item),
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
}
