/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use std::str::FromStr;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, Promise, ext_contract, AccountId, env, Gas};

// Gas for callback function
const XCC_GAS: Gas = Gas(7_000_000_000_000);

// External smart contract module configuration
#[ext_contract(ext_accounts_contract)]
pub trait Accounts {
    fn get_address_info(&self, msg: String) -> String;
}

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
     * Makes a cross-contract call to an external contract (i.e accounts.c4-sankore.testnet)
     */
    pub fn call_accounts_contract(&self, msg: String) -> Promise {
        // Invokes get_address_info function of accounts.c4-sankore.testnet smart contract
        ext_accounts_contract::ext(self.accounts_account_id()).get_address_info(msg)
        // Handles the response of the cross-contract call using response_callback function
        .then(
            Self::ext(env::current_account_id())
            .with_static_gas(XCC_GAS)
            .response_callback()
        )
    }

    /**
     * The callback function handling cross-contract call response
     */
    #[private]
    pub fn response_callback(#[callback_unwrap] response: String) -> String {
        let mut result = String::from(response);
        result.push_str(" : processed in XCC Smart Contract");
        result
    }

    /**
     * Returns the wallet address of the external contract (i.e accounts.c4-sankore.testnet)
     */
    fn accounts_account_id(&self) -> AccountId {
        let acc = AccountId::from_str("accounts.c4-sankore.testnet").unwrap();
        acc
    }
    
}