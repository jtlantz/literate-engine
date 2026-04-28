use dashmap::DashMap;

use crate::account::Account;

/// typed AccountId for easier readability
type AccountId = u16;

/// A generic system to process the csv input and record keep accounts while processing
pub struct System {
    // since clients are given on dispute actions, we'll record keep transactions at the account
    // level instead of by the system
    // Using a dashmap as it's an ergonomic wrapper of RwLock<HashMap<K, V>>
    // probably don't need this since we're not doing anything async though
    // might remove later
    accounts: DashMap<AccountId, Account>,
}

impl System {
    pub fn new() -> Self {
        Self {
            accounts: DashMap::new(),
        }
    }
}
