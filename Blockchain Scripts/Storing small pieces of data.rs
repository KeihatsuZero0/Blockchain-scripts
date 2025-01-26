use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::near_bindgen;
use near_sdk::{env, near_bindgen, PanicOnDefault};
use near_sdk::{AccountId, Balance};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct SimpleStorage {
    data: UnorderedMap<AccountId, String>,
}

#[near_bindgen]
impl SimpleStorage {
    #[init]
    pub fn new() -> Self {
        assert!(env::state_exists(), "Already initialized");
        Self {
            data: UnorderedMap::new(b"s".to_vec()),
        }
    }

    pub fn store(&mut self, account_id: AccountId, data: String) {
        self.data.insert(&account_id, &data);
    }

    pub fn get(&self, account_id: AccountId) -> Option<String> {
        self.data.get(&account_id)
    }
}