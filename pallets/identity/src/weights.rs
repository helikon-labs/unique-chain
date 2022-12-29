// Copyright 2019-2022 Unique Network (Gibraltar) Ltd.
// This file is part of Unique Network.

// Unique Network is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Unique Network is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Unique Network. If not, see <http://www.gnu.org/licenses/>.

// Original license:
// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_identity
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-07, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm2`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_identity
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/identity/src/weights.rs
// --header=./HEADER-APACHE2
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_identity.
pub trait WeightInfo {
	fn add_registrar(r: u32, ) -> Weight;
	fn set_identity(r: u32, x: u32, ) -> Weight;
	fn set_subs_new(s: u32, ) -> Weight;
	fn set_subs_old(p: u32, ) -> Weight;
	fn clear_identity(r: u32, s: u32, x: u32, ) -> Weight;
	fn request_judgement(r: u32, x: u32, ) -> Weight;
	fn cancel_request(r: u32, x: u32, ) -> Weight;
	fn set_fee(r: u32, ) -> Weight;
	fn set_account_id(r: u32, ) -> Weight;
	fn set_fields(r: u32, ) -> Weight;
	fn provide_judgement(r: u32, x: u32, ) -> Weight;
	fn kill_identity(r: u32, s: u32, x: u32, ) -> Weight;
	fn force_insert_identities(x: u32, n: u32, ) -> Weight;
	fn force_remove_identities(x: u32, n: u32, ) -> Weight;
	fn add_sub(s: u32, ) -> Weight;
	fn rename_sub(s: u32, ) -> Weight;
	fn remove_sub(s: u32, ) -> Weight;
	fn quit_sub(s: u32, ) -> Weight;
}

/// Weights for pallet_identity using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn add_registrar(r: u32, ) -> Weight {
		// Minimum execution time: 20_269 nanoseconds.
		Weight::from_ref_time(21_910_543 as u64)
			// Standard Error: 4_604
			.saturating_add(Weight::from_ref_time(223_104 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn set_identity(r: u32, x: u32, ) -> Weight {
		// Minimum execution time: 41_872 nanoseconds.
		Weight::from_ref_time(40_230_216 as u64)
			// Standard Error: 2_342
			.saturating_add(Weight::from_ref_time(145_168 as u64).saturating_mul(r as u64))
			// Standard Error: 457
			.saturating_add(Weight::from_ref_time(291_732 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:2 w:2)
	/// The range of component `s` is `[0, 100]`.
	fn set_subs_new(s: u32, ) -> Weight {
		// Minimum execution time: 12_024 nanoseconds.
		Weight::from_ref_time(32_550_819 as u64)
			// Standard Error: 5_057
			.saturating_add(Weight::from_ref_time(2_521_245 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:2)
	/// The range of component `p` is `[0, 100]`.
	fn set_subs_old(p: u32, ) -> Weight {
		// Minimum execution time: 12_232 nanoseconds.
		Weight::from_ref_time(34_009_761 as u64)
			// Standard Error: 5_047
			.saturating_add(Weight::from_ref_time(1_113_100 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(p as u64)))
	}
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity IdentityOf (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:100)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	/// The range of component `x` is `[0, 100]`.
	fn clear_identity(r: u32, s: u32, x: u32, ) -> Weight {
		// Minimum execution time: 57_144 nanoseconds.
		Weight::from_ref_time(41_559_247 as u64)
			// Standard Error: 9_996
			.saturating_add(Weight::from_ref_time(146_770 as u64).saturating_mul(r as u64))
			// Standard Error: 1_952
			.saturating_add(Weight::from_ref_time(1_086_673 as u64).saturating_mul(s as u64))
			// Standard Error: 1_952
			.saturating_add(Weight::from_ref_time(162_481 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	// Storage: Identity Registrars (r:1 w:0)
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn request_judgement(r: u32, x: u32, ) -> Weight {
		// Minimum execution time: 44_726 nanoseconds.
		Weight::from_ref_time(41_637_308 as u64)
			// Standard Error: 1_907
			.saturating_add(Weight::from_ref_time(219_078 as u64).saturating_mul(r as u64))
			// Standard Error: 372
			.saturating_add(Weight::from_ref_time(309_888 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn cancel_request(r: u32, x: u32, ) -> Weight {
		// Minimum execution time: 39_719 nanoseconds.
		Weight::from_ref_time(38_008_751 as u64)
			// Standard Error: 2_394
			.saturating_add(Weight::from_ref_time(181_870 as u64).saturating_mul(r as u64))
			// Standard Error: 467
			.saturating_add(Weight::from_ref_time(314_990 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn set_fee(r: u32, ) -> Weight {
		// Minimum execution time: 10_634 nanoseconds.
		Weight::from_ref_time(11_383_704 as u64)
			// Standard Error: 2_250
			.saturating_add(Weight::from_ref_time(193_094 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn set_account_id(r: u32, ) -> Weight {
		// Minimum execution time: 10_840 nanoseconds.
		Weight::from_ref_time(11_638_740 as u64)
			// Standard Error: 1_985
			.saturating_add(Weight::from_ref_time(193_016 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn set_fields(r: u32, ) -> Weight {
		// Minimum execution time: 10_748 nanoseconds.
		Weight::from_ref_time(11_346_901 as u64)
			// Standard Error: 2_132
			.saturating_add(Weight::from_ref_time(196_630 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity Registrars (r:1 w:0)
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	/// The range of component `x` is `[0, 100]`.
	fn provide_judgement(r: u32, x: u32, ) -> Weight {
		// Minimum execution time: 33_682 nanoseconds.
		Weight::from_ref_time(31_336_603 as u64)
			// Standard Error: 3_056
			.saturating_add(Weight::from_ref_time(200_403 as u64).saturating_mul(r as u64))
			// Standard Error: 565
			.saturating_add(Weight::from_ref_time(525_142 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity IdentityOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:100)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	/// The range of component `x` is `[0, 100]`.
	fn kill_identity(r: u32, s: u32, x: u32, ) -> Weight {
		// Minimum execution time: 68_794 nanoseconds.
		Weight::from_ref_time(52_114_486 as u64)
			// Standard Error: 4_808
			.saturating_add(Weight::from_ref_time(153_462 as u64).saturating_mul(r as u64))
			// Standard Error: 939
			.saturating_add(Weight::from_ref_time(1_084_612 as u64).saturating_mul(s as u64))
			// Standard Error: 939
			.saturating_add(Weight::from_ref_time(170_112 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `x` is `[0, 100]`.
	/// The range of component `n` is `[0, 600]`.
	fn force_insert_identities(x: u32, n: u32) -> Weight {
		// Minimum execution time: 41_872 nanoseconds.
		Weight::from_ref_time(40_230_216 as u64)
			// Standard Error: 2_342
			.saturating_add(Weight::from_ref_time(145_168 as u64))
			// Standard Error: 457
			.saturating_add(Weight::from_ref_time(291_732 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64).saturating_mul(n as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `x` is `[0, 100]`.
	/// The range of component `n` is `[0, 600]`.
	fn force_remove_identities(x: u32, n: u32) -> Weight {
		// Minimum execution time: 41_872 nanoseconds.
		Weight::from_ref_time(40_230_216 as u64)
			// Standard Error: 2_342
			.saturating_add(Weight::from_ref_time(145_168 as u64))
			// Standard Error: 457
			.saturating_add(Weight::from_ref_time(291_732 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64).saturating_mul(n as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[0, 99]`.
	fn add_sub(s: u32, ) -> Weight {
		// Minimum execution time: 37_914 nanoseconds.
		Weight::from_ref_time(43_488_083 as u64)
			// Standard Error: 1_631
			.saturating_add(Weight::from_ref_time(118_845 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	/// The range of component `s` is `[1, 100]`.
	fn rename_sub(s: u32, ) -> Weight {
		// Minimum execution time: 16_124 nanoseconds.
		Weight::from_ref_time(18_580_462 as u64)
			// Standard Error: 688
			.saturating_add(Weight::from_ref_time(67_220 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[1, 100]`.
	fn remove_sub(s: u32, ) -> Weight {
		// Minimum execution time: 41_517 nanoseconds.
		Weight::from_ref_time(45_123_530 as u64)
			// Standard Error: 1_530
			.saturating_add(Weight::from_ref_time(105_429 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[0, 99]`.
	fn quit_sub(s: u32, ) -> Weight {
		// Minimum execution time: 30_171 nanoseconds.
		Weight::from_ref_time(33_355_514 as u64)
			// Standard Error: 1_286
			.saturating_add(Weight::from_ref_time(114_716 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn add_registrar(r: u32, ) -> Weight {
		// Minimum execution time: 20_269 nanoseconds.
		Weight::from_ref_time(21_910_543 as u64)
			// Standard Error: 4_604
			.saturating_add(Weight::from_ref_time(223_104 as u64).saturating_mul(r as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn set_identity(r: u32, x: u32, ) -> Weight {
		// Minimum execution time: 41_872 nanoseconds.
		Weight::from_ref_time(40_230_216 as u64)
			// Standard Error: 2_342
			.saturating_add(Weight::from_ref_time(145_168 as u64).saturating_mul(r as u64))
			// Standard Error: 457
			.saturating_add(Weight::from_ref_time(291_732 as u64).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:2 w:2)
	/// The range of component `s` is `[0, 100]`.
	fn set_subs_new(s: u32, ) -> Weight {
		// Minimum execution time: 12_024 nanoseconds.
		Weight::from_ref_time(32_550_819 as u64)
			// Standard Error: 5_057
			.saturating_add(Weight::from_ref_time(2_521_245 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(s as u64)))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:2)
	/// The range of component `p` is `[0, 100]`.
	fn set_subs_old(p: u32, ) -> Weight {
		// Minimum execution time: 12_232 nanoseconds.
		Weight::from_ref_time(34_009_761 as u64)
			// Standard Error: 5_047
			.saturating_add(Weight::from_ref_time(1_113_100 as u64).saturating_mul(p as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(p as u64)))
	}
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity IdentityOf (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:100)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	/// The range of component `x` is `[0, 100]`.
	fn clear_identity(r: u32, s: u32, x: u32, ) -> Weight {
		// Minimum execution time: 57_144 nanoseconds.
		Weight::from_ref_time(41_559_247 as u64)
			// Standard Error: 9_996
			.saturating_add(Weight::from_ref_time(146_770 as u64).saturating_mul(r as u64))
			// Standard Error: 1_952
			.saturating_add(Weight::from_ref_time(1_086_673 as u64).saturating_mul(s as u64))
			// Standard Error: 1_952
			.saturating_add(Weight::from_ref_time(162_481 as u64).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	// Storage: Identity Registrars (r:1 w:0)
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn request_judgement(r: u32, x: u32, ) -> Weight {
		// Minimum execution time: 44_726 nanoseconds.
		Weight::from_ref_time(41_637_308 as u64)
			// Standard Error: 1_907
			.saturating_add(Weight::from_ref_time(219_078 as u64).saturating_mul(r as u64))
			// Standard Error: 372
			.saturating_add(Weight::from_ref_time(309_888 as u64).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn cancel_request(r: u32, x: u32, ) -> Weight {
		// Minimum execution time: 39_719 nanoseconds.
		Weight::from_ref_time(38_008_751 as u64)
			// Standard Error: 2_394
			.saturating_add(Weight::from_ref_time(181_870 as u64).saturating_mul(r as u64))
			// Standard Error: 467
			.saturating_add(Weight::from_ref_time(314_990 as u64).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn set_fee(r: u32, ) -> Weight {
		// Minimum execution time: 10_634 nanoseconds.
		Weight::from_ref_time(11_383_704 as u64)
			// Standard Error: 2_250
			.saturating_add(Weight::from_ref_time(193_094 as u64).saturating_mul(r as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn set_account_id(r: u32, ) -> Weight {
		// Minimum execution time: 10_840 nanoseconds.
		Weight::from_ref_time(11_638_740 as u64)
			// Standard Error: 1_985
			.saturating_add(Weight::from_ref_time(193_016 as u64).saturating_mul(r as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn set_fields(r: u32, ) -> Weight {
		// Minimum execution time: 10_748 nanoseconds.
		Weight::from_ref_time(11_346_901 as u64)
			// Standard Error: 2_132
			.saturating_add(Weight::from_ref_time(196_630 as u64).saturating_mul(r as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Identity Registrars (r:1 w:0)
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	/// The range of component `x` is `[0, 100]`.
	fn provide_judgement(r: u32, x: u32, ) -> Weight {
		// Minimum execution time: 33_682 nanoseconds.
		Weight::from_ref_time(31_336_603 as u64)
			// Standard Error: 3_056
			.saturating_add(Weight::from_ref_time(200_403 as u64).saturating_mul(r as u64))
			// Standard Error: 565
			.saturating_add(Weight::from_ref_time(525_142 as u64).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity IdentityOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:100)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	/// The range of component `x` is `[0, 100]`.
	fn kill_identity(r: u32, s: u32, x: u32, ) -> Weight {
		// Minimum execution time: 68_794 nanoseconds.
		Weight::from_ref_time(52_114_486 as u64)
			// Standard Error: 4_808
			.saturating_add(Weight::from_ref_time(153_462 as u64).saturating_mul(r as u64))
			// Standard Error: 939
			.saturating_add(Weight::from_ref_time(1_084_612 as u64).saturating_mul(s as u64))
			// Standard Error: 939
			.saturating_add(Weight::from_ref_time(170_112 as u64).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `x` is `[0, 100]`.
	/// The range of component `n` is `[0, 600]`.
	fn force_insert_identities(x: u32, n: u32) -> Weight {
		// Minimum execution time: 41_872 nanoseconds.
		Weight::from_ref_time(40_230_216 as u64)
			// Standard Error: 2_342
			.saturating_add(Weight::from_ref_time(145_168 as u64))
			// Standard Error: 457
			.saturating_add(Weight::from_ref_time(291_732 as u64).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64).saturating_mul(n as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `x` is `[0, 100]`.
	/// The range of component `n` is `[0, 600]`.
	fn force_remove_identities(x: u32, n: u32) -> Weight {
		// Minimum execution time: 41_872 nanoseconds.
		Weight::from_ref_time(40_230_216 as u64)
			// Standard Error: 2_342
			.saturating_add(Weight::from_ref_time(145_168 as u64))
			// Standard Error: 457
			.saturating_add(Weight::from_ref_time(291_732 as u64).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64).saturating_mul(n as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[0, 99]`.
	fn add_sub(s: u32, ) -> Weight {
		// Minimum execution time: 37_914 nanoseconds.
		Weight::from_ref_time(43_488_083 as u64)
			// Standard Error: 1_631
			.saturating_add(Weight::from_ref_time(118_845 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	/// The range of component `s` is `[1, 100]`.
	fn rename_sub(s: u32, ) -> Weight {
		// Minimum execution time: 16_124 nanoseconds.
		Weight::from_ref_time(18_580_462 as u64)
			// Standard Error: 688
			.saturating_add(Weight::from_ref_time(67_220 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[1, 100]`.
	fn remove_sub(s: u32, ) -> Weight {
		// Minimum execution time: 41_517 nanoseconds.
		Weight::from_ref_time(45_123_530 as u64)
			// Standard Error: 1_530
			.saturating_add(Weight::from_ref_time(105_429 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[0, 99]`.
	fn quit_sub(s: u32, ) -> Weight {
		// Minimum execution time: 30_171 nanoseconds.
		Weight::from_ref_time(33_355_514 as u64)
			// Standard Error: 1_286
			.saturating_add(Weight::from_ref_time(114_716 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
}
