// Template adopted from https://github.com/paritytech/substrate/blob/master/.maintain/frame-weight-template.hbs

//! Autogenerated weights for pallet_foreign_assets
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-26, STEPS: `50`, REPEAT: `400`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bench-host`, CPU: `Intel(R) Core(TM) i7-8700 CPU @ 3.20GHz`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// target/production/unique-collator
// benchmark
// pallet
// --pallet
// pallet-foreign-assets
// --wasm-execution
// compiled
// --extrinsic
// *
// --template=.maintain/frame-weight-template.hbs
// --steps=50
// --repeat=400
// --heap-pages=4096
// --output=./pallets/foreign-assets/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_foreign_assets.
pub trait WeightInfo {
	fn register_foreign_asset() -> Weight;
	fn update_foreign_asset() -> Weight;
}

/// Weights for pallet_foreign_assets using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Common CreatedCollectionCount (r:1 w:1)
	/// Proof: Common CreatedCollectionCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Common DestroyedCollectionCount (r:1 w:0)
	/// Proof: Common DestroyedCollectionCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ForeignAssets NextForeignAssetId (r:1 w:1)
	/// Proof: ForeignAssets NextForeignAssetId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: ForeignAssets LocationToCurrencyIds (r:1 w:1)
	/// Proof: ForeignAssets LocationToCurrencyIds (max_values: None, max_size: Some(614), added: 3089, mode: MaxEncodedLen)
	/// Storage: ForeignAssets ForeignAssetLocations (r:1 w:1)
	/// Proof: ForeignAssets ForeignAssetLocations (max_values: None, max_size: Some(614), added: 3089, mode: MaxEncodedLen)
	/// Storage: ForeignAssets AssetMetadatas (r:1 w:1)
	/// Proof: ForeignAssets AssetMetadatas (max_values: None, max_size: Some(71), added: 2546, mode: MaxEncodedLen)
	/// Storage: ForeignAssets AssetBinding (r:1 w:1)
	/// Proof: ForeignAssets AssetBinding (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: Common AdminAmount (r:0 w:1)
	/// Proof: Common AdminAmount (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: Common CollectionPropertyPermissions (r:0 w:1)
	/// Proof: Common CollectionPropertyPermissions (max_values: None, max_size: Some(16726), added: 19201, mode: MaxEncodedLen)
	/// Storage: Common CollectionProperties (r:0 w:1)
	/// Proof: Common CollectionProperties (max_values: None, max_size: Some(40992), added: 43467, mode: MaxEncodedLen)
	/// Storage: Common CollectionById (r:0 w:1)
	/// Proof: Common CollectionById (max_values: None, max_size: Some(860), added: 3335, mode: MaxEncodedLen)
	fn register_foreign_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `286`
		//  Estimated: `6196`
		// Minimum execution time: 33_294_000 picoseconds.
		Weight::from_parts(34_011_000, 6196)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(12_u64))
	}
	/// Storage: ForeignAssets ForeignAssetLocations (r:1 w:1)
	/// Proof: ForeignAssets ForeignAssetLocations (max_values: None, max_size: Some(614), added: 3089, mode: MaxEncodedLen)
	/// Storage: ForeignAssets AssetMetadatas (r:1 w:1)
	/// Proof: ForeignAssets AssetMetadatas (max_values: None, max_size: Some(71), added: 2546, mode: MaxEncodedLen)
	fn update_foreign_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `197`
		//  Estimated: `4079`
		// Minimum execution time: 9_296_000 picoseconds.
		Weight::from_parts(9_594_000, 4079)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Common CreatedCollectionCount (r:1 w:1)
	/// Proof: Common CreatedCollectionCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Common DestroyedCollectionCount (r:1 w:0)
	/// Proof: Common DestroyedCollectionCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ForeignAssets NextForeignAssetId (r:1 w:1)
	/// Proof: ForeignAssets NextForeignAssetId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: ForeignAssets LocationToCurrencyIds (r:1 w:1)
	/// Proof: ForeignAssets LocationToCurrencyIds (max_values: None, max_size: Some(614), added: 3089, mode: MaxEncodedLen)
	/// Storage: ForeignAssets ForeignAssetLocations (r:1 w:1)
	/// Proof: ForeignAssets ForeignAssetLocations (max_values: None, max_size: Some(614), added: 3089, mode: MaxEncodedLen)
	/// Storage: ForeignAssets AssetMetadatas (r:1 w:1)
	/// Proof: ForeignAssets AssetMetadatas (max_values: None, max_size: Some(71), added: 2546, mode: MaxEncodedLen)
	/// Storage: ForeignAssets AssetBinding (r:1 w:1)
	/// Proof: ForeignAssets AssetBinding (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: Common AdminAmount (r:0 w:1)
	/// Proof: Common AdminAmount (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: Common CollectionPropertyPermissions (r:0 w:1)
	/// Proof: Common CollectionPropertyPermissions (max_values: None, max_size: Some(16726), added: 19201, mode: MaxEncodedLen)
	/// Storage: Common CollectionProperties (r:0 w:1)
	/// Proof: Common CollectionProperties (max_values: None, max_size: Some(40992), added: 43467, mode: MaxEncodedLen)
	/// Storage: Common CollectionById (r:0 w:1)
	/// Proof: Common CollectionById (max_values: None, max_size: Some(860), added: 3335, mode: MaxEncodedLen)
	fn register_foreign_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `286`
		//  Estimated: `6196`
		// Minimum execution time: 33_294_000 picoseconds.
		Weight::from_parts(34_011_000, 6196)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(12_u64))
	}
	/// Storage: ForeignAssets ForeignAssetLocations (r:1 w:1)
	/// Proof: ForeignAssets ForeignAssetLocations (max_values: None, max_size: Some(614), added: 3089, mode: MaxEncodedLen)
	/// Storage: ForeignAssets AssetMetadatas (r:1 w:1)
	/// Proof: ForeignAssets AssetMetadatas (max_values: None, max_size: Some(71), added: 2546, mode: MaxEncodedLen)
	fn update_foreign_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `197`
		//  Estimated: `4079`
		// Minimum execution time: 9_296_000 picoseconds.
		Weight::from_parts(9_594_000, 4079)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}

