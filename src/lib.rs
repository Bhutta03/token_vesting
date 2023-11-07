
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise};

const MAX_VEST_AMOUNT: Balance = 1000;
const MAX_PER_VEST_AMOUNT: Balance = 100;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct VestingContract {
    pub owner_id: AccountId,
    pub total_tokens: Balance,
    pub vested_tokens: Balance,
    pub beneficiary_id: AccountId,
}

impl Default for VestingContract {
    fn default() -> Self {
        env::panic(b"This contract should be initialized before usage")
    }
}

#[near_bindgen]
impl VestingContract{
    #[init]
    pub fn new(owner_id: AccountId, total_tokens: Balance, beneficiary_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner_id,
            total_tokens,
            vested_tokens: 0,
            beneficiary_id,
        }
    }

    pub fn vest_tokens(&mut self, tokens: Balance) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only the owner can vest tokens");
        assert!(tokens <= MAX_PER_VEST_AMOUNT, "Vesting amount per transaction exceeds the maximum per vest limit");
        assert!(tokens <= MAX_VEST_AMOUNT, "Vesting amount exceeds the maximum limit");

        self.vested_tokens += tokens;
        assert!(self.vested_tokens <= self.total_tokens, "Cannot vest more");
        Promise::new(self.beneficiary_id.clone()).transfer(tokens);
    }
}