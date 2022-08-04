/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen, env};

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    
}

// Define the default, which automatically initializes the contract
impl Default for Contract{
    fn default() -> Self{
        Self{}
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    /**
     * Responds with the signer and predecessor wallet addresses in a string
     */
    pub fn get_address_info(&self, msg: String) -> String {
        let signer = env::signer_account_id();
        let predecessor = env::predecessor_account_id();

        let mut result = String::from("Accounts:\n");
        result.push_str("Signer Address: ");
        result.push_str(signer.as_str());


        result.push_str("\nPredecessor Address: ");
        result.push_str(predecessor.as_str());


        result.push_str("\nCaller Message: ");
        result.push_str(msg.as_str());

        result
    }
}