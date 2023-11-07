// Import necessary dependencies and traits.
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise};

// Define constants for maximum vesting amounts.
const MAX_VEST_AMOUNT: Balance = 1000;
const MAX_PER_VEST_AMOUNT: Balance = 100;

// Annotate the struct with the #[near_bindgen] attribute to make it a near smart contract.
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct VestingContract {
    // Fields to store contract state.
    pub owner_id: AccountId,         // The owner's account ID.
    pub total_tokens: Balance,      // Total tokens available for vesting.
    pub vested_tokens: Balance,     // Tokens already vested.
    pub beneficiary_id: AccountId,  // The beneficiary's account ID.
}

// Implement the Default trait for the VestingContract struct.
impl Default for VestingContract {
    // The default implementation, called when uninitialized contract is accessed.
    fn default() -> Self {
        env::panic(b"This contract should be initialized before usage");
    }
}

// Annotate the implementation block with #[near_bindgen] to expose methods as entry points.
#[near_bindgen]
impl VestingContract {
    // Initialize the contract with provided parameters.
    #[init]
    pub fn new(owner_id: AccountId, total_tokens: Balance, beneficiary_id: AccountId) -> Self {
        // Ensure the contract is not already initialized.
        assert!(!env::state_exists(), "Already initialized");
        
        // Create a new instance of the VestingContract with initial parameters.
        Self {
            owner_id,
            total_tokens,
            vested_tokens: 0,
            beneficiary_id,
        }
    }

    // Vest tokens from the contract to the beneficiary.
    pub fn vest_tokens(&mut self, tokens: Balance) {
        // Ensure only the owner can vest tokens.
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only the owner can vest tokens");

        // Ensure the vesting amount per transaction is within limits.
        assert!(tokens <= MAX_PER_VEST_AMOUNT, "Vesting amount per transaction exceeds the maximum per vest limit");
        assert!(tokens <= MAX_VEST_AMOUNT, "Vesting amount exceeds the maximum limit");

        // Increase the vested tokens.
        self.vested_tokens += tokens;

        // Ensure the vested tokens do not exceed the total tokens available.
        assert!(self.vested_tokens <= self.total_tokens, "Cannot vest more");

        // Transfer the vested tokens to the beneficiary.
        Promise::new(self.beneficiary_id.clone()).transfer(tokens);
    }
}
