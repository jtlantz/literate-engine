use anyhow::Result;
use std::collections::HashMap;

use crate::{
    line_items::InputLineItem,
    transaction::{Transaction, TxId, TxType},
};

/// typed AccountId for easier readability
pub(crate) type AccountId = u16;

#[derive(Debug)]
pub(crate) struct Account {
    cx: AccountId,
    /// the processor will take mutable Accounts so we can update
    /// this with lockability by each new account object
    /// does not need a mutex as this won't be async
    transactions: HashMap<TxId, Transaction>,

    available: f32,
    held: f32,
    /// total amount of funds in the account, can be computed based off of available + held
    frozen: bool,
}

impl Account {
    pub(crate) fn new(cx: u16) -> Self {
        Self {
            cx,
            transactions: HashMap::new(),
            available: 0.0,
            held: 0.0,
            frozen: false,
        }
    }

    // TODO: fail deposit and withdrawl if amounts are empty? weird to withdrawl nothing but i
    // suppose it's ok

    pub(crate) fn deposit(&mut self, tx: InputLineItem) -> Result<()> {
        if self.frozen {
            return Err(anyhow::anyhow!("Customer account is frozen"));
        }

        let amount = tx.amount.unwrap_or_else(|| 0.0);
        let transaction = Transaction::new(TxType::Deposit, amount);

        // update funds in the account
        self.available += amount;

        self.transactions.insert(tx.tx, transaction);

        Ok(())
    }

    pub(crate) fn withdrawl(&mut self, tx: InputLineItem) -> Result<()> {
        if self.frozen {
            return Err(anyhow::anyhow!("Customer account is frozen"));
        }

        let amount = tx.amount.unwrap_or_else(|| 0.0);
        // validate we have available funds in account
        if self.available < amount {
            return Err(anyhow::anyhow!("Insufficient Funds available"));
        }

        let transaction = Transaction::new(TxType::Withdrawl, amount);

        self.available -= amount;

        self.transactions.insert(tx.tx, transaction);

        Ok(())
    }

    pub(crate) fn dispute(&mut self, tx: InputLineItem) -> Result<()> {
        // validate tx is in customer ledger
        let cus_tx = self
            .transactions
            .get_mut(&tx.tx)
            .ok_or_else(|| anyhow::anyhow!("transaction doesn't exist in customer ledger"))?;

        // hold customer funds
        self.held += cus_tx.amount();
        // assuming here we can make the customer's available balance go negative
        // no protection on this
        self.available -= cus_tx.amount();
        cus_tx.dispute();

        Ok(())
    }

    pub(crate) fn resolve(&mut self, tx: InputLineItem) -> Result<()> {
        // validate tx is in customer ledger
        let cus_tx = self
            .transactions
            .get_mut(&tx.tx)
            .ok_or_else(|| anyhow::anyhow!("transaction doesn't exist in customer ledger"))?;

        // release funds
        self.held -= cus_tx.amount();
        self.available += cus_tx.amount();

        cus_tx.resolve();

        Ok(())
    }

    pub(crate) fn chargeback(&mut self, tx: InputLineItem) -> Result<()> {
        // validate tx is in customer ledger
        let cus_tx = self
            .transactions
            .get_mut(&tx.tx)
            .ok_or_else(|| anyhow::anyhow!("transaction doesn't exist in customer ledger"))?;

        // release funds but freeze customer account
        self.held -= cus_tx.amount();
        self.available -= cus_tx.amount();
        cus_tx.charge_back();

        self.frozen = true;

        Ok(())
    }

    pub(crate) fn total(&self) -> f32 {
        self.available + self.held
    }
}
