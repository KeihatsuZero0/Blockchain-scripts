use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::near_bindgen;
use near_sdk::{env, near_bindgen, PanicOnDefault, Promise};
use near_sdk::{AccountId, Balance, PromiseResult};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct MyToken {
    total_supply: Balance,
    balances: UnorderedMap<AccountId, Balance>,
    allowances: UnorderedMap<(AccountId, AccountId), Balance>,
}

#[near_bindgen]
impl MyToken {
    #[init]
    pub fn new(total_supply: Balance) -> Self {
        assert!(env::state_exists(), "Already initialized");
        assert!(total_supply > 0, "Total supply must be greater than 0");

        Self {
            total_supply,
            balances: UnorderedMap::new(b"b".to_vec()),
            allowances: UnorderedMap::new(b"a".to_vec()),
        }
    }

    pub fn get_total_supply(&self) -> Balance {
        self.total_supply
    }

    pub fn get_balance(&self, account_id: AccountId) -> Balance {
        self.balances.get(&account_id).unwrap_or(0)
    }

    pub fn transfer(&mut self, recipient: AccountId, amount: Balance) -> Promise {
        assert_ne!(env::signer_account_id(), recipient, "Cannot transfer to self");
        assert!(amount > 0, "Transfer amount must be greater than 0");

        let sender_id = env::signer_account_id();
        let sender_balance = self.balances.get(&sender_id).expect("Sender balance not found"); 
        assert!(sender_balance >= amount, "Insufficient balance");

        self.balances.insert(&sender_id, &(sender_balance - amount));
        self.balances.insert(&recipient, &(self.get_balance(&recipient) + amount));

        Promise::new(recipient).transfer(amount)
    }

    pub fn approve(&mut self, account_id: AccountId, amount: Balance) -> Promise {
        assert_ne!(env::signer_account_id(), account_id, "Cannot approve self");
        assert!(amount > 0, "Approval amount must be greater than 0");
        self.allowances.insert(&(&env::signer_account_id(), &account_id), &amount);
        Promise::new(env::current_account_id()).transfer(0) // No funds are transferred
    }

    pub fn transfer_from(&mut self, sender: AccountId, recipient: AccountId, amount: Balance) -> Promise {
        assert_ne!(env::signer_account_id(), sender, "Cannot transfer from self"); 
        assert_ne!(env::signer_account_id(), recipient, "Cannot transfer to self");
        assert!(amount > 0, "Transfer amount must be greater than 0");

        let allowance = self.allowances.get(&(&sender, &env::signer_account_id())).expect("Insufficient allowance"); 
        assert!(allowance >= amount, "Insufficient allowance");

        self.balances.insert(&sender, &(self.get_balance(&sender) - amount));
        self.balances.insert(&recipient, &(self.get_balance(&recipient) + amount));
        self.allowances.insert(&(&sender, &env::signer_account_id()), &(allowance - amount));

        Promise::new(recipient).transfer(amount)
    }

    pub fn get_allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
        self.allowances.get(&(&owner, &spender)).unwrap_or(0)
    }
}