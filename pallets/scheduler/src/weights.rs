// Template adopted from https://github.com/paritytech/substrate/blob/master/.maintain/frame-weight-template.hbs

//! Autogenerated weights for pallet_unq_scheduler
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-09, STEPS: `50`, REPEAT: 200, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// target/release/unique-collator
// benchmark
// pallet
// --pallet
// pallet-unq-scheduler
// --wasm-execution
// compiled
// --extrinsic
// *
// --template
// .maintain/frame-weight-template.hbs
// --steps=50
// --repeat=200
// --heap-pages=4096
// --output=./pallets/scheduler/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_unq_scheduler.
pub trait WeightInfo {
	fn on_initialize_periodic_named_resolved(s: u32, ) -> Weight;
	fn on_initialize_named_resolved(s: u32, ) -> Weight;
	fn on_initialize_periodic_resolved(s: u32, ) -> Weight;
	fn on_initialize_resolved(s: u32, ) -> Weight;
	fn schedule_named(s: u32, ) -> Weight;
	fn cancel_named(s: u32, ) -> Weight;
}

/// Weights for pallet_unq_scheduler using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: System AllExtrinsicsLen (r:1 w:1)
	// Storage: System BlockWeight (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	fn on_initialize_periodic_named_resolved(s: u32, ) -> Weight {
		(39_822_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((31_823_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: System AllExtrinsicsLen (r:1 w:1)
	// Storage: System BlockWeight (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	fn on_initialize_named_resolved(s: u32, ) -> Weight {
		(36_019_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((22_660_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: System AllExtrinsicsLen (r:1 w:1)
	// Storage: System BlockWeight (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	fn on_initialize_periodic_resolved(s: u32, ) -> Weight {
		(36_613_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((31_895_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: System AllExtrinsicsLen (r:1 w:1)
	// Storage: System BlockWeight (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	fn on_initialize_resolved(s: u32, ) -> Weight {
		(36_252_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((22_662_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn schedule_named(s: u32, ) -> Weight {
		(34_561_000 as Weight)
			// Standard Error: 0
			.saturating_add((161_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn cancel_named(s: u32, ) -> Weight {
		(30_932_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((1_537_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: System AllExtrinsicsLen (r:1 w:1)
	// Storage: System BlockWeight (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	fn on_initialize_periodic_named_resolved(s: u32, ) -> Weight {
		(39_822_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((31_823_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes((2 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: System AllExtrinsicsLen (r:1 w:1)
	// Storage: System BlockWeight (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	fn on_initialize_named_resolved(s: u32, ) -> Weight {
		(36_019_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((22_660_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: System AllExtrinsicsLen (r:1 w:1)
	// Storage: System BlockWeight (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	fn on_initialize_periodic_resolved(s: u32, ) -> Weight {
		(36_613_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((31_895_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes((2 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: System AllExtrinsicsLen (r:1 w:1)
	// Storage: System BlockWeight (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	fn on_initialize_resolved(s: u32, ) -> Weight {
		(36_252_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((22_662_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn schedule_named(s: u32, ) -> Weight {
		(34_561_000 as Weight)
			// Standard Error: 0
			.saturating_add((161_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn cancel_named(s: u32, ) -> Weight {
		(30_932_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((1_537_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
}
