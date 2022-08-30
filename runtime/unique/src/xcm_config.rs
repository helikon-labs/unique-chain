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

use cumulus_pallet_xcm;
use frame_support::{
    {match_types, parameter_types, weights::Weight},
    pallet_prelude::Get,
    traits::{Contains, Everything, fungibles},
};
use frame_system::EnsureRoot;
use orml_traits::{location::AbsoluteReserveProvider, parameter_type_with_key};
use pallet_xcm::XcmPassthrough;
use polkadot_parachain::primitives::Sibling;
use sp_runtime::traits::{AccountIdConversion, CheckedConversion, Convert, Zero};
use sp_std::{borrow::Borrow, marker::PhantomData, vec, vec::Vec};
use xcm::{
    latest::{MultiAsset, Xcm},
    prelude::{Concrete, Fungible as XcmFungible},
    v1::{BodyId, Junction::*, Junctions::*, MultiLocation, NetworkId},
};
use xcm_builder::{
    AllowKnownQueryResponses, AllowSubscriptionsFrom,
    AccountId32Aliases, AllowTopLevelPaidExecutionFrom, AllowUnpaidExecutionFrom,
    EnsureXcmOrigin, FixedWeightBounds, FungiblesAdapter, LocationInverter, ParentAsSuperuser,
    ParentIsPreset, RelayChainAsNative, SiblingParachainAsNative, SiblingParachainConvertsVia,
    SignedAccountId32AsNative, SignedToAccountId32, SovereignSignedViaLocation, TakeWeightCredit,
    ConvertedConcreteAssetId,
};
use xcm_executor::{
    {Config, XcmExecutor},
    traits::{Convert as ConvertXcm, FilterAssetLocation, JustTry, MatchesFungible},
};

use up_common::{
    constants::{MAXIMUM_BLOCK_WEIGHT, UNIQUE},
    types::{AccountId, Balance},
};
use pallet_foreing_assets::{
    AssetIds, AssetIdMapping, XcmForeignAssetIdMapping, CurrencyId, NativeCurrency,
    FreeForAll, TryAsForeing, ForeignAssetId,
};
use crate::{
    Balances, Call, DmpQueue, Event, Origin, ParachainInfo,
    ParachainSystem, PolkadotXcm, Runtime, XcmpQueue,
};
use crate::runtime_common::config::substrate::{TreasuryModuleId, MaxLocks, MaxReserves};
use crate::runtime_common::config::pallets::TreasuryAccountId;
use crate::runtime_common::config::xcm::*;
use crate::*;
use xcm::opaque::latest::prelude::{ DepositReserveAsset, DepositAsset, TransferAsset, TransferReserveAsset };

// Signed version of balance
pub type Amount = i128;


pub type Barrier = DenyThenTry<
    (
        DenyTransact,
        DenyExchangeWithUnknownLocation,
    ),
    (
        TakeWeightCredit,
        AllowTopLevelPaidExecutionFrom<Everything>,
        // Parent and its exec plurality get free execution
        AllowUnpaidExecutionFrom<ParentOrParentsExecutivePlurality>,
        // Expected responses are OK.
        AllowKnownQueryResponses<PolkadotXcm>,
        // Subscriptions for version tracking are OK.
        AllowSubscriptionsFrom<ParentOrSiblings>,
    ),
>;

pub fn get_allowed_locations() -> Vec<MultiLocation> {
    vec![
        // Self location
        MultiLocation { parents: 0, interior: Here },
        // Parent location
        MultiLocation { parents: 1, interior: Here },
        // Karura/Acala location
        MultiLocation { parents: 1, interior: X1(Parachain(2000)) },
        // Moonbeam location
        MultiLocation { parents: 1, interior: X1(Parachain(2004)) },
        // Self parachain address
        MultiLocation { parents: 1, interior: X1(Parachain(ParachainInfo::get().into())) },
    ]
}

// Allow xcm exchange only with locations in list
pub struct DenyExchangeWithUnknownLocation;
impl TryPass for DenyExchangeWithUnknownLocation {
    fn try_pass<Call>(
        origin: &MultiLocation,
        message: &mut Xcm<Call>,
    ) -> Result<(), ()> {

        // Check if deposit or transfer belongs to allowed parachains
        let mut allowed = get_allowed_locations().contains(origin);

        message.0.iter().for_each(|inst| {
            match inst {
                DepositReserveAsset { dest: dst, .. } => { allowed |= get_allowed_locations().contains(dst); }
                TransferReserveAsset { dest: dst, .. } => { allowed |= get_allowed_locations().contains(dst); }
                _ => {}
            }
        });

        if allowed {
            return Ok(());
        }

        log::warn!(
			target: "xcm::barrier",
			"Unexpected deposit or transfer location"
		);
        // Deny
        Err(())
    }
}


