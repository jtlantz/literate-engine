use std::{convert::Infallible, str::FromStr};

use crate::account::AccountId;

pub(crate) enum LineItemType {
    Deposit,
    Withdrawl,
    Dispute,
    Resolve,
    Chargeback,
    Unknown,
}

impl FromStr for LineItemType {
    /// Mapping to unknown so this won't error out at this fn level
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "deposit" => Self::Deposit,
            "withdrawl" => Self::Withdrawl,
            "dispute" => Self::Dispute,
            "resolve" => Self::Resolve,
            "chargeback" => Self::Chargeback,
            _ => Self::Unknown,
        })
    }
}

pub(crate) struct InputLineItems {
    r#type: LineItemType,
    cx: u16,
    tx: u32,
    /// Alternatively to Option, could set this as 0 when the line item type is not deposit or
    /// withdrawl
    amount: Option<f32>,
}

pub(crate) struct OutputLineItems {
    cx: AccountId,
}
