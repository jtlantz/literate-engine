pub(crate) type TxId = u32;

#[derive(Debug)]
pub(crate) struct Transaction {
    tx_type: TxType,
    amount: f32,
}

// TODO: implement From for this for easier parsing from line items, would couple them a little bit
// but more ergonomic
impl Transaction {
    pub(crate) fn new(tx_type: TxType, amount: f32) -> Self {
        Self { tx_type, amount }
    }
}

#[derive(Debug)]
pub(crate) enum TxType {
    Deposit,
    Withdrawl,
    /// An initiated dispute referring to a specific transaction
    Dispute(TxId),
    /// Resolve, meaning we make funds available
    Resolve(TxId),
    /// ChargeBack, we freeze the account, freeze funds
    /// (still allow withdrawls?)
    ChargeBack(TxId),
}
