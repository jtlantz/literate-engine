pub(crate) type TxId = u32;

#[derive(Debug)]
pub(crate) struct Transaction {
    #[allow(unused)]
    tx_type: TxType,
    amount: f32,
    disputed: DisputeType,
}

// TODO: implement From for this for easier parsing from line items, would couple them a little bit
// but more ergonomic
impl Transaction {
    pub(crate) fn new(tx_type: TxType, amount: f32) -> Self {
        Self {
            tx_type,
            amount,
            disputed: DisputeType::Nuetral,
        }
    }

    pub(crate) fn amount(&self) -> f32 {
        self.amount
    }

    pub(crate) fn dispute(&mut self) {
        self.disputed = DisputeType::Dispute;
    }

    pub(crate) fn resolve(&mut self) {
        self.disputed = DisputeType::Resolve;
    }

    pub(crate) fn charge_back(&mut self) {
        self.disputed = DisputeType::ChargeBack;
    }
}

#[derive(Debug)]
pub(crate) enum TxType {
    Deposit,
    Withdrawl,
}

#[derive(Debug)]
pub(crate) enum DisputeType {
    /// No dispute has been placed on this tx
    Nuetral,
    /// An initiated dispute referring to a specific transaction
    Dispute,
    /// Resolve, meaning we make funds available
    Resolve,
    /// ChargeBack, we freeze the account, freeze funds
    /// (still allow withdrawls?)
    ChargeBack,
}
