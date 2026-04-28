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
    total: f32,

    frozen: bool,
}

impl Account {
    pub(crate) fn new(cx: u16) -> Self {
        Self {
            cx,
            transactions: HashMap::new(),
            available: 0.0,
            held: 0.0,
            total: 0.0,
            frozen: false,
        }
    }

    // TODO: fail deposit and withdrawl if amounts are empty? weird to withdrawl nothing but i
    // suppose it's ok

    pub(crate) fn deposit(&mut self, tx: InputLineItem) -> Result<()> {
        let amount = tx.amount.unwrap_or_else(|| 0.0);
        let transaction = Transaction::new(TxType::Deposit, amount);

        // TODO: This overwrites transactions if one with the previous id already existed
        // evaluated if this needs to account for that or not
        self.transactions.insert(tx.tx, transaction);

        Ok(())
    }

    pub(crate) fn withdrawl(&mut self, tx: InputLineItem) -> Result<()> {
        let amount = tx.amount.unwrap_or_else(|| 0.0);
        let transaction = Transaction::new(TxType::Withdrawl, amount);

        // TODO: This overwrites transactions if one with the previous id already existed
        // evaluated if this needs to account for that or not
        self.transactions.insert(tx.tx, transaction);

        Ok(())
    }

    pub(crate) fn dispute(&mut self, _tx: InputLineItem) -> Result<()> {
        Ok(())
    }

    pub(crate) fn resolve(&mut self, _tx: InputLineItem) -> Result<()> {
        Ok(())
    }

    pub(crate) fn chargeback(&mut self, _tx: InputLineItem) -> Result<()> {
        Ok(())
    }
}
