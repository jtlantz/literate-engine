pub(crate) type TxId = u32;

pub(crate) struct Transaction {
    tx_type: TxType,
}

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
