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
    pub struct Funding {
        address counterparty0;
        uint256 balance0;
        address counterparty1;
        uint256 balance1;
        address asset;
        bytes sig0;
        bytes sig1;
    }

    pub struct Commitment {
        uint256 balance0;
        uint256 balance1;
        uint32 nonce;
        bytes sig0;
        bytes sig1;
    }

    pub struct Justice {
        Commitment commitment;
        uint64 timestamp;
    }

    #[entrypoint]
    pub struct Stasis {
        mapping(uint256 => Funding) channels;
        mapping(uint256 => Funding) justice_queue;
        uint256 channel_ids;
    }
}

/// Define an implementation of the generated Counter struct, defining a set_number
/// and increment method using the features of the Stylus SDK.
#[external]
impl Stasis {
}
