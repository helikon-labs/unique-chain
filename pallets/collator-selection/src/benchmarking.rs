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
// Copyright (C) 2021 Parity Technologies (UK) Ltd.
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

//! Benchmarking setup for pallet-collator-selection

use super::*;

#[allow(unused)]
use crate::Pallet as CollatorSelection;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::{
	assert_ok,
	codec::Decode,
	traits::{Currency, EnsureOrigin, Get},
};
use frame_system::{EventRecord, RawOrigin};
use pallet_authorship::EventHandler;
use pallet_session::{self as session, SessionManager};
use pallet_configuration::{
	self as configuration, BalanceOf,
	CollatorSelectionDesiredCollatorsOverride as DesiredCollators,
	CollatorSelectionLicenseBondOverride as LicenseBond,
};
use sp_std::prelude::*;

const SEED: u32 = 0;

// TODO: remove if this is given in substrate commit.
macro_rules! whitelist {
	($acc:ident) => {
		frame_benchmarking::benchmarking::add_to_whitelist(
			frame_system::Account::<T>::hashed_key_for(&$acc).into(),
		);
	};
}

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	let events = frame_system::Pallet::<T>::events();
	let system_event: <T as frame_system::Config>::RuntimeEvent = generic_event.into();
	// compare to the last event record
	let EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}

fn create_funded_user<T: Config>(
	string: &'static str,
	n: u32,
	balance_factor: u32,
) -> T::AccountId {
	let user = account(string, n, SEED);
	let balance = T::Currency::minimum_balance() * balance_factor.into();
	let _ = T::Currency::make_free_balance_be(&user, balance);
	user
}

fn keys<T: Config + session::Config>(c: u32) -> <T as session::Config>::Keys {
	use rand::{RngCore, SeedableRng};

	let keys = {
		let mut keys = [0u8; 128];

		if c > 0 {
			let mut rng = rand::rngs::StdRng::seed_from_u64(c as u64);
			rng.fill_bytes(&mut keys);
		}

		keys
	};

	Decode::decode(&mut &keys[..]).unwrap()
}

fn validator<T: Config + session::Config>(c: u32) -> (T::AccountId, <T as session::Config>::Keys) {
	(create_funded_user::<T>("candidate", c, 1000), keys::<T>(c))
}

fn register_validators<T: Config + session::Config>(count: u32) -> Vec<T::AccountId> {
	let validators = (0..count).map(|c| validator::<T>(c)).collect::<Vec<_>>();

	for (who, keys) in validators.clone() {
		<session::Pallet<T>>::set_keys(RawOrigin::Signed(who).into(), keys, Vec::new()).unwrap();
	}

	validators.into_iter().map(|(who, _)| who).collect()
}

fn register_invulnerables<T: Config + configuration::Config>(count: u32) {
	let candidates = (0..count)
		.map(|c| account("candidate", c, SEED))
		.collect::<Vec<_>>();

	for who in candidates {
		<CollatorSelection<T>>::add_invulnerable(T::UpdateOrigin::successful_origin(), who)
			.unwrap();
	}
}

fn register_candidates<T: Config + configuration::Config>(count: u32) {
	let candidates = (0..count)
		.map(|c| account("candidate", c, SEED))
		.collect::<Vec<_>>();
	/*assert!(
		<LicenseBond<T>>::get() > 0u32.into(),
		"Bond cannot be zero!"
	);*/

	for who in candidates {
		T::Currency::make_free_balance_be(&who, <LicenseBond<T>>::get() * 2u32.into());
		<CollatorSelection<T>>::get_license(RawOrigin::Signed(who.clone()).into()).unwrap();
		<CollatorSelection<T>>::onboard(RawOrigin::Signed(who).into()).unwrap();
	}
}

fn get_licenses<T: Config + configuration::Config>(count: u32) {
	let candidates = (0..count)
		.map(|c| account("candidate", c, SEED))
		.collect::<Vec<_>>();
	/*assert!(
		<LicenseBond<T>>::get() > 0u32.into(),
		"Bond cannot be zero!"
	);*/

	for who in candidates {
		T::Currency::make_free_balance_be(&who, <LicenseBond<T>>::get() * 2u32.into());
		<CollatorSelection<T>>::get_license(RawOrigin::Signed(who.clone()).into()).unwrap();
	}
}