match_types! {
	pub type ParentOrParentsExecutivePlurality: impl Contains<MultiLocation> = {
		MultiLocation { parents: 1, interior: Here } |
		MultiLocation { parents: 1, interior: X1(Plurality { id: BodyId::Executive, .. }) }
	};
	pub type ParentOrSiblings: impl Contains<MultiLocation> = {
		MultiLocation { parents: 1, interior: Here } |
		MultiLocation { parents: 1, interior: X1(_) }
	};
}

pub fn get_all_module_accounts() -> Vec<AccountId> {
    vec![TreasuryModuleId::get().into_account_truncating()]
}

pub struct DustRemovalWhitelist;
impl Contains<AccountId> for DustRemovalWhitelist {
    fn contains(a: &AccountId) -> bool {
        get_all_module_accounts().contains(a)
    }
}

parameter_type_with_key! {
	pub ExistentialDeposits: |currency_id: CurrencyId| -> Balance {
		match currency_id {
			CurrencyId::NativeAssetId(symbol) => match symbol {
				NativeCurrency::Here => 0,
				NativeCurrency::Parent=> 0,
			},
			_ => 100_000
		}
	};
}

impl orml_tokens::Config for Runtime {
    type Event = Event;
    type Balance = Balance;
    type Amount = Amount;
    type CurrencyId = CurrencyId;
    type WeightInfo = ();
    type ExistentialDeposits = ExistentialDeposits;
    type OnDust = orml_tokens::TransferDust<Runtime, TreasuryAccountId>;
    type MaxLocks = MaxLocks;
    type MaxReserves = MaxReserves;
    // TODO: Add all module accounts
    type DustRemovalWhitelist = DustRemovalWhitelist;
    /// The id type for named reserves.
    type ReserveIdentifier = ();
    type OnNewTokenAccount = ();
    type OnKilledTokenAccount = ();
}

impl orml_xtokens::Config for Runtime {
    type Event = Event;
    type Balance = Balance;
    type CurrencyId = CurrencyId;
    type CurrencyIdConvert = CurrencyIdConvert;
    type AccountIdToMultiLocation = AccountIdToMultiLocation;
    type SelfLocation = SelfLocation;
    type XcmExecutor = XcmExecutor<XcmConfig<Self>>;
    type Weigher = FixedWeightBounds<UnitWeightCost, Call, MaxInstructions>;
    type BaseXcmWeight = BaseXcmWeight;
    type LocationInverter = LocationInverter<Ancestry>;
    type MaxAssetsForTransfer = MaxAssetsForTransfer;
    type MinXcmFee = ParachainMinFee;
    type MultiLocationsFilter = Everything;
    type ReserveProvider = AbsoluteReserveProvider;
}

pub struct CurrencyIdConvert;
impl Convert<AssetIds, Option<MultiLocation>> for CurrencyIdConvert {
    fn convert(id: AssetIds) -> Option<MultiLocation> {
        match id {
            AssetIds::NativeAssetId(NativeCurrency::Here) => Some(MultiLocation::new(
                1,
                X1(Parachain(ParachainInfo::get().into())),
            )),
            _ => None,
        }
    }
}

parameter_types! {
	pub const BaseXcmWeight: Weight = 100_000_000; // TODO: recheck this
	pub const MaxAssetsForTransfer: usize = 2;

    pub Ancestry: MultiLocation = Parachain(ParachainInfo::parachain_id().into()).into();
    pub SelfLocation: MultiLocation = MultiLocation::new(1, X1(Parachain(ParachainInfo::get().into())));
}

parameter_type_with_key! {
	pub ParachainMinFee: |_location: MultiLocation| -> Option<u128> {
		Some(100_000_000)
	};
}


pub struct AccountIdToMultiLocation;
impl Convert<AccountId, MultiLocation> for AccountIdToMultiLocation {
    fn convert(account: AccountId) -> MultiLocation {
        X1(AccountId32 {
            network: NetworkId::Any,
            id: account.into(),
        })
            .into()
    }
}
