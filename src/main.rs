// Implements a basic state channel layer2

// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Import the Stylus SDK along with alloy primitive types for use in our program.
use stylus_sdk::{alloy_primitives::U256, prelude::*};

// Define storage structs we'll need.
sol_storage! {
    // does stylus even pack structs?
    pub struct Channel {
        address counterparty0;
        address counterparty1;
        uint64 timestamp;
        uint32 asset_id;
        uint256 balance0;
        uint256 balance1;
    }

    #[entrypoint]
    pub struct Stasis {
        mapping(address => Channel) channels;
        mapping(uint32 => address) asset_ids;
    }
}

/// Define an implementation of the generated Counter struct, defining a set_number
/// and increment method using the features of the Stylus SDK.
#[external]
impl Stasis {
    pub fn register_asset(
        &mut self, 
        new_asset: Address,
        index: u32
    ) -> Result<(), Vec<u8>> {
        if self.asset_ids.contains_key(index) {
            return Err(b"Asset already registered");
        }

        self.asset_ids.insert(index, new_asset);
    }

    
    
}