benchmarks! {
	where_clause { where T: pallet_authorship::Config + session::Config + configuration::Config }

	add_invulnerable {
		let b in 1 .. T::MaxCollators::get() - 3;
		register_validators::<T>(b);
		register_invulnerables::<T>(b);

		// log::info!("{} {}", <Invulnerables<T>>::get().len(), b);

		let new_invulnerable: T::AccountId = whitelisted_caller();
		let bond: BalanceOf<T> = T::Currency::minimum_balance() * 2u32.into();
		T::Currency::make_free_balance_be(&new_invulnerable, bond.clone());

		<session::Pallet<T>>::set_keys(
			RawOrigin::Signed(new_invulnerable.clone()).into(),
			keys::<T>(b + 1),
			Vec::new()
		).unwrap();

		let root_origin = T::UpdateOrigin::successful_origin();
	}: {
		assert_ok!(
			<CollatorSelection<T>>::add_invulnerable(root_origin, new_invulnerable.clone())
		);
	}
	verify {
		assert_last_event::<T>(Event::InvulnerableAdded{invulnerable: new_invulnerable}.into());
	}

	remove_invulnerable {
		let b in 1 .. T::MaxCollators::get();
		register_validators::<T>(b);
		register_invulnerables::<T>(b);

		let root_origin = T::UpdateOrigin::successful_origin();
		let leaving = <Invulnerables<T>>::get().last().unwrap().clone();
		whitelist!(leaving);
	}: {
		assert_ok!(
			<CollatorSelection<T>>::remove_invulnerable(root_origin, leaving.clone())
		);
	}
	verify {
		assert_last_event::<T>(Event::InvulnerableRemoved{invulnerable: leaving}.into());
	}

	get_license {
		let c in 1 .. T::MaxCollators::get();

		<LicenseBond<T>>::put(T::Currency::minimum_balance());

		register_validators::<T>(c);
		get_licenses::<T>(c);

		let caller: T::AccountId = whitelisted_caller();
		let bond: BalanceOf<T> = T::Currency::minimum_balance() * 2u32.into();
		T::Currency::make_free_balance_be(&caller, bond.clone());

		<session::Pallet<T>>::set_keys(
			RawOrigin::Signed(caller.clone()).into(),
			keys::<T>(c + 1),
			Vec::new()
		).unwrap();

	}: _(RawOrigin::Signed(caller.clone()))
	verify {
		assert_last_event::<T>(Event::LicenseObtained{account_id: caller, deposit: bond / 2u32.into()}.into());
	}

	// worst case is when we have all the max-candidate slots filled except one, and we fill that
	// one.
	onboard {
		let c in 1 .. 5;

		<LicenseBond<T>>::put(T::Currency::minimum_balance());
		<DesiredCollators<T>>::put(c + 2);

		register_validators::<T>(c);
		register_candidates::<T>(c);

		let caller: T::AccountId = whitelisted_caller();
		let bond: BalanceOf<T> = T::Currency::minimum_balance() * 2u32.into();
		T::Currency::make_free_balance_be(&caller, bond.clone());

		let origin = RawOrigin::Signed(caller.clone());

		<session::Pallet<T>>::set_keys(
			origin.clone().into(),
			keys::<T>(c + 1),
			Vec::new()
		).unwrap();

		assert_ok!(
			<CollatorSelection<T>>::get_license(origin.clone().into())
		);
	}: _(origin)
	verify {
		assert_last_event::<T>(Event::CandidateAdded{account_id: caller}.into());
	}

	// worst case is the last candidate leaving.
	offboard {
		let c in 1 .. T::MaxCollators::get();
		<LicenseBond<T>>::put(T::Currency::minimum_balance());
		<DesiredCollators<T>>::put(c + 2);

		register_validators::<T>(c);
		register_candidates::<T>(c);

		let leaving = <Candidates<T>>::get().last().unwrap().clone();
		whitelist!(leaving);
	}: _(RawOrigin::Signed(leaving.clone()))
	verify {
		assert_last_event::<T>(Event::CandidateRemoved{account_id: leaving}.into());
	}

	// worst case is the last candidate leaving.
	release_license {
		let c in 1 .. T::MaxCollators::get();
		let bond = T::Currency::minimum_balance();
		<LicenseBond<T>>::put(bond);
		<DesiredCollators<T>>::put(c);

		register_validators::<T>(c);
		register_candidates::<T>(c);

		let leaving = <Candidates<T>>::get().last().unwrap().clone();
		whitelist!(leaving);
	}: _(RawOrigin::Signed(leaving.clone()))
	verify {
		assert_last_event::<T>(Event::LicenseReleased{account_id: leaving, deposit_returned: bond}.into());
	}

	// worst case is the last candidate leaving.
	force_release_license {
		let c in 1 .. T::MaxCollators::get();
		let bond = T::Currency::minimum_balance();
		<LicenseBond<T>>::put(bond);
		<DesiredCollators<T>>::put(c);

		register_validators::<T>(c);
		register_candidates::<T>(c);

		let leaving = <Candidates<T>>::get().last().unwrap().clone();
		whitelist!(leaving);
		let origin = T::UpdateOrigin::successful_origin();
	}: {
		assert_ok!(
			<CollatorSelection<T>>::force_release_license(origin, leaving.clone())
		);
	}
	verify {
		assert_last_event::<T>(Event::LicenseReleased{account_id: leaving, deposit_returned: bond}.into());
	}

	// worst case is paying a non-existing candidate account.
	note_author {
		<LicenseBond<T>>::put(T::Currency::minimum_balance());
		T::Currency::make_free_balance_be(
			&<CollatorSelection<T>>::account_id(),
			T::Currency::minimum_balance() * 4u32.into(),
		);
		let author = account("author", 0, SEED);
		let new_block: T::BlockNumber = 10u32.into();

		frame_system::Pallet::<T>::set_block_number(new_block);
		assert!(T::Currency::free_balance(&author) == 0u32.into());
	}: {
		<CollatorSelection<T> as EventHandler<_, _>>::note_author(author.clone())
	} verify {
		assert!(T::Currency::free_balance(&author) > 0u32.into());
		assert_eq!(frame_system::Pallet::<T>::block_number(), new_block);
	}

	// worst case for new session.
	new_session {
		let r in 1 .. T::MaxCollators::get();
		let c in 1 .. T::MaxCollators::get();

		<LicenseBond<T>>::put(T::Currency::minimum_balance());
		<DesiredCollators<T>>::put(c);
		frame_system::Pallet::<T>::set_block_number(0u32.into());

		register_validators::<T>(c);
		register_candidates::<T>(c);

		let new_block: T::BlockNumber = 1800u32.into();
		let zero_block: T::BlockNumber = 0u32.into();
		let candidates = <Candidates<T>>::get();

		let non_removals = c.saturating_sub(r);

		for i in 0..c {
			<LastAuthoredBlock<T>>::insert(candidates[i as usize].clone(), zero_block);
		}

		if non_removals > 0 {
			for i in 0..non_removals {
				<LastAuthoredBlock<T>>::insert(candidates[i as usize].clone(), new_block);
			}
		} else {
			for i in 0..c {
				<LastAuthoredBlock<T>>::insert(candidates[i as usize].clone(), new_block);
			}
		}

		let pre_length = <Candidates<T>>::get().len();

		frame_system::Pallet::<T>::set_block_number(new_block);

		assert!(<Candidates<T>>::get().len() == c as usize);
	}: {
		<CollatorSelection<T> as SessionManager<_>>::new_session(0)
	} verify {
		if c > r {
			assert!(<Candidates<T>>::get().len() < pre_length);
		} else {
			assert!(<Candidates<T>>::get().len() == pre_length);
		}
	}
}

impl_benchmark_test_suite!(
	CollatorSelection,
	crate::mock::new_test_ext(),
	crate::mock::Test,
);
