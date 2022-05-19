// Template adopted from https://github.com/paritytech/substrate/blob/master/.maintain/frame-weight-template.hbs

//! Autogenerated weights for pallet_nonfungible
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-01, STEPS: `50`, REPEAT: 200, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// target/release/unique-collator
// benchmark
// --pallet
// pallet-nonfungible
// --wasm-execution
// compiled
// --extrinsic
// *
// --template
// .maintain/frame-weight-template.hbs
// --steps=50
// --repeat=200
// --heap-pages=4096
// --output=./pallets/nonfungible/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_nonfungible.
pub trait WeightInfo {
	fn create_item() -> Weight;
	fn create_multiple_items(b: u32, ) -> Weight;
	fn create_multiple_items_ex(b: u32, ) -> Weight;
	fn burn_item() -> Weight;
	fn set_collection_properties(amount: u32) -> Weight;
	fn delete_collection_properties(amount: u32) -> Weight;
	fn set_token_properties(amount: u32) -> Weight;
	fn delete_token_properties(amount: u32) -> Weight;
	fn set_property_permissions(amount: u32) -> Weight;
	fn transfer() -> Weight;
	fn approve() -> Weight;
	fn transfer_from() -> Weight;
	fn burn_from() -> Weight;
}

/// Weights for pallet_nonfungible using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Nonfungible TokensMinted (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:1 w:1)
	// Storage: Nonfungible TokenData (r:0 w:1)
	// Storage: Nonfungible Owned (r:0 w:1)
	fn create_item() -> Weight {
		(18_450_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Nonfungible TokensMinted (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:1 w:1)
	// Storage: Nonfungible TokenData (r:0 w:4)
	// Storage: Nonfungible Owned (r:0 w:4)
	fn create_multiple_items(b: u32, ) -> Weight {
		(10_228_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((4_392_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(b as Weight)))
	}
	// Storage: Nonfungible TokensMinted (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:4 w:4)
	// Storage: Nonfungible TokenData (r:0 w:4)
	// Storage: Nonfungible Owned (r:0 w:4)
	fn create_multiple_items_ex(b: u32, ) -> Weight {
		(6_543_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((7_175_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(b as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(b as Weight)))
	}
	// Storage: Nonfungible TokenData (r:1 w:1)
	// Storage: Nonfungible TokensBurnt (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:1 w:1)
	// Storage: Nonfungible Allowance (r:1 w:0)
	// Storage: Nonfungible Owned (r:0 w:1)
	fn burn_item() -> Weight {
		(24_554_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}

	fn set_collection_properties(amount: u32) -> Weight {
		// TODO calculate appropriate weight
		(50_000_000 as Weight).saturating_mul(amount as Weight)
	}

	fn delete_collection_properties(amount: u32) -> Weight {
		// TODO calculate appropriate weight
		(50_000_000 as Weight).saturating_mul(amount as Weight)
	}

	fn set_token_properties(amount: u32) -> Weight {
		// TODO calculate appropriate weight
		(50_000_000 as Weight).saturating_mul(amount as Weight)
	}

	fn delete_token_properties(amount: u32) -> Weight {
		// TODO calculate appropriate weight
		(50_000_000 as Weight).saturating_mul(amount as Weight)
	}

	fn set_property_permissions(amount: u32) -> Weight {
		// TODO calculate appropriate weight
		(50_000_000 as Weight).saturating_mul(amount as Weight)
	}

	// Storage: Nonfungible TokenData (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:2 w:2)
	// Storage: Nonfungible Allowance (r:1 w:0)
	// Storage: Nonfungible Owned (r:0 w:2)
	fn transfer() -> Weight {
		(28_339_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Nonfungible TokenData (r:1 w:0)
	// Storage: Nonfungible Allowance (r:1 w:1)
	fn approve() -> Weight {
		(17_616_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Nonfungible Allowance (r:1 w:1)
	// Storage: Nonfungible TokenData (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:2 w:2)
	// Storage: Nonfungible Owned (r:0 w:2)
	fn transfer_from() -> Weight {
		(32_196_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: Nonfungible Allowance (r:1 w:1)
	// Storage: Nonfungible TokenData (r:1 w:1)
	// Storage: Nonfungible TokensBurnt (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:1 w:1)
	// Storage: Nonfungible Owned (r:0 w:1)
	fn burn_from() -> Weight {
		(27_580_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Nonfungible TokensMinted (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:1 w:1)
	// Storage: Nonfungible TokenData (r:0 w:1)
	// Storage: Nonfungible Owned (r:0 w:1)
	fn create_item() -> Weight {
		(18_450_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Nonfungible TokensMinted (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:1 w:1)
	// Storage: Nonfungible TokenData (r:0 w:4)
	// Storage: Nonfungible Owned (r:0 w:4)
	fn create_multiple_items(b: u32, ) -> Weight {
		(10_228_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((4_392_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes((2 as Weight).saturating_mul(b as Weight)))
	}
	// Storage: Nonfungible TokensMinted (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:4 w:4)
	// Storage: Nonfungible TokenData (r:0 w:4)
	// Storage: Nonfungible Owned (r:0 w:4)
	fn create_multiple_items_ex(b: u32, ) -> Weight {
		(6_543_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((7_175_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(b as Weight)))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes((3 as Weight).saturating_mul(b as Weight)))
	}
	// Storage: Nonfungible TokenData (r:1 w:1)
	// Storage: Nonfungible TokensBurnt (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:1 w:1)
	// Storage: Nonfungible Allowance (r:1 w:0)
	// Storage: Nonfungible Owned (r:0 w:1)
	fn burn_item() -> Weight {
		(24_554_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}

	fn set_collection_properties(amount: u32) -> Weight {
		// TODO calculate appropriate weight
		(50_000_000 as Weight).saturating_mul(amount as Weight)
	}

	fn delete_collection_properties(amount: u32) -> Weight {
		// TODO calculate appropriate weight
		(50_000_000 as Weight).saturating_mul(amount as Weight)
	}

	fn set_token_properties(amount: u32) -> Weight {
		// TODO calculate appropriate weight
		(50_000_000 as Weight).saturating_mul(amount as Weight)
	}

	fn delete_token_properties(amount: u32) -> Weight {
		// TODO calculate appropriate weight
		(50_000_000 as Weight).saturating_mul(amount as Weight)
	}

	fn set_property_permissions(amount: u32) -> Weight {
		// TODO calculate appropriate weight
		(50_000_000 as Weight).saturating_mul(amount as Weight)
	}

	// Storage: Nonfungible TokenData (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:2 w:2)
	// Storage: Nonfungible Allowance (r:1 w:0)
	// Storage: Nonfungible Owned (r:0 w:2)
	fn transfer() -> Weight {
		(28_339_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	// Storage: Nonfungible TokenData (r:1 w:0)
	// Storage: Nonfungible Allowance (r:1 w:1)
	fn approve() -> Weight {
		(17_616_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Nonfungible Allowance (r:1 w:1)
	// Storage: Nonfungible TokenData (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:2 w:2)
	// Storage: Nonfungible Owned (r:0 w:2)
	fn transfer_from() -> Weight {
		(32_196_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	// Storage: Nonfungible Allowance (r:1 w:1)
	// Storage: Nonfungible TokenData (r:1 w:1)
	// Storage: Nonfungible TokensBurnt (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:1 w:1)
	// Storage: Nonfungible Owned (r:0 w:1)
	fn burn_from() -> Weight {
		(27_580_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
}
