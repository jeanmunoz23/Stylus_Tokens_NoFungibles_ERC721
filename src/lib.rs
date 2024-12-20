//! Stylus Educateth Example
//!
//! This contract is an example implementation of an ERC721-like token in Stylus.
//! It allows minting unique tokens and retrieving a static token URI for all tokens.
//!
//! The program is ABI-equivalent with Solidity, which means you can call it from both Solidity and Rust.
//! To do this, run `cargo stylus export-abi`.
//!
//! Note: this code is a template-only and has not been audited.

// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Import items from the SDK.
use stylus_sdk::{alloy_primitives::U256, prelude::*};

// Define persistent storage for the contract.
sol_storage! {
    #[entrypoint]
    pub struct Educateth {
        uint256 next_token_id;
        mapping(address => StorageList<U256>) owner_to_tokens;
    }
}

#[public]
impl Educateth {
    pub fn token_uri(&self, _token_id: U256) -> String {
        "https://gateway.lighthouse.storage/ipfs/bafkreibtrimcpcuhf2oygrb6c7xspt4nrsqjww6k3mf5x67xx6zj2qvlti/".to_string()
    }

    pub fn safe_mint(&mut self, to: Address) {
        let caller = self.get_caller();
        let owner = self.get_owner();

        assert_eq!(caller, owner, "Caller is not the owner");

        let token_id = self.next_token_id.get();
        self.next_token_id.set(token_id + U256::from(1));

        let mut tokens = self.owner_to_tokens.get(to).unwrap_or_default();
        tokens.push(token_id);
        self.owner_to_tokens.insert(to, tokens);
    }

    pub fn get_owner(&self) -> Address {
        Address::from_slice(&[0x00; 20])
    }

    pub fn tokens_of_owner(&self, owner: Address) -> Vec<U256> {
        self.owner_to_tokens.get(owner).unwrap_or_default()
    }
}

