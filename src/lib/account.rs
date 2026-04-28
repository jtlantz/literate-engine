use std::collections::HashMap;

use crate::transaction::{Transaction, TxId};

/// typed AccountId for easier readability
pub(crate) type AccountId = u16;

pub(crate) struct Account {
    cx: AccountId,
    /// the processor will take mutable Accounts so we can update
    /// this with lockability by each new account object
    /// does not need a mutex as this won't be async
    transactions: HashMap<TxId, Transaction>,
    frozen: bool,
}
