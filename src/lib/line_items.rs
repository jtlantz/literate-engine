use std::{convert::Infallible, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::account::{Account, AccountId};

#[derive(Debug)]
pub(crate) enum LineItemType {
    Deposit,
    Withdrawal,
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
            "withdrawal" => Self::Withdrawal,
            "dispute" => Self::Dispute,
            "resolve" => Self::Resolve,
            "chargeback" => Self::Chargeback,
            _ => Self::Unknown,
        })
    }
}

impl<'de> Deserialize<'de> for LineItemType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let item = String::deserialize(deserializer)?;
        // LineItemType is infallible, making unwrap here ok
        Ok(LineItemType::from_str(&item).unwrap())
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct InputLineItem {
    pub(crate) r#type: LineItemType,
    pub(crate) client: u16,
    pub(crate) tx: u32,
    /// Alternatively to Option, could set this as 0 when the line item type is not deposit or
    /// withdrawl
    pub(crate) amount: Option<f32>,
}

#[derive(Debug, Serialize)]
pub(crate) struct OutputLineItem {
    client: AccountId,
    available: f32,
    held: f32,
    total: f32,
    locked: bool,
}

impl From<&Account> for OutputLineItem {
    fn from(account: &Account) -> Self {
        OutputLineItem {
            client: account.cx(),
            available: account.avaliable(),
            held: account.held(),
            total: account.total(),
            locked: account.locked(),
        }
    }
}
