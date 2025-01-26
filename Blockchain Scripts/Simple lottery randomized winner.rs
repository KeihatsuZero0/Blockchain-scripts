use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedSet;
use near_sdk::near_bindgen;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, PanicOnDefault, Promise};
use near_sdk::{AccountId, Balance, PromiseResult};
use rand::{prelude::Rng, SeedableRng};
use rand_chacha::ChaChaRng;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Lottery {
    // ... (Other relevant fields)
    players: UnorderedSet<AccountId>,
    ticket_price: Balance,
    winner: Option<AccountId>,
    rng_seed: [u8; 32], 
}

#[near_bindgen]
impl Lottery {
    #[init]
    pub fn new(ticket_price: Balance) -> Self {
        assert!(env::state_exists(), "Already initialized");
        Self {
            players: UnorderedSet::new(b"p".to_vec()),
            ticket_price,
            winner: None,
            rng_seed: env::random_seed(), 
        }
    }

    pub fn buy_ticket(&mut self) {
        assert!(self.winner.is_none(), "Lottery is already over");
        assert!(env::attached_deposit() >= self.ticket_price, "Insufficient balance");
        self.players.insert(&env::signer_account_id());
    }

    pub fn draw_winner(&mut self) {
        assert!(self.winner.is_none(), "Lottery is already over");
        assert!(self.players.len() > 0, "No players have bought tickets");

        // Generate a random number using the seed
        let mut rng = ChaChaRng::from_seed(self.rng_seed);
        let winner_index = rng.gen_range(0..self.players.len());

        // Get the winner's account ID
        let mut winner_id = self.players.iter().nth(winner_index).unwrap().clone(); 

        // Transfer winnings to the winner
        Promise::new(winner_id).transfer(env::attached_deposit() * (self.players.len() as u128 - 1)); 

        self.winner = Some(winner_id);
    }
}