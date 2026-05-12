use near_sdk::{near_bindgen, AccountId};

use crate::storage::*;

#[near_bindgen]
impl MarketFactory {
    /// Returns every market account tracked by the factory.
    pub fn get_markets_list(&self) -> Vec<AccountId> {
        self.markets.to_vec()
    }

    /// Returns the number of markets tracked by the factory.
    pub fn get_markets_count(&self) -> u64 {
        self.markets.len()
    }

    /// Returns a paginated slice of tracked market accounts.
    pub fn get_markets(&self, from_index: u64, limit: u64) -> Vec<AccountId> {
        let elements = self.markets.as_vector();

        (from_index..std::cmp::min(from_index + limit, elements.len()))
            .filter_map(|index| elements.get(index))
            .collect()
    }
}
