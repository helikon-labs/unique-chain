// Template adopted from https://github.com/paritytech/substrate/blob/master/.maintain/frame-weight-template.hbs

//! Autogenerated weights for pallet_app_promotion
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-08-09, STEPS: `50`, REPEAT: 80, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// target/release/unique-collator
// benchmark
// pallet
// --pallet
// pallet-app-promotion
// --wasm-execution
// compiled
// --extrinsic
// *
// --template
// .maintain/frame-weight-template.hbs
// --steps=50
// --repeat=80
// --heap-pages=4096
// --output=./pallets/app-promotion/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_app_promotion.
pub trait WeightInfo {
	fn on_initialize() -> Weight;
	fn start_app_promotion() -> Weight;
	fn set_admin_address() -> Weight;
	fn stake() -> Weight;
	fn unstake() -> Weight;
	fn recalculate_stake() -> Weight;
}

/// Weights for pallet_app_promotion using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Promotion PendingUnstake (r:1 w:0)
	// Storage: Promotion NextInterestBlock (r:1 w:0)
	fn on_initialize() -> Weight {
		(2_705_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	// Storage: Promotion StartBlock (r:1 w:1)
	// Storage: Promotion NextInterestBlock (r:0 w:1)
	fn start_app_promotion() -> Weight {
		(1_436_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Promotion Admin (r:0 w:1)
	fn set_admin_address() -> Weight {
		(516_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainSystem ValidationData (r:1 w:0)
	// Storage: Promotion Staked (r:1 w:1)
	// Storage: Promotion TotalStaked (r:1 w:1)
	fn stake() -> Weight {
		(10_019_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainSystem ValidationData (r:1 w:0)
	// Storage: Promotion Staked (r:1 w:1)
	// Storage: Promotion TotalStaked (r:1 w:1)
	fn unstake() -> Weight {
		(10_619_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: System Account (r:2 w:0)
	// Storage: Promotion Staked (r:0 w:1)
	fn recalculate_stake() -> Weight {
		(4_932_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Promotion PendingUnstake (r:1 w:0)
	// Storage: Promotion NextInterestBlock (r:1 w:0)
	fn on_initialize() -> Weight {
		(2_705_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
	}
	// Storage: Promotion StartBlock (r:1 w:1)
	// Storage: Promotion NextInterestBlock (r:0 w:1)
	fn start_app_promotion() -> Weight {
		(1_436_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Promotion Admin (r:0 w:1)
	fn set_admin_address() -> Weight {
		(516_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainSystem ValidationData (r:1 w:0)
	// Storage: Promotion Staked (r:1 w:1)
	// Storage: Promotion TotalStaked (r:1 w:1)
	fn stake() -> Weight {
		(10_019_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainSystem ValidationData (r:1 w:0)
	// Storage: Promotion Staked (r:1 w:1)
	// Storage: Promotion TotalStaked (r:1 w:1)
	fn unstake() -> Weight {
		(10_619_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: System Account (r:2 w:0)
	// Storage: Promotion Staked (r:0 w:1)
	fn recalculate_stake() -> Weight {
		(4_932_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
