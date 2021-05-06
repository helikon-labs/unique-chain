//
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.
//

#![recursion_limit = "1024"]

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
pub use serde::*;

use core::ops::{Deref, DerefMut};
use codec::{Decode, Encode};
pub use frame_support::{
    construct_runtime, decl_event, decl_module, decl_storage, decl_error,
    dispatch::DispatchResult,
    ensure, fail, parameter_types,
    traits::{
        Currency, ExistenceRequirement, Get, Imbalance, KeyOwnerProofSystem, OnUnbalanced,
        Randomness, IsSubType, WithdrawReasons,
    },
    weights::{
        constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
        DispatchInfo, GetDispatchInfo, IdentityFee, Pays, PostDispatchInfo, Weight,
        WeightToFeePolynomial, DispatchClass,
    },
    StorageValue,
    transactional,
};

use frame_system::{self as system, ensure_signed, ensure_root};
use sp_core::{H160, H256};
use sp_runtime::sp_std::prelude::Vec;
use sp_runtime::{
    traits::{
        Hash, DispatchInfoOf, Dispatchable, PostDispatchInfoOf, Saturating, SaturatedConversion, SignedExtension, Zero,
    },
    transaction_validity::{
        TransactionPriority, InvalidTransaction, TransactionValidity, TransactionValidityError, ValidTransaction,
    },
    FixedPointOperand, FixedU128,
};
use sp_runtime::traits::StaticLookup;
use pallet_contracts::chain_extension::UncheckedFrom;
use pallet_ethereum::EthereumTransactionSender;
use pallet_transaction_payment::OnChargeTransaction;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod default_weights;
mod eth;

pub use eth::NftErcSupport;
pub use eth::account::*;

pub const MAX_DECIMAL_POINTS: DecimalPoints = 30;
pub const MAX_REFUNGIBLE_PIECES: u128 = 1_000_000_000_000_000_000_000;
pub const MAX_SPONSOR_TIMEOUT: u32 = 10_368_000;
pub const MAX_TOKEN_OWNERSHIP: u32 = 10_000_000;

// Structs
// #region

pub type CollectionId = u32;
pub type TokenId = u32;
pub type DecimalPoints = u8;

#[derive(Encode, Decode, Eq, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum CollectionMode {
    Invalid,
    NFT,
    // decimal points
    Fungible(DecimalPoints),
    ReFungible,
}

impl Default for CollectionMode {
    fn default() -> Self {
        Self::Invalid
    }
}

impl Into<u8> for CollectionMode {
    fn into(self) -> u8 {
        match self {
            CollectionMode::Invalid => 0,
            CollectionMode::NFT => 1,
            CollectionMode::Fungible(_) => 2,
            CollectionMode::ReFungible => 3,
        }
    }
}

#[derive(Encode, Decode, Eq, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum AccessMode {
    Normal,
    WhiteList,
}
impl Default for AccessMode {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Encode, Decode, Eq, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum SchemaVersion {
    ImageURL,
    Unique,
}
impl Default for SchemaVersion {
    fn default() -> Self {
        Self::ImageURL
    }
}

#[derive(Encode, Decode, Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Ownership<AccountId> {
    pub owner: AccountId,
    pub fraction: u128,
}

#[derive(Encode, Decode, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum SponsorshipState<AccountId> {
    /// The fees are applied to the transaction sender
    Disabled,
    Unconfirmed(AccountId),
    /// Transactions are sponsored by specified account
    Confirmed(AccountId),
}

impl<AccountId> SponsorshipState<AccountId> {
    fn sponsor(&self) -> Option<&AccountId> {
        match self {
            Self::Confirmed(sponsor) => Some(sponsor),
            _ => None,
        }
    }

    fn pending_sponsor(&self) -> Option<&AccountId> {
        match self {
            Self::Unconfirmed(sponsor) | Self::Confirmed(sponsor) => Some(sponsor),
            _ => None,
        }
    }

    fn confirmed(&self) -> bool {
        matches!(self, Self::Confirmed(_))
    }
}

impl<T> Default for SponsorshipState<T> {
    fn default() -> Self {
        Self::Disabled
    }
}

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Collection<T: Config> {
    pub owner: T::CrossAccountId,
    pub mode: CollectionMode,
    pub access: AccessMode,
    pub decimal_points: DecimalPoints,
    pub name: Vec<u16>,        // 64 include null escape char
    pub description: Vec<u16>, // 256 include null escape char
    pub token_prefix: Vec<u8>, // 16 include null escape char
    pub mint_mode: bool,
    pub offchain_schema: Vec<u8>,
    pub schema_version: SchemaVersion,
    pub sponsorship: SponsorshipState<T::AccountId>,
    pub limits: CollectionLimits<T::BlockNumber>, // Collection private restrictions 
    pub variable_on_chain_schema: Vec<u8>, //
    pub const_on_chain_schema: Vec<u8>, //
}

pub struct CollectionHandle<T: Config> {
    pub id: CollectionId,
    collection: Collection<T>,
    logs: eth::log::LogRecorder,
}
impl<T: Config> CollectionHandle<T> {
	pub fn get(id: CollectionId) -> Option<Self> {
		<CollectionById<T>>::get(id)
			.map(|collection| Self {
				id,
				collection,
                logs: eth::log::LogRecorder::default(),
			})
	}
    pub fn log(&self, topics: Vec<H256>, data: eth::abi::AbiWriter) {
        self.logs.log(topics, data)
    }
    pub fn into_inner(self) -> Collection<T> {
        self.collection.clone()
    }
}

impl<T: Config> Deref for CollectionHandle<T> {
    type Target = Collection<T>;

    fn deref(&self) -> &Self::Target {
        &self.collection
    }
}

impl<T: Config> DerefMut for CollectionHandle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.collection
    }
}

#[derive(Encode, Decode, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct NftItemType<AccountId> {
    pub owner: AccountId,
    pub const_data: Vec<u8>,
    pub variable_data: Vec<u8>,
}

#[derive(Encode, Decode, Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct FungibleItemType {
    pub value: u128,
}

#[derive(Encode, Decode, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ReFungibleItemType<AccountId> {
    pub owner: Vec<Ownership<AccountId>>,
    pub const_data: Vec<u8>,
    pub variable_data: Vec<u8>,
}

// #[derive(Encode, Decode, Default, Debug, Clone, PartialEq)]
// #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
// pub struct VestingItem<AccountId, Moment> {
//     pub sender: AccountId,
//     pub recipient: AccountId,
//     pub collection_id: CollectionId,
//     pub item_id: TokenId,
//     pub amount: u64,
//     pub vesting_date: Moment,
// }

#[derive(Encode, Decode, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct CollectionLimits<BlockNumber: Encode + Decode> {
    pub account_token_ownership_limit: u32,
    pub sponsored_data_size: u32,
    /// None - setVariableMetadata is not sponsored
    /// Some(v) - setVariableMetadata is sponsored 
    ///           if there is v block between txs
    pub sponsored_data_rate_limit: Option<BlockNumber>,
    pub token_limit: u32,

    // Timeouts for item types in passed blocks
    pub sponsor_transfer_timeout: u32,
    pub owner_can_transfer: bool,
    pub owner_can_destroy: bool,
}

impl<BlockNumber: Encode + Decode> Default for CollectionLimits<BlockNumber> {
    fn default() -> Self {
        Self { 
            account_token_ownership_limit: 10_000_000, 
            token_limit: u32::max_value(),
            sponsored_data_size: u32::MAX, 
            sponsored_data_rate_limit: None,
            sponsor_transfer_timeout: 14400,
            owner_can_transfer: true,
            owner_can_destroy: true
        }
    }
}

#[derive(Encode, Decode, Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ChainLimits {
    pub collection_numbers_limit: u32,
    pub account_token_ownership_limit: u32,
    pub collections_admins_limit: u64,
    pub custom_data_limit: u32,

    // Timeouts for item types in passed blocks
    pub nft_sponsor_transfer_timeout: u32,
    pub fungible_sponsor_transfer_timeout: u32,
    pub refungible_sponsor_transfer_timeout: u32,

    // Schema limits
    pub offchain_schema_limit: u32,
    pub variable_on_chain_schema_limit: u32,
    pub const_on_chain_schema_limit: u32,
}

pub trait WeightInfo {
	fn create_collection() -> Weight;
	fn destroy_collection() -> Weight;
	fn add_to_white_list() -> Weight;
	fn remove_from_white_list() -> Weight;
    fn set_public_access_mode() -> Weight;
    fn set_mint_permission() -> Weight;
    fn change_collection_owner() -> Weight;
    fn add_collection_admin() -> Weight;
    fn remove_collection_admin() -> Weight;
    fn set_collection_sponsor() -> Weight;
    fn confirm_sponsorship() -> Weight;
    fn remove_collection_sponsor() -> Weight;
    fn create_item(s: usize) -> Weight;
    fn burn_item() -> Weight;
    fn transfer() -> Weight;
    fn approve() -> Weight;
    fn transfer_from() -> Weight;
    fn set_offchain_schema() -> Weight;
    fn set_const_on_chain_schema() -> Weight;
    fn set_variable_on_chain_schema() -> Weight;
    fn set_variable_meta_data() -> Weight;
    fn enable_contract_sponsoring() -> Weight;
    fn set_schema_version() -> Weight;
    fn set_chain_limits() -> Weight;
    fn set_contract_sponsoring_rate_limit() -> Weight;
    fn set_variable_meta_data_sponsoring_rate_limit() -> Weight;
    fn toggle_contract_white_list() -> Weight;
    fn add_to_contract_white_list() -> Weight;
    fn remove_from_contract_white_list() -> Weight;
    fn set_collection_limits() -> Weight;
}

#[derive(Encode, Decode, Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct CreateNftData {
    pub const_data: Vec<u8>,
    pub variable_data: Vec<u8>,
}

#[derive(Encode, Decode, Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct CreateFungibleData {
    pub value: u128,
}

#[derive(Encode, Decode, Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct CreateReFungibleData {
    pub const_data: Vec<u8>,
    pub variable_data: Vec<u8>,
    pub pieces: u128,
}

#[derive(Encode, Decode, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum CreateItemData {
    NFT(CreateNftData),
    Fungible(CreateFungibleData),
    ReFungible(CreateReFungibleData),
}

impl CreateItemData {
    pub fn len(&self) -> usize {
        let len = match self {
            CreateItemData::NFT(data) => data.variable_data.len() + data.const_data.len(),
            CreateItemData::ReFungible(data) => data.variable_data.len() + data.const_data.len(),
            _ => 0
        };
        
        return len;
    }
}

impl From<CreateNftData> for CreateItemData {
    fn from(item: CreateNftData) -> Self {
        CreateItemData::NFT(item)
    }
}

impl From<CreateReFungibleData> for CreateItemData {
    fn from(item: CreateReFungibleData) -> Self {
        CreateItemData::ReFungible(item)
    }
}

impl From<CreateFungibleData> for CreateItemData {
    fn from(item: CreateFungibleData) -> Self {
        CreateItemData::Fungible(item)
    }
}


decl_error! {
	/// Error for non-fungible-token module.
	pub enum Error for Module<T: Config> {
        /// Total collections bound exceeded.
        TotalCollectionsLimitExceeded,
		/// Decimal_points parameter must be lower than MAX_DECIMAL_POINTS constant, currently it is 30.
        CollectionDecimalPointLimitExceeded, 
        /// Collection name can not be longer than 63 char.
        CollectionNameLimitExceeded, 
        /// Collection description can not be longer than 255 char.
        CollectionDescriptionLimitExceeded, 
        /// Token prefix can not be longer than 15 char.
        CollectionTokenPrefixLimitExceeded,
        /// This collection does not exist.
        CollectionNotFound,
        /// Item not exists.
        TokenNotFound,
        /// Admin not found
        AdminNotFound,
        /// Arithmetic calculation overflow.
        NumOverflow,       
        /// Account already has admin role.
        AlreadyAdmin,  
        /// You do not own this collection.
        NoPermission,
        /// This address is not set as sponsor, use setCollectionSponsor first.
        ConfirmUnsetSponsorFail,
        /// Collection is not in mint mode.
        PublicMintingNotAllowed,
        /// Sender parameter and item owner must be equal.
        MustBeTokenOwner,
        /// Item balance not enough.
        TokenValueTooLow,
        /// Size of item is too large.
        NftSizeLimitExceeded,
        /// No approve found
        ApproveNotFound,
        /// Requested value more than approved.
        TokenValueNotEnough,
        /// Only approved addresses can call this method.
        ApproveRequired,
        /// Address is not in white list.
        AddresNotInWhiteList,
        /// Number of collection admins bound exceeded.
        CollectionAdminsLimitExceeded,
        /// Owned tokens by a single address bound exceeded.
        AddressOwnershipLimitExceeded,
        /// Length of items properties must be greater than 0.
        EmptyArgument,
        /// const_data exceeded data limit.
        TokenConstDataLimitExceeded,
        /// variable_data exceeded data limit.
        TokenVariableDataLimitExceeded,
        /// Not NFT item data used to mint in NFT collection.
        NotNftDataUsedToMintNftCollectionToken,
        /// Not Fungible item data used to mint in Fungible collection.
        NotFungibleDataUsedToMintFungibleCollectionToken,
        /// Not Re Fungible item data used to mint in Re Fungible collection.
        NotReFungibleDataUsedToMintReFungibleCollectionToken,
        /// Unexpected collection type.
        UnexpectedCollectionType,
        /// Can't store metadata in fungible tokens.
        CantStoreMetadataInFungibleTokens,
        /// Collection token limit exceeded
        CollectionTokenLimitExceeded,
        /// Account token limit exceeded per collection
        AccountTokenLimitExceeded,
        /// Collection limit bounds per collection exceeded
        CollectionLimitBoundsExceeded,
        /// Tried to enable permissions which are only permitted to be disabled
        OwnerPermissionsCantBeReverted,
        /// Schema data size limit bound exceeded
        SchemaDataLimitExceeded,
        /// Maximum refungibility exceeded
        WrongRefungiblePieces,
        /// createRefungible should be called with one owner
        BadCreateRefungibleCall,
	}
}

pub trait Config: system::Config + Sized + pallet_transaction_payment::Config + pallet_contracts::Config {
    type Event: From<Event<Self>> + Into<<Self as system::Config>::Event>;

    /// Weight information for extrinsics in this pallet.
	type WeightInfo: WeightInfo;

    type EvmAddressMapping: pallet_evm::AddressMapping<Self::AccountId>;
    type EvmBackwardsAddressMapping: EvmBackwardsAddressMapping<Self::AccountId>;
    type EvmWithdrawOrigin: pallet_evm::EnsureAddressOrigin<Self::Origin, Success = Self::AccountId>;

	type CrossAccountId: CrossAccountId<Self::AccountId>;
    type Currency: Currency<Self::AccountId>;
    type CollectionCreationPrice: Get<<<Self as Config>::Currency as Currency<Self::AccountId>>::Balance>;
    type TreasuryAccountId: Get<Self::AccountId>;

    type EthereumChainId: Get<u64>;
    type EthereumTransactionSender: pallet_ethereum::EthereumTransactionSender;
}

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

// #endregion

// # Used definitions
//
// ## User control levels
//
// chain-controlled - key is uncontrolled by user
//                    i.e autoincrementing index
//                    can use non-cryptographic hash
// real - key is controlled by user
//        but it is hard to generate enough colliding values, i.e owner of signed txs
//        can use non-cryptographic hash
// controlled - key is completly controlled by users
//              i.e maps with mutable keys
//              should use cryptographic hash
//
// ## User control level downgrade reasons
//
// ?1 - chain-controlled -> controlled
//      collections/tokens can be destroyed, resulting in massive holes
// ?2 - chain-controlled -> controlled
//      same as ?1, but can be only added, resulting in easier exploitation
// ?3 - real -> controlled
//      no confirmation required, so addresses can be easily generated
decl_storage! {
    trait Store for Module<T: Config> as Nft {

        //#region Private members
        /// Id of next collection
        CreatedCollectionCount: u32;
        /// Used for migrations
        ChainVersion: u64;
        /// Id of last collection token
        /// Collection id (controlled?1)
        ItemListIndex: map hasher(blake2_128_concat) CollectionId => TokenId;
        //#endregion

        //#region Chain limits struct
        pub ChainLimit get(fn chain_limit) config(): ChainLimits;
        //#endregion

        //#region Bound counters
        /// Amount of collections destroyed, used for total amount tracking with
        /// CreatedCollectionCount
        DestroyedCollectionCount: u32;
        /// Total amount of account owned tokens (NFTs + RFTs + unique fungibles)
        /// Account id (real)
        pub AccountItemCount get(fn account_item_count): map hasher(twox_64_concat) T::AccountId => u32;
        //#endregion

        //#region Basic collections
        /// Collection info
        /// Collection id (controlled?1)
        pub CollectionById get(fn collection_id) config(): map hasher(blake2_128_concat) CollectionId => Option<Collection<T>> = None;
        /// List of collection admins
        /// Collection id (controlled?2)
        pub AdminList get(fn admin_list_collection): map hasher(blake2_128_concat) CollectionId => Vec<T::CrossAccountId>;
        /// Whitelisted collection users
        /// Collection id (controlled?2), user id (controlled?3)
        pub WhiteList get(fn white_list): double_map hasher(blake2_128_concat) CollectionId, hasher(blake2_128_concat) T::AccountId => bool;
        //#endregion

        /// How many of collection items user have
        /// Collection id (controlled?2), account id (real)
        pub Balance get(fn balance_count): double_map hasher(blake2_128_concat) CollectionId, hasher(twox_64_concat) T::AccountId => u128;

        /// Amount of items which spender can transfer out of owners account (via transferFrom)
        /// Collection id (controlled?2), (token id (controlled ?2) + owner account id (real) + spender account id (controlled?3))
        /// TODO: Off chain worker should remove from this map when token gets removed
        pub Allowances get(fn approved): double_map hasher(blake2_128_concat) CollectionId, hasher(blake2_128_concat) (TokenId, T::AccountId, T::AccountId) => u128;

        //#region Item collections
        /// Collection id (controlled?2), token id (controlled?1)
        pub NftItemList get(fn nft_item_id) config(): double_map hasher(blake2_128_concat) CollectionId, hasher(blake2_128_concat) TokenId => Option<NftItemType<T::CrossAccountId>>;
        /// Collection id (controlled?2), owner (controlled?2)
        pub FungibleItemList get(fn fungible_item_id) config(): double_map hasher(blake2_128_concat) CollectionId, hasher(blake2_128_concat) T::AccountId => FungibleItemType;
        /// Collection id (controlled?2), token id (controlled?1)
        pub ReFungibleItemList get(fn refungible_item_id) config(): double_map hasher(blake2_128_concat) CollectionId, hasher(blake2_128_concat) TokenId => Option<ReFungibleItemType<T::CrossAccountId>>;
        //#endregion

        //#region Index list
        /// Collection id (controlled?2), tokens owner (controlled?2)
        pub AddressTokens get(fn address_tokens): double_map hasher(blake2_128_concat) CollectionId, hasher(blake2_128_concat) T::AccountId => Vec<TokenId>;
        //#endregion

        //#region Tokens transfer rate limit baskets
        /// (Collection id (controlled?2), who created (real))
        /// TODO: Off chain worker should remove from this map when collection gets removed
        pub CreateItemBasket get(fn create_item_basket): map hasher(blake2_128_concat) (CollectionId, T::AccountId) => T::BlockNumber;
        /// Collection id (controlled?2), token id (controlled?2)
        pub NftTransferBasket get(fn nft_transfer_basket): double_map hasher(blake2_128_concat) CollectionId, hasher(blake2_128_concat) TokenId => T::BlockNumber;
        /// Collection id (controlled?2), owning user (real)
        pub FungibleTransferBasket get(fn fungible_transfer_basket): double_map hasher(blake2_128_concat) CollectionId, hasher(twox_64_concat) T::AccountId => T::BlockNumber;
        /// Collection id (controlled?2), token id (controlled?2)
        pub ReFungibleTransferBasket get(fn refungible_transfer_basket): double_map hasher(blake2_128_concat) CollectionId, hasher(blake2_128_concat) TokenId => T::BlockNumber;
        //#endregion

        /// Variable metadata sponsoring
        /// Collection id (controlled?2), token id (controlled?2)
        pub VariableMetaDataBasket get(fn variable_meta_data_basket): double_map hasher(blake2_128_concat) CollectionId, hasher(blake2_128_concat) TokenId => Option<T::BlockNumber> = None;
      
        //#region Contract Sponsorship and Ownership
        /// Contract address (real)
        pub ContractOwner get(fn contract_owner): map hasher(twox_64_concat) T::AccountId => Option<T::AccountId>;
        /// Contract address (real)
        pub ContractSelfSponsoring get(fn contract_self_sponsoring): map hasher(twox_64_concat) T::AccountId => bool;
        /// (Contract address(real), caller (real))
        pub ContractSponsorBasket get(fn contract_sponsor_basket): map hasher(twox_64_concat) (T::AccountId, T::AccountId) => T::BlockNumber;
        /// Contract address (real)
        pub ContractSponsoringRateLimit get(fn contract_sponsoring_rate_limit): map hasher(twox_64_concat) T::AccountId => T::BlockNumber;
        /// Contract address (real)
        pub ContractWhiteListEnabled get(fn contract_white_list_enabled): map hasher(twox_64_concat) T::AccountId => bool; 
        /// Contract address (real) => Whitelisted user (controlled?3)
        pub ContractWhiteList get(fn contract_white_list): double_map hasher(twox_64_concat) T::AccountId, hasher(blake2_128_concat) T::AccountId => bool; 
        //#endregion
    }
    add_extra_genesis {
        build(|config: &GenesisConfig<T>| {
            // Modification of storage
            for (_num, _c) in &config.collection_id {
                <Module<T>>::init_collection(_c);
            }

            for (_num, _c, _i) in &config.nft_item_id {
                <Module<T>>::init_nft_token(*_c, _i);
            }

            for (collection_id, account_id, fungible_item) in &config.fungible_item_id {
                <Module<T>>::init_fungible_token(*collection_id, &T::CrossAccountId::from_sub(account_id.clone()), fungible_item);
            }

            for (_num, _c, _i) in &config.refungible_item_id {
                <Module<T>>::init_refungible_token(*_c, _i);
            }
        })
    }
}

decl_event!(
    pub enum Event<T>
    where
        CrossAccountId = <T as Config>::CrossAccountId,
    {
        /// New collection was created
        /// 
        /// # Arguments
        /// 
        /// * collection_id: Globally unique identifier of newly created collection.
        /// 
        /// * mode: [CollectionMode] converted into u8.
        /// 
        /// * account_id: Collection owner.
        CollectionCreated(CollectionId, u8, CrossAccountId),

        /// New item was created.
        /// 
        /// # Arguments
        /// 
        /// * collection_id: Id of the collection where item was created.
        /// 
        /// * item_id: Id of an item. Unique within the collection.
        ///
        /// * recipient: Owner of newly created item 
        ItemCreated(CollectionId, TokenId, CrossAccountId),

        /// Collection item was burned.
        /// 
        /// # Arguments
        /// 
        /// collection_id.
        /// 
        /// item_id: Identifier of burned NFT.
        ItemDestroyed(CollectionId, TokenId),

        /// Item was transferred
        ///
        /// * collection_id: Id of collection to which item is belong
        ///
        /// * item_id: Id of an item
        ///
        /// * sender: Original owner of item
        ///
        /// * recipient: New owner of item
        ///
        /// * amount: Always 1 for NFT
        Transfer(CollectionId, TokenId, CrossAccountId, CrossAccountId, u128),

        /// * collection_id
        ///
        /// * item_id
        ///
        /// * sender
        ///
        /// * spender
        ///
        /// * amount
        Approved(CollectionId, TokenId, CrossAccountId, CrossAccountId, u128),
    }
);

decl_module! {
    pub struct Module<T: Config> for enum Call 
    where 
        origin: T::Origin
    {
        fn deposit_event() = default;
        type Error = Error<T>;

        fn on_initialize(now: T::BlockNumber) -> Weight {
            0
        }

        /// This method creates a Collection of NFTs. Each Token may have multiple properties encoded as an array of bytes of certain length. The initial owner and admin of the collection are set to the address that signed the transaction. Both addresses can be changed later.
        /// 
        /// # Permissions
        /// 
        /// * Anyone.
        /// 
        /// # Arguments
        /// 
        /// * collection_name: UTF-16 string with collection name (limit 64 characters), will be stored as zero-terminated.
        /// 
        /// * collection_description: UTF-16 string with collection description (limit 256 characters), will be stored as zero-terminated.
        /// 
        /// * token_prefix: UTF-8 string with token prefix.
        /// 
        /// * mode: [CollectionMode] collection type and type dependent data.
        // returns collection ID
        #[weight = <T as Config>::WeightInfo::create_collection()]
        #[transactional]
        pub fn create_collection(origin,
                                 collection_name: Vec<u16>,
                                 collection_description: Vec<u16>,
                                 token_prefix: Vec<u8>,
                                 mode: CollectionMode) -> DispatchResult {

            // Anyone can create a collection
            let who = T::CrossAccountId::from_sub(ensure_signed(origin)?);

            // Take a (non-refundable) deposit of collection creation
            let mut imbalance = <<<T as Config>::Currency as Currency<T::AccountId>>::PositiveImbalance>::zero();
            imbalance.subsume(<<T as Config>::Currency as Currency<T::AccountId>>::deposit_creating(
                &T::TreasuryAccountId::get(),
                T::CollectionCreationPrice::get(),
            ));
            <T as Config>::Currency::settle(
                who.as_sub(),
                imbalance,
                WithdrawReasons::TRANSFER,
                ExistenceRequirement::KeepAlive,
            ).map_err(|_| Error::<T>::NoPermission)?;

            let decimal_points = match mode {
                CollectionMode::Fungible(points) => points,
                _ => 0
            };

            let chain_limit = ChainLimit::get();

            let created_count = CreatedCollectionCount::get();
            let destroyed_count = DestroyedCollectionCount::get();

            // bound Total number of collections
            ensure!(created_count - destroyed_count < chain_limit.collection_numbers_limit, Error::<T>::TotalCollectionsLimitExceeded);

            // check params
            ensure!(decimal_points <= MAX_DECIMAL_POINTS, Error::<T>::CollectionDecimalPointLimitExceeded);
            ensure!(collection_name.len() <= 64, Error::<T>::CollectionNameLimitExceeded);
            ensure!(collection_description.len() <= 256, Error::<T>::CollectionDescriptionLimitExceeded);
            ensure!(token_prefix.len() <= 16, Error::<T>::CollectionTokenPrefixLimitExceeded);

            // Generate next collection ID
            let next_id = created_count
                .checked_add(1)
                .ok_or(Error::<T>::NumOverflow)?;

            CreatedCollectionCount::put(next_id);

            let limits = CollectionLimits {
                sponsored_data_size: chain_limit.custom_data_limit,
                ..Default::default()
            };

            // Create new collection
            let new_collection = Collection {
                owner: who.clone(),
                name: collection_name,
                mode: mode.clone(),
                mint_mode: false,
                access: AccessMode::Normal,
                description: collection_description,
                decimal_points: decimal_points,
                token_prefix: token_prefix,
                offchain_schema: Vec::new(),
                schema_version: SchemaVersion::ImageURL,
                sponsorship: SponsorshipState::Disabled,
                variable_on_chain_schema: Vec::new(),
                const_on_chain_schema: Vec::new(),
                limits,
            };

            // Add new collection to map
            <CollectionById<T>>::insert(next_id, new_collection);

            // call event
            Self::deposit_event(RawEvent::CollectionCreated(next_id, mode.into(), who));

            Ok(())
        }

        /// **DANGEROUS**: Destroys collection and all NFTs within this collection. Users irrecoverably lose their assets and may lose real money.
        /// 
        /// # Permissions
        /// 
        /// * Collection Owner.
        /// 
        /// # Arguments
        /// 
        /// * collection_id: collection to destroy.
        #[weight = <T as Config>::WeightInfo::destroy_collection()]
        #[transactional]
        pub fn destroy_collection(origin, collection_id: CollectionId) -> DispatchResult {

            let sender = T::CrossAccountId::from_sub(ensure_signed(origin)?);
            let collection = Self::get_collection(collection_id)?;
            Self::check_owner_permissions(&collection, &sender)?;
            if !collection.limits.owner_can_destroy {
                fail!(Error::<T>::NoPermission);
            }

            <AddressTokens<T>>::remove_prefix(collection_id);
            <Allowances<T>>::remove_prefix(collection_id);
            <Balance<T>>::remove_prefix(collection_id);
            <ItemListIndex>::remove(collection_id);
            <AdminList<T>>::remove(collection_id);
            <CollectionById<T>>::remove(collection_id);
            <WhiteList<T>>::remove_prefix(collection_id);

            <NftItemList<T>>::remove_prefix(collection_id);
            <FungibleItemList<T>>::remove_prefix(collection_id);
            <ReFungibleItemList<T>>::remove_prefix(collection_id);

            <NftTransferBasket<T>>::remove_prefix(collection_id);
            <FungibleTransferBasket<T>>::remove_prefix(collection_id);
            <ReFungibleTransferBasket<T>>::remove_prefix(collection_id);

            <VariableMetaDataBasket<T>>::remove_prefix(collection_id);

            DestroyedCollectionCount::put(DestroyedCollectionCount::get()
                .checked_add(1)
                .ok_or(Error::<T>::NumOverflow)?);

            Ok(())
        }

        /// Add an address to white list.
        /// 
        /// # Permissions
        /// 
        /// * Collection Owner
        /// * Collection Admin
        /// 
        /// # Arguments
        /// 
        /// * collection_id.
        /// 
        /// * address.
        #[weight = <T as Config>::WeightInfo::add_to_white_list()]
        #[transactional]
        pub fn add_to_white_list(origin, collection_id: CollectionId, address: T::CrossAccountId) -> DispatchResult{

            let sender = T::CrossAccountId::from_sub(ensure_signed(origin)?);
            let collection = Self::get_collection(collection_id)?;

            Self::toggle_white_list_internal(
                &sender,
                &collection,
                &address,
                true,
            )?;

            Ok(())
        }

        /// Remove an address from white list.
        /// 
        /// # Permissions
        /// 
        /// * Collection Owner
        /// * Collection Admin
        /// 
        /// # Arguments
        /// 
        /// * collection_id.
        /// 
        /// * address.
        #[weight = <T as Config>::WeightInfo::remove_from_white_list()]
        #[transactional]
        pub fn remove_from_white_list(origin, collection_id: CollectionId, address: T::CrossAccountId) -> DispatchResult{

            let sender = T::CrossAccountId::from_sub(ensure_signed(origin)?);
            let collection = Self::get_collection(collection_id)?;

            Self::toggle_white_list_internal(
                &sender,
                &collection,
                &address,
                false,
            )?;

            Ok(())
        }

        /// Toggle between normal and white list access for the methods with access for `Anyone`.
        /// 
        /// # Permissions
        /// 
        /// * Collection Owner.
        /// 
        /// # Arguments
        /// 
        /// * collection_id.
        /// 
        /// * mode: [AccessMode]
        #[weight = <T as Config>::WeightInfo::set_public_access_mode()]
        #[transactional]
        pub fn set_public_access_mode(origin, collection_id: CollectionId, mode: AccessMode) -> DispatchResult
        {
            let sender = T::CrossAccountId::from_sub(ensure_signed(origin)?);

            let mut target_collection = Self::get_collection(collection_id)?;
            Self::check_owner_permissions(&target_collection, &sender)?;
            target_collection.access = mode;
            Self::save_collection(target_collection);

            Ok(())
        }

        /// Allows Anyone to create tokens if:
        /// * White List is enabled, and
        /// * Address is added to white list, and
        /// * This method was called with True parameter
        /// 
        /// # Permissions
        /// * Collection Owner
        ///
        /// # Arguments
        /// 
        /// * collection_id.
        /// 
        /// * mint_permission: Boolean parameter. If True, allows minting to Anyone with conditions above.
        #[weight = <T as Config>::WeightInfo::set_mint_permission()]
        #[transactional]
        pub fn set_mint_permission(origin, collection_id: CollectionId, mint_permission: bool) -> DispatchResult
        {
            let sender = T::CrossAccountId::from_sub(ensure_signed(origin)?);

            let mut target_collection = Self::get_collection(collection_id)?;
            Self::check_owner_permissions(&target_collection, &sender)?;
            target_collection.mint_mode = mint_permission;
            Self::save_collection(target_collection);

            Ok(())
        }

        /// Change the owner of the collection.
        /// 
        /// # Permissions
        /// 
        /// * Collection Owner.
        /// 
        /// # Arguments
        /// 
        /// * collection_id.
        /// 
        /// * new_owner.
        #[weight = <T as Config>::WeightInfo::change_collection_owner()]
        #[transactional]
        pub fn change_collection_owner(origin, collection_id: CollectionId, new_owner: T::CrossAccountId) -> DispatchResult {

            let sender = T::CrossAccountId::from_sub(ensure_signed(origin)?);
            let mut target_collection = Self::get_collection(collection_id)?;
            Self::check_owner_permissions(&target_collection, &sender)?;
            target_collection.owner = new_owner;
            Self::save_collection(target_collection);

            Ok(())
        }

        /// Adds an admin of the Collection.
        /// NFT Collection can be controlled by multiple admin addresses (some which can also be servers, for example). Admins can issue and burn NFTs, as well as add and remove other admins, but cannot change NFT or Collection ownership. 
        /// 
        /// # Permissions
        /// 
        /// * Collection Owner.
        /// * Collection Admin.
        /// 
        /// # Arguments
        /// 
        /// * collection_id: ID of the Collection to add admin for.
        /// 
        /// * new_admin_id: Address of new admin to add.
        #[weight = <T as Config>::WeightInfo::add_collection_admin()]
        #[transactional]
        pub fn add_collection_admin(origin, collection_id: CollectionId, new_admin_id: T::CrossAccountId) -> DispatchResult {
            let sender = T::CrossAccountId::from_sub(ensure_signed(origin)?);
            let collection = Self::get_collection(collection_id)?;
            Self::check_owner_or_admin_permissions(&collection, &sender)?;
            let mut admin_arr = <AdminList<T>>::get(collection_id);

            match admin_arr.binary_search(&new_admin_id) {
                Ok(_) => {},
                Err(idx) => {
                    let limits = ChainLimit::get();
                    ensure!(admin_arr.len() < limits.collections_admins_limit as usize, Error::<T>::CollectionAdminsLimitExceeded);
                    admin_arr.insert(idx, new_admin_id);
                    <AdminList<T>>::insert(collection_id, admin_arr);
                }
            }
            Ok(())
        }

        /// Remove admin address of the Collection. An admin address can remove itself. List of admins may become empty, in which case only Collection Owner will be able to add an Admin.
        ///
        /// # Permissions
        /// 
        /// * Collection Owner.
        /// * Collection Admin.
        /// 
        /// # Arguments
        /// 
        /// * collection_id: ID of the Collection to remove admin for.
        /// 
        /// * account_id: Address of admin to remove.
        #[weight = <T as Config>::WeightInfo::remove_collection_admin()]
        #[transactional]
        pub fn remove_collection_admin(origin, collection_id: CollectionId, account_id: T::CrossAccountId) -> DispatchResult {
            let sender = T::CrossAccountId::from_sub(ensure_signed(origin)?);
            let collection = Self::get_collection(collection_id)?;
            Self::check_owner_or_admin_permissions(&collection, &sender)?;
            let mut admin_arr = <AdminList<T>>::get(collection_id);

            match admin_arr.binary_search(&account_id) {
                Ok(idx) => {
                    admin_arr.remove(idx);
                    <AdminList<T>>::insert(collection_id, admin_arr);
                },
                Err(_) => {}
            }
            Ok(())
        }

        /// # Permissions
        /// 
        /// * Collection Owner
        /// 
        /// # Arguments
        /// 
        /// * collection_id.
        /// 
        /// * new_sponsor.
        #[weight = <T as Config>::WeightInfo::set_collection_sponsor()]
        #[transactional]
        pub fn set_collection_sponsor(origin, collection_id: CollectionId, new_sponsor: T::AccountId) -> DispatchResult {
            let sender = T::CrossAccountId::from_sub(ensure_signed(origin)?);
            let mut target_collection = Self::get_collection(collection_id)?;
            Self::check_owner_permissions(&target_collection, &sender)?;

            target_collection.sponsorship = SponsorshipState::Unconfirmed(new_sponsor);
            Self::save_collection(target_collection);

            Ok(())
        }

        /// # Permissions
        /// 
        /// * Sponsor.
        /// 
        /// # Arguments
        /// 
        /// * collection_id.
        #[weight = <T as Config>::WeightInfo::confirm_sponsorship()]
        #[transactional]
        pub fn confirm_sponsorship(origin, collection_id: CollectionId) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let mut target_collection = Self::get_collection(collection_id)?;
            ensure!(
                target_collection.sponsorship.pending_sponsor() == Some(&sender),
                Error::<T>::ConfirmUnsetSponsorFail
            );

            target_collection.sponsorship = SponsorshipState::Confirmed(sender);
            Self::save_collection(target_collection);

            Ok(())
        }

        /// Switch back to pay-per-own-transaction model.
        ///
        /// # Permissions
        ///
        /// * Collection owner.
        /// 
        /// # Arguments
        /// 
        /// * collection_id.
        #[weight = <T as Config>::WeightInfo::remove_collection_sponsor()]
        #[transactional]
        pub fn remove_collection_sponsor(origin, collection_id: CollectionId) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let mut target_collection = Self::get_collection(collection_id)?;
            Self::check_owner_permissions(&target_collection, &T::CrossAccountId::from_sub(sender))?;

            target_collection.sponsorship = SponsorshipState::Disabled;
            Self::save_collection(target_collection);

            Ok(())
        }

        /// This method creates a concrete instance of NFT Collection created with CreateCollection method.
        /// 
        /// # Permissions
        /// 
        /// * Collection Owner.
        /// * Collection Admin.
        /// * Anyone if
        ///     * White List is enabled, and
        ///     * Address is added to white list, and
        ///     * MintPermission is enabled (see SetMintPermission method)
        /// 
        /// # Arguments
        /// 
        /// * collection_id: ID of the collection.
        /// 
        /// * owner: Address, initial owner of the NFT.
        ///
        /// * data: Token data to store on chain.
        // #[weight =
        // (130_000_000 as Weight)
        // .saturating_add((2135 as Weight).saturating_mul((properties.len() as u64) as Weight))
        // .saturating_add(RocksDbWeight::get().reads(10 as Weight))
        // .saturating_add(RocksDbWeight::get().writes(8 as Weight))]

        #[weight = <T as Config>::WeightInfo::create_item(data.len())]
        #[transactional]
        pub fn create_item(origin, collection_id: CollectionId, owner: T::CrossAccountId, data: CreateItemData) -> DispatchResult {
            let sender = T::CrossAccountId::from_sub(ensure_signed(origin)?);
            let collection = Self::get_collection(collection_id)?;

            Self::create_item_internal(&sender, &collection, &owner, data);

            Self::submit_logs(collection)?;
            Ok(())
        }

        /// This method creates multiple items in a collection created with CreateCollection method.
        /// 
        /// # Permissions
        /// 
        /// * Collection Owner.
        /// * Collection Admin.
        /// * Anyone if
        ///     * White List is enabled, and
        ///     * Address is added to white list, and
        ///     * MintPermission is enabled (see SetMintPermission method)
        /// 
        /// # Arguments
        /// 
        /// * collection_id: ID of the collection.
        /// 
        /// * itemsData: Array items properties. Each property is an array of bytes itself, see [create_item].
        /// 
        /// * owner: Address, initial owner of the NFT.
        #[weight = <T as Config>::WeightInfo::create_item(items_data.into_iter()
                               .map(|data| { data.len() })
                               .sum())]
        #[transactional]
        pub fn create_multiple_items(origin, collection_id: CollectionId, owner: T::CrossAccountId, items_data: Vec<CreateItemData>) -> DispatchResult {

            ensure!(items_data.len() > 0, Error::<T>::EmptyArgument);
            let sender = T::CrossAccountId::from_sub(ensure_signed(origin)?);
            let collection = Self::get_collection(collection_id)?;

            Self::create_multiple_items_internal(&sender, &collection, &owner, items_data)?;

            Self::submit_logs(collection)?;
            Ok(())
        }

        /// Destroys a concrete instance of NFT.
        /// 
        /// # Permissions
        /// 
        /// * Collection Owner.
        /// * Collection Admin.
        /// * Current NFT Owner.
        /// 
        /// # Arguments
        /// 
        /// * collection_id: ID of the collection.
        /// 
        /// * item_id: ID of NFT to burn.
        #[weight = <T as Config>::WeightInfo::burn_item()]
        #[transactional]
        pub fn burn_item(origin, collection_id: CollectionId, item_id: TokenId, value: u128) -> DispatchResult {

            let sender = T::CrossAccountId::from_sub(ensure_signed(origin)?);
            let target_collection = Self::get_collection(collection_id)?;

            Self::burn_item_internal(&sender, &target_collection, item_id, value)?;

            Self::submit_logs(target_collection)?;
            Ok(())
        }

        /// Change ownership of the token.
        /// 
        /// # Permissions
        /// 
        /// * Collection Owner
        /// * Collection Admin
        /// * Current NFT owner
        ///
        /// # Arguments
        /// 
        /// * recipient: Address of token recipient.
        /// 
        /// * collection_id.
        /// 
        /// * item_id: ID of the item
        ///     * Non-Fungible Mode: Required.
        ///     * Fungible Mode: Ignored.
        ///     * Re-Fungible Mode: Required.
        /// 
        /// * value: Amount to transfer.
        ///     * Non-Fungible Mode: Ignored
        ///     * Fungible Mode: Must specify transferred amount
        ///     * Re-Fungible Mode: Must specify transferred portion (between 0 and 1)
        #[weight = <T as Config>::WeightInfo::transfer()]
        #[transactional]
        pub fn transfer(origin, recipient: T::CrossAccountId, collection_id: CollectionId, item_id: TokenId, value: u128) -> DispatchResult {
            let sender = T::CrossAccountId::from_sub(ensure_signed(origin)?);
            let collection = Self::get_collection(collection_id)?;

            Self::transfer_internal(&sender, &recipient, &collection, item_id, value)?;

            Self::submit_logs(collection)?;
            Ok(())
        }

        /// Set, change, or remove approved address to transfer the ownership of the NFT.
        /// 
        /// # Permissions
        /// 
        /// * Collection Owner
        /// * Collection Admin
        /// * Current NFT owner
        /// 
        /// # Arguments
        /// 
        /// * approved: Address that is approved to transfer this NFT or zero (if needed to remove approval).
        /// 
        /// * collection_id.
        /// 
        /// * item_id: ID of the item.
        #[weight = <T as Config>::WeightInfo::approve()]
        #[transactional]
        pub fn approve(origin, spender: T::CrossAccountId, collection_id: CollectionId, item_id: TokenId, amount: u128) -> DispatchResult {
            let sender = T::CrossAccountId::from_sub(ensure_signed(origin)?);
            let collection = Self::get_collection(collection_id)?;

            Self::approve_internal(&sender, &spender, &collection, item_id, amount)?;

            Self::submit_logs(collection)?;
            Ok(())
        }
        
        /// Change ownership of a NFT on behalf of the owner. See Approve method for additional information. After this method executes, the approval is removed so that the approved address will not be able to transfer this NFT again from this owner.
        /// 
        /// # Permissions
        /// * Collection Owner
        /// * Collection Admin
        /// * Current NFT owner
        /// * Address approved by current NFT owner
        /// 
        /// # Arguments
        /// 
        /// * from: Address that owns token.
        /// 
        /// * recipient: Address of token recipient.
        /// 
        /// * collection_id.
        /// 
        /// * item_id: ID of the item.
        /// 
        /// * value: Amount to transfer.
        #[weight = <T as Config>::WeightInfo::transfer_from()]
        #[transactional]
        pub fn transfer_from(origin, from: T::CrossAccountId, recipient: T::CrossAccountId, collection_id: CollectionId, item_id: TokenId, value: u128 ) -> DispatchResult {
            let sender = T::CrossAccountId::from_sub(ensure_signed(origin)?);
            let collection = Self::get_collection(collection_id)?;

            Self::transfer_from_internal(&sender, &from, &recipient, &collection, item_id, value)?;

            Self::submit_logs(collection)?;
            Ok(())
        }
        // #[weight = 0]
        //     // let no_perm_mes = "You do not have permissions to modify this collection";
        //     // ensure!(<ApprovedList<T>>::contains_key((collection_id, item_id)), no_perm_mes);
        //     // let list_itm = <ApprovedList<T>>::get((collection_id, item_id));
        //     // ensure!(list_itm.contains(&new_owner.clone()), no_perm_mes);

        //     // // on_nft_received  call

        //     // Self::transfer(origin, collection_id, item_id, new_owner)?;

        //     Ok(())
        // }

        /// Set off-chain data schema.
        /// 
        /// # Permissions
        /// 
        /// * Collection Owner
        /// * Collection Admin
        /// 
        /// # Arguments
        /// 
        /// * collection_id.
        /// 
        /// * schema: String representing the offchain data schema.
        #[weight = <T as Config>::WeightInfo::set_variable_meta_data()]
        #[transactional]
        pub fn set_variable_meta_data (
            origin,
            collection_id: CollectionId,
            item_id: TokenId,
            data: Vec<u8>
        ) -> DispatchResult {
            let sender = T::CrossAccountId::from_sub(ensure_signed(origin)?);
            
            let collection = Self::get_collection(collection_id)?;

            Self::set_variable_meta_data_internal(&sender, &collection, item_id, data)?;

            Ok(())
        }
 
        /// Set schema standard
        /// ImageURL
        /// Unique
        /// 
        /// # Permissions
        /// 
        /// * Collection Owner
        /// * Collection Admin
        /// 
        /// # Arguments
        /// 
        /// * collection_id.
        /// 
        /// * schema: SchemaVersion: enum
        #[weight = <T as Config>::WeightInfo::set_schema_version()]
        #[transactional]
        pub fn set_schema_version(
            origin,
            collection_id: CollectionId,
            version: SchemaVersion
        ) -> DispatchResult {
            let sender = T::CrossAccountId::from_sub(ensure_signed(origin)?);
            let mut target_collection = Self::get_collection(collection_id)?;
            Self::check_owner_or_admin_permissions(&target_collection, &sender)?;
            target_collection.schema_version = version;
            Self::save_collection(target_collection);

            Ok(())
        }

        /// Set off-chain data schema.
        /// 
        /// # Permissions
        /// 
        /// * Collection Owner
        /// * Collection Admin
        /// 
        /// # Arguments
        /// 
        /// * collection_id.
        /// 
        /// * schema: String representing the offchain data schema.
        #[weight = <T as Config>::WeightInfo::set_offchain_schema()]
        #[transactional]
        pub fn set_offchain_schema(
            origin,
            collection_id: CollectionId,
            schema: Vec<u8>
        ) -> DispatchResult {
            let sender = T::CrossAccountId::from_sub(ensure_signed(origin)?);
            let mut target_collection = Self::get_collection(collection_id)?;
            Self::check_owner_or_admin_permissions(&target_collection, &sender)?;

            // check schema limit
            ensure!(schema.len() as u32 <= ChainLimit::get().offchain_schema_limit, "");

            target_collection.offchain_schema = schema;
            Self::save_collection(target_collection);

            Ok(())
        }

        /// Set const on-chain data schema.
        /// 
        /// # Permissions
        /// 
        /// * Collection Owner
        /// * Collection Admin
        /// 
        /// # Arguments
        /// 
        /// * collection_id.
        /// 
        /// * schema: String representing the const on-chain data schema.
        #[weight = <T as Config>::WeightInfo::set_const_on_chain_schema()]
        #[transactional]
        pub fn set_const_on_chain_schema (
            origin,
            collection_id: CollectionId,
            schema: Vec<u8>
        ) -> DispatchResult {
            let sender = T::CrossAccountId::from_sub(ensure_signed(origin)?);
            let mut target_collection = Self::get_collection(collection_id)?;
            Self::check_owner_or_admin_permissions(&target_collection, &sender)?;

            // check schema limit
            ensure!(schema.len() as u32 <= ChainLimit::get().const_on_chain_schema_limit, "");

            target_collection.const_on_chain_schema = schema;
            Self::save_collection(target_collection);

            Ok(())
        }

        /// Set variable on-chain data schema.
        /// 
        /// # Permissions
        /// 
        /// * Collection Owner
        /// * Collection Admin
        /// 
        /// # Arguments
        /// 
        /// * collection_id.
        /// 
        /// * schema: String representing the variable on-chain data schema.
        #[weight = <T as Config>::WeightInfo::set_const_on_chain_schema()]
        #[transactional]
        pub fn set_variable_on_chain_schema (
            origin,
            collection_id: CollectionId,
            schema: Vec<u8>
        ) -> DispatchResult {
            let sender = T::CrossAccountId::from_sub(ensure_signed(origin)?);
            let mut target_collection = Self::get_collection(collection_id)?;
            Self::check_owner_or_admin_permissions(&target_collection, &sender)?;

            // check schema limit
            ensure!(schema.len() as u32 <= ChainLimit::get().variable_on_chain_schema_limit, "");

            target_collection.variable_on_chain_schema = schema;
            Self::save_collection(target_collection);

            Ok(())
        }

        // Sudo permissions function
        #[weight = <T as Config>::WeightInfo::set_chain_limits()]
        #[transactional]
        pub fn set_chain_limits(
            origin,
            limits: ChainLimits
        ) -> DispatchResult {

            #[cfg(not(feature = "runtime-benchmarks"))]
            ensure_root(origin)?;

            <ChainLimit>::put(limits);
            Ok(())
        }

        /// Enable smart contract self-sponsoring.
        /// 
        /// # Permissions
        /// 
        /// * Contract Owner
        /// 
        /// # Arguments
        /// 
        /// * contract address
        /// * enable flag
        /// 
        #[weight = <T as Config>::WeightInfo::enable_contract_sponsoring()]
        #[transactional]
        pub fn enable_contract_sponsoring(
            origin,
            contract_address: T::AccountId,
            enable: bool
        ) -> DispatchResult {

            let sender = ensure_signed(origin)?;

            #[cfg(feature = "runtime-benchmarks")]
            <ContractOwner<T>>::insert(contract_address.clone(), sender.clone());

            Self::ensure_contract_owned(sender, &contract_address)?;

            <ContractSelfSponsoring<T>>::insert(contract_address, enable);
            Ok(())
        }

        /// Set the rate limit for contract sponsoring to specified number of blocks.
        /// 
        /// If not set (has the default value of 0 blocks), the sponsoring will be disabled. 
        /// If set to the number B (for blocks), the transactions will be sponsored with a rate 
        /// limit of B, i.e. fees for every transaction sent to this smart contract will be paid 
        /// from contract endowment if there are at least B blocks between such transactions. 
        /// Nonetheless, if transactions are sent more frequently, the fees are paid by the sender.
        /// 
        /// # Permissions
        /// 
        /// * Contract Owner
        /// 
        /// # Arguments
        /// 
        /// -`contract_address`: Address of the contract to sponsor
        /// -`rate_limit`: Number of blocks to wait until the next sponsored transaction is allowed
        /// 
        #[weight = <T as Config>::WeightInfo::set_contract_sponsoring_rate_limit()]
        #[transactional]
        pub fn set_contract_sponsoring_rate_limit(
            origin,
            contract_address: T::AccountId,
            rate_limit: T::BlockNumber
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            #[cfg(feature = "runtime-benchmarks")]
            <ContractOwner<T>>::insert(contract_address.clone(), sender.clone());

            Self::ensure_contract_owned(sender, &contract_address)?;
            <ContractSponsoringRateLimit<T>>::insert(contract_address, rate_limit);
            Ok(())
        }

        /// Enable the white list for a contract. Only addresses added to the white list with addToContractWhiteList will be able to call this smart contract.
        /// 
        /// # Permissions
        /// 
        /// * Address that deployed smart contract.
        /// 
        /// # Arguments
        /// 
        /// -`contract_address`: Address of the contract.
        /// 
        /// - `enable`: .  
        #[weight = <T as Config>::WeightInfo::toggle_contract_white_list()]
        #[transactional]
        pub fn toggle_contract_white_list(
            origin,
            contract_address: T::AccountId,
            enable: bool
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            #[cfg(feature = "runtime-benchmarks")]
            <ContractOwner<T>>::insert(contract_address.clone(), sender.clone());

            Self::ensure_contract_owned(sender, &contract_address)?;
            if enable {
                <ContractWhiteListEnabled<T>>::insert(contract_address, true);
            } else {
                <ContractWhiteListEnabled<T>>::remove(contract_address);
            }
            Ok(())
        }
        
        /// Add an address to smart contract white list.
        /// 
        /// # Permissions
        /// 
        /// * Address that deployed smart contract.
        /// 
        /// # Arguments
        /// 
        /// -`contract_address`: Address of the contract.
        ///
        /// -`account_address`: Address to add.
        #[weight = <T as Config>::WeightInfo::add_to_contract_white_list()]
        #[transactional]
        pub fn add_to_contract_white_list(
            origin,
            contract_address: T::AccountId,
            account_address: T::AccountId
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            #[cfg(feature = "runtime-benchmarks")]
            <ContractOwner<T>>::insert(contract_address.clone(), sender.clone());
            
            Self::ensure_contract_owned(sender, &contract_address)?;      
            <ContractWhiteList<T>>::insert(contract_address, account_address, true);
            Ok(())
        }

        /// Remove an address from smart contract white list.
        /// 
        /// # Permissions
        /// 
        /// * Address that deployed smart contract.
        /// 
        /// # Arguments
        /// 
        /// -`contract_address`: Address of the contract.
        ///
        /// -`account_address`: Address to remove.
        #[weight = <T as Config>::WeightInfo::remove_from_contract_white_list()]
        #[transactional]
        pub fn remove_from_contract_white_list(
            origin,
            contract_address: T::AccountId,
            account_address: T::AccountId
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            #[cfg(feature = "runtime-benchmarks")]
            <ContractOwner<T>>::insert(contract_address.clone(), sender.clone());

            Self::ensure_contract_owned(sender, &contract_address)?;
            <ContractWhiteList<T>>::remove(contract_address, account_address);
            Ok(())
        }

        #[weight = <T as Config>::WeightInfo::set_collection_limits()]
        #[transactional]
        pub fn set_collection_limits(
            origin,
            collection_id: u32,
            new_limits: CollectionLimits<T::BlockNumber>,
        ) -> DispatchResult {
            let sender = T::CrossAccountId::from_sub(ensure_signed(origin)?);
            let mut target_collection = Self::get_collection(collection_id)?;
            Self::check_owner_permissions(&target_collection, &sender)?;
            let old_limits = &target_collection.limits;
            let chain_limits = ChainLimit::get();

            // collection bounds
            ensure!(new_limits.sponsor_transfer_timeout <= MAX_SPONSOR_TIMEOUT &&
                new_limits.account_token_ownership_limit <= MAX_TOKEN_OWNERSHIP && 
                new_limits.sponsored_data_size <= chain_limits.custom_data_limit,
                Error::<T>::CollectionLimitBoundsExceeded);

            // token_limit   check  prev
            ensure!(old_limits.token_limit >= new_limits.token_limit, Error::<T>::CollectionTokenLimitExceeded);
            ensure!(new_limits.token_limit > 0, Error::<T>::CollectionTokenLimitExceeded);

            ensure!(
                (old_limits.owner_can_transfer || !new_limits.owner_can_transfer) &&
                (old_limits.owner_can_destroy || !new_limits.owner_can_destroy),
                Error::<T>::OwnerPermissionsCantBeReverted,
            );

            target_collection.limits = new_limits;
            Self::save_collection(target_collection);

            Ok(())
        } 
    }
}

impl<T: Config> Module<T> {
    pub fn create_item_internal(sender: &T::CrossAccountId, collection: &CollectionHandle<T>, owner: &T::CrossAccountId, data: CreateItemData) -> DispatchResult {
        Self::can_create_items_in_collection(&collection, &sender, &owner, 1)?;
        Self::validate_create_item_args(&collection, &data)?;
        Self::create_item_no_validation(&collection, owner, data)?;

        Ok(())
    }

    pub fn transfer_internal(sender: &T::CrossAccountId, recipient: &T::CrossAccountId, target_collection: &CollectionHandle<T>, item_id: TokenId, value: u128) -> DispatchResult {
        // Limits check
        Self::is_correct_transfer(target_collection, &recipient)?;

        // Transfer permissions check
        ensure!(Self::is_item_owner(&sender, target_collection, item_id) ||
            Self::is_owner_or_admin_permissions(target_collection, &sender),
            Error::<T>::NoPermission);

        if target_collection.access == AccessMode::WhiteList {
            Self::check_white_list(target_collection, &sender)?;
            Self::check_white_list(target_collection, &recipient)?;
        }

        match target_collection.mode
        {
            CollectionMode::NFT => Self::transfer_nft(target_collection, item_id, sender.clone(), recipient.clone())?,
            CollectionMode::Fungible(_)  => Self::transfer_fungible(target_collection, value, &sender, &recipient)?,
            CollectionMode::ReFungible  => Self::transfer_refungible(target_collection, item_id, value, sender.clone(), recipient.clone())?,
            _ => ()
        };

        Self::deposit_event(RawEvent::Transfer(target_collection.id, item_id, sender.clone(), recipient.clone(), value));

        Ok(())
    }

	pub fn approve_internal(
		sender: &T::CrossAccountId,
		spender: &T::CrossAccountId,
		collection: &CollectionHandle<T>,
		item_id: TokenId,
		amount: u128
	) -> DispatchResult {
		Self::token_exists(&collection, item_id)?;

		// Transfer permissions check
		let bypasses_limits = collection.limits.owner_can_transfer &&
			Self::is_owner_or_admin_permissions(
				&collection,
				&sender,
			);

		let allowance_limit = if bypasses_limits {
			None
		} else if let Some(amount) = Self::owned_amount(
			&sender,
			&collection,
			item_id,
		) {
			Some(amount)
		} else {
			fail!(Error::<T>::NoPermission);
		};

		if collection.access == AccessMode::WhiteList {
			Self::check_white_list(&collection, &sender)?;
			Self::check_white_list(&collection, &spender)?;
		}

		let allowance: u128 = amount
			.checked_add(<Allowances<T>>::get(collection.id, (item_id, sender.as_sub(), spender.as_sub())))
			.ok_or(Error::<T>::NumOverflow)?;
		if let Some(limit) = allowance_limit {
			ensure!(limit >= allowance, Error::<T>::TokenValueTooLow);
		}
		<Allowances<T>>::insert(collection.id, (item_id, sender.as_sub(), spender.as_sub()), allowance);

		if matches!(collection.mode, CollectionMode::NFT) {
			// TODO: NFT: only one owner may exist for token in ERC721
			collection.log(
				Vec::from([
					eth::APPROVAL_NFT_TOPIC,
					eth::address_to_topic(sender.as_eth()),
					eth::address_to_topic(spender.as_eth()),
				]),
				abi_encode!(uint256(item_id.into())),
			);
		}

		if matches!(collection.mode, CollectionMode::Fungible(_)) {
			// TODO: NFT: only one owner may exist for token in ERC20
			collection.log(
				Vec::from([
					eth::APPROVAL_FUNGIBLE_TOPIC,
					eth::address_to_topic(sender.as_eth()),
					eth::address_to_topic(spender.as_eth()),
				]),
				abi_encode!(uint256(allowance.into())),
			);
		}

		Self::deposit_event(RawEvent::Approved(collection.id, item_id, sender.clone(), spender.clone(), allowance));
		Ok(())
	}

	pub fn transfer_from_internal(
		sender: &T::CrossAccountId,
		from: &T::CrossAccountId,
		recipient: &T::CrossAccountId,
		collection: &CollectionHandle<T>,
		item_id: TokenId,
		amount: u128,
	) -> DispatchResult {
		// Check approval
		let approval: u128 = <Allowances<T>>::get(collection.id, (item_id, from.as_sub(), sender.as_sub()));

		// Limits check
		Self::is_correct_transfer(&collection, &recipient)?;

		// Transfer permissions check
		ensure!(
			approval >= amount || 
			(
				collection.limits.owner_can_transfer &&
				Self::is_owner_or_admin_permissions(&collection, &sender)
			),
			Error::<T>::NoPermission
		);

		if collection.access == AccessMode::WhiteList {
			Self::check_white_list(&collection, &sender)?;
			Self::check_white_list(&collection, &recipient)?;
		}

		// Reduce approval by transferred amount or remove if remaining approval drops to 0
		let allowance = approval.saturating_sub(amount);
		if allowance > 0 {
			<Allowances<T>>::insert(collection.id, (item_id, from.as_sub(), sender.as_sub()), allowance);
		} else {
			<Allowances<T>>::remove(collection.id, (item_id, from.as_sub(), sender.as_sub()));
		}

		match collection.mode {
			CollectionMode::NFT => {
				Self::transfer_nft(&collection, item_id, from.clone(), recipient.clone())?
			}
			CollectionMode::Fungible(_) => {
				Self::transfer_fungible(&collection, amount, &from, &recipient)?
			}
			CollectionMode::ReFungible => {
				Self::transfer_refungible(&collection, item_id, amount, from.clone(), recipient.clone())?
			}
			_ => ()
		};

		if matches!(collection.mode, CollectionMode::Fungible(_)) {
			collection.log(
				Vec::from([
					eth::APPROVAL_FUNGIBLE_TOPIC,
					eth::address_to_topic(from.as_eth()),
					eth::address_to_topic(sender.as_eth()),
				]),
				abi_encode!(uint256(allowance.into())),
			);
		}

		Ok(())
	}

    pub fn set_variable_meta_data_internal(
        sender: &T::CrossAccountId,
        collection: &CollectionHandle<T>, 
        item_id: TokenId,
        data: Vec<u8>,
    ) -> DispatchResult {
        Self::token_exists(&collection, item_id)?;

        ensure!(ChainLimit::get().custom_data_limit >= data.len() as u32, Error::<T>::TokenVariableDataLimitExceeded);

        // Modify permissions check
        ensure!(Self::is_item_owner(&sender, &collection, item_id) ||
            Self::is_owner_or_admin_permissions(&collection, &sender),
            Error::<T>::NoPermission);

        match collection.mode
        {
            CollectionMode::NFT => Self::set_nft_variable_data(&collection, item_id, data)?,
            CollectionMode::ReFungible  => Self::set_re_fungible_variable_data(&collection, item_id, data)?,
            CollectionMode::Fungible(_) => fail!(Error::<T>::CantStoreMetadataInFungibleTokens),
            _ => fail!(Error::<T>::UnexpectedCollectionType)
        };

        Ok(())
    }

    pub fn create_multiple_items_internal(
        sender: &T::CrossAccountId,
        collection: &CollectionHandle<T>,
        owner: &T::CrossAccountId,
        items_data: Vec<CreateItemData>,
    ) -> DispatchResult {
        Self::can_create_items_in_collection(&collection, &sender, &owner, items_data.len() as u32)?;

        for data in &items_data {
            Self::validate_create_item_args(&collection, data)?;
        }
        for data in &items_data {
            Self::create_item_no_validation(&collection, owner, data.clone())?;
        }

        Ok(())
    }

    pub fn burn_item_internal(
        sender: &T::CrossAccountId,
        collection: &CollectionHandle<T>,
        item_id: TokenId,
        value: u128,
    ) -> DispatchResult {
        ensure!(
            Self::is_item_owner(&sender, &collection, item_id) ||
            (
                collection.limits.owner_can_transfer &&
                Self::is_owner_or_admin_permissions(&collection, &sender)
            ),
            Error::<T>::NoPermission
        );

        if collection.access == AccessMode::WhiteList {
            Self::check_white_list(&collection, &sender)?;
        }

        match collection.mode
        {
            CollectionMode::NFT => Self::burn_nft_item(&collection, item_id)?,
            CollectionMode::Fungible(_)  => Self::burn_fungible_item(&sender, &collection, value)?,
            CollectionMode::ReFungible  => Self::burn_refungible_item(&collection, item_id, &sender)?,
            _ => ()
        };

        Ok(())
    }

    pub fn toggle_white_list_internal(
        sender: &T::CrossAccountId,
        collection: &CollectionHandle<T>,
        address: &T::CrossAccountId,
        whitelisted: bool,
    ) -> DispatchResult {
        Self::check_owner_or_admin_permissions(&collection, &sender)?;

        if whitelisted {
            <WhiteList<T>>::insert(collection.id, address.as_sub(), true);
        } else {
            <WhiteList<T>>::remove(collection.id, address.as_sub());
        }

        Ok(())
    }

    fn is_correct_transfer(collection: &CollectionHandle<T>, recipient: &T::CrossAccountId) -> DispatchResult {
        let collection_id = collection.id;

        // check token limit and account token limit
        let account_items: u32 = <AddressTokens<T>>::get(collection_id, recipient.as_sub()).len() as u32;
        ensure!(collection.limits.account_token_ownership_limit > account_items,  Error::<T>::AccountTokenLimitExceeded);
        
        Ok(())
    }

    fn can_create_items_in_collection(collection: &CollectionHandle<T>, sender: &T::CrossAccountId, owner: &T::CrossAccountId, amount: u32) -> DispatchResult {
        let collection_id = collection.id;

        // check token limit and account token limit
        let total_items: u32 = ItemListIndex::get(collection_id)
            .checked_add(amount)
            .ok_or(Error::<T>::CollectionTokenLimitExceeded)?;
        let account_items: u32 = (<AddressTokens<T>>::get(collection_id, owner.as_sub()).len() as u32)
            .checked_add(amount)
            .ok_or(Error::<T>::AccountTokenLimitExceeded)?;
        ensure!(collection.limits.token_limit >= total_items,  Error::<T>::CollectionTokenLimitExceeded);
        ensure!(collection.limits.account_token_ownership_limit >= account_items,  Error::<T>::AccountTokenLimitExceeded);

        if !Self::is_owner_or_admin_permissions(collection, &sender) {
            ensure!(collection.mint_mode == true, Error::<T>::PublicMintingNotAllowed);
            Self::check_white_list(collection, owner)?;
            Self::check_white_list(collection, sender)?;
        }

        Ok(())
    }

    fn validate_create_item_args(target_collection: &CollectionHandle<T>, data: &CreateItemData) -> DispatchResult {
        match target_collection.mode
        {
            CollectionMode::NFT => {
                if let CreateItemData::NFT(data) = data {
                    // check sizes
                    ensure!(ChainLimit::get().custom_data_limit >= data.const_data.len() as u32, Error::<T>::TokenConstDataLimitExceeded);
                    ensure!(ChainLimit::get().custom_data_limit >= data.variable_data.len() as u32, Error::<T>::TokenVariableDataLimitExceeded);
                } else {
                    fail!(Error::<T>::NotNftDataUsedToMintNftCollectionToken);
                }
            },
            CollectionMode::Fungible(_) => {
                if let CreateItemData::Fungible(_) = data {
                } else {
                    fail!(Error::<T>::NotFungibleDataUsedToMintFungibleCollectionToken);
                }
            },
            CollectionMode::ReFungible => {
                if let CreateItemData::ReFungible(data) = data {

                    // check sizes
                    ensure!(ChainLimit::get().custom_data_limit >= data.const_data.len() as u32, Error::<T>::TokenConstDataLimitExceeded);
                    ensure!(ChainLimit::get().custom_data_limit >= data.variable_data.len() as u32, Error::<T>::TokenVariableDataLimitExceeded);

                    // Check refungibility limits
                    ensure!(data.pieces <= MAX_REFUNGIBLE_PIECES, Error::<T>::WrongRefungiblePieces);
                    ensure!(data.pieces > 0, Error::<T>::WrongRefungiblePieces);
                } else {
                    fail!(Error::<T>::NotReFungibleDataUsedToMintReFungibleCollectionToken);
                }
            },
            _ => { fail!(Error::<T>::UnexpectedCollectionType); }
        };

        Ok(())
    }

    fn create_item_no_validation(collection: &CollectionHandle<T>, owner: &T::CrossAccountId, data: CreateItemData) -> DispatchResult {
        match data
        {
            CreateItemData::NFT(data) => {
                let item = NftItemType {
                    owner: owner.clone(),
                    const_data: data.const_data,
                    variable_data: data.variable_data
                };

                Self::add_nft_item(collection, item)?;
            },
            CreateItemData::Fungible(data) => {
                Self::add_fungible_item(collection, &owner, data.value)?;
            },
            CreateItemData::ReFungible(data) => {
                let mut owner_list = Vec::new();
                owner_list.push(Ownership {owner: owner.clone(), fraction: data.pieces});

                let item = ReFungibleItemType {
                    owner: owner_list,
                    const_data: data.const_data,
                    variable_data: data.variable_data
                };

                Self::add_refungible_item(collection, item)?;
            }
        };

        Ok(())
    }

    fn add_fungible_item(collection: &CollectionHandle<T>, owner: &T::CrossAccountId, value: u128) -> DispatchResult {
        let collection_id = collection.id;

        // Does new owner already have an account?
        let balance: u128 = <FungibleItemList<T>>::get(collection_id, owner.as_sub()).value;

        // Mint 
        let item = FungibleItemType {
            value: balance.checked_add(value).ok_or(Error::<T>::NumOverflow)?,
        };
        <FungibleItemList<T>>::insert(collection_id, owner.as_sub(), item);

        // Update balance
        let new_balance = <Balance<T>>::get(collection_id, owner.as_sub())
            .checked_add(value)
            .ok_or(Error::<T>::NumOverflow)?;
        <Balance<T>>::insert(collection_id, owner.as_sub(), new_balance);

        Self::deposit_event(RawEvent::ItemCreated(collection_id, 0, owner.clone()));
        Ok(())
    }

    fn add_refungible_item(collection: &CollectionHandle<T>, item: ReFungibleItemType<T::CrossAccountId>) -> DispatchResult {
        let collection_id = collection.id;

        let current_index = <ItemListIndex>::get(collection_id)
            .checked_add(1)
            .ok_or(Error::<T>::NumOverflow)?;
        let itemcopy = item.clone();

        ensure!(
            item.owner.len() == 1,
            Error::<T>::BadCreateRefungibleCall,
        );
        let item_owner = item.owner.first().expect("only one owner is defined");

        let value = item_owner.fraction;
        let owner = item_owner.owner.clone();

        Self::add_token_index(collection_id, current_index, &owner)?;

        <ItemListIndex>::insert(collection_id, current_index);
        <ReFungibleItemList<T>>::insert(collection_id, current_index, itemcopy);

        // Update balance
        let new_balance = <Balance<T>>::get(collection_id, owner.as_sub())
            .checked_add(value)
            .ok_or(Error::<T>::NumOverflow)?;
        <Balance<T>>::insert(collection_id, owner.as_sub(), new_balance);

        Self::deposit_event(RawEvent::ItemCreated(collection_id, current_index, owner));
        Ok(())
    }

    fn add_nft_item(collection: &CollectionHandle<T>, item: NftItemType<T::CrossAccountId>) -> DispatchResult {
        let collection_id = collection.id;

        let current_index = <ItemListIndex>::get(collection_id)
            .checked_add(1)
            .ok_or(Error::<T>::NumOverflow)?;

        let item_owner = item.owner.clone();
        Self::add_token_index(collection_id, current_index, &item.owner)?;

        <ItemListIndex>::insert(collection_id, current_index);
        <NftItemList<T>>::insert(collection_id, current_index, item);

        // Update balance
        let new_balance = <Balance<T>>::get(collection_id, item_owner.as_sub())
            .checked_add(1)
            .ok_or(Error::<T>::NumOverflow)?;
        <Balance<T>>::insert(collection_id, item_owner.as_sub(), new_balance);

        collection.log(
            Vec::from([
                eth::TRANSFER_NFT_TOPIC,
                eth::address_to_topic(&H160::default()),
                eth::address_to_topic(item_owner.as_eth()),
            ]),
            abi_encode!(uint256(current_index.into())),
        );
        Self::deposit_event(RawEvent::ItemCreated(collection_id, current_index, item_owner));
        Ok(())
    }

    fn burn_refungible_item(
        collection: &CollectionHandle<T>,
        item_id: TokenId,
        owner: &T::CrossAccountId,
    ) -> DispatchResult {
        let collection_id = collection.id;

        let mut token = <ReFungibleItemList<T>>::get(collection_id, item_id)
            .ok_or(Error::<T>::TokenNotFound)?;
        let rft_balance = token
            .owner
            .iter()
            .find(|&i| i.owner == *owner)
            .ok_or(Error::<T>::TokenNotFound)?;
        Self::remove_token_index(collection_id, item_id, owner)?;

        // update balance
        let new_balance = <Balance<T>>::get(collection_id, rft_balance.owner.as_sub())
            .checked_sub(rft_balance.fraction)
            .ok_or(Error::<T>::NumOverflow)?;
        <Balance<T>>::insert(collection_id, rft_balance.owner.as_sub(), new_balance);

        // Re-create owners list with sender removed
        let index = token
            .owner
            .iter()
            .position(|i| i.owner == *owner)
            .expect("owned item is exists");
        token.owner.remove(index);
        let owner_count = token.owner.len();

        // Burn the token completely if this was the last (only) owner
        if owner_count == 0 {
            <ReFungibleItemList<T>>::remove(collection_id, item_id);
            <VariableMetaDataBasket<T>>::remove(collection_id, item_id);
        }
        else {
            <ReFungibleItemList<T>>::insert(collection_id, item_id, token);
        }

        Ok(())
    }

    fn burn_nft_item(collection: &CollectionHandle<T>, item_id: TokenId) -> DispatchResult {
        let collection_id = collection.id;

        let item = <NftItemList<T>>::get(collection_id, item_id)
            .ok_or(Error::<T>::TokenNotFound)?;
        Self::remove_token_index(collection_id, item_id, &item.owner)?;

        // update balance
        let new_balance = <Balance<T>>::get(collection_id, item.owner.as_sub())
            .checked_sub(1)
            .ok_or(Error::<T>::NumOverflow)?;
        <Balance<T>>::insert(collection_id, item.owner.as_sub(), new_balance);
        <NftItemList<T>>::remove(collection_id, item_id);
        <VariableMetaDataBasket<T>>::remove(collection_id, item_id);

        Self::deposit_event(RawEvent::ItemDestroyed(collection.id, item_id));
        Ok(())
    }

    fn burn_fungible_item(owner: &T::CrossAccountId, collection: &CollectionHandle<T>, value: u128) -> DispatchResult {
        let collection_id = collection.id;

        let mut balance = <FungibleItemList<T>>::get(collection_id, owner.as_sub());
        ensure!(balance.value >= value, Error::<T>::TokenValueNotEnough);

        // update balance
        let new_balance = <Balance<T>>::get(collection_id, owner.as_sub())
            .checked_sub(value)
            .ok_or(Error::<T>::NumOverflow)?;
        <Balance<T>>::insert(collection_id, owner.as_sub(), new_balance);

        if balance.value - value > 0 {
            balance.value -= value;
            <FungibleItemList<T>>::insert(collection_id, owner.as_sub(), balance);
        }
        else {
            <FungibleItemList<T>>::remove(collection_id, owner.as_sub());
        }

        collection.log(
            Vec::from([
                eth::TRANSFER_FUNGIBLE_TOPIC,
                eth::address_to_topic(owner.as_eth()),
                eth::address_to_topic(&H160::default()),
            ]),
            abi_encode!(uint256(value.into())),
        );
        Ok(())
    }

    pub fn get_collection(collection_id: CollectionId) -> Result<CollectionHandle<T>, sp_runtime::DispatchError> {
        Ok(<CollectionHandle<T>>::get(collection_id)
            .ok_or(Error::<T>::CollectionNotFound)?)
    }

    fn save_collection(collection: CollectionHandle<T>) {
        <CollectionById<T>>::insert(collection.id, collection.into_inner());
    }

    fn submit_logs(collection: CollectionHandle<T>) -> DispatchResult {
        if collection.logs.is_empty() {
            return Ok(())
        }
        T::EthereumTransactionSender::submit_logs_transaction(
            eth::generate_transaction(collection.id, T::EthereumChainId::get()),
            collection.logs.retrieve_logs_for_contract(eth::collection_id_to_address(collection.id)),
        )
    }

    fn check_owner_permissions(target_collection: &CollectionHandle<T>, subject: &T::CrossAccountId) -> DispatchResult {
        ensure!(
            *subject == target_collection.owner,
            Error::<T>::NoPermission
        );

        Ok(())
    }

    fn is_owner_or_admin_permissions(collection: &CollectionHandle<T>, subject: &T::CrossAccountId) -> bool {
        *subject == collection.owner || <AdminList<T>>::get(collection.id).contains(&subject)
    }

    fn check_owner_or_admin_permissions(
        collection: &CollectionHandle<T>,
        subject: &T::CrossAccountId,
    ) -> DispatchResult {
        ensure!(Self::is_owner_or_admin_permissions(collection, subject), Error::<T>::NoPermission);

        Ok(())
    }

    fn owned_amount(
        subject: &T::CrossAccountId,
        target_collection: &CollectionHandle<T>,
        item_id: TokenId,
    ) -> Option<u128> {
        let collection_id = target_collection.id;

        match target_collection.mode {
            CollectionMode::NFT => (<NftItemList<T>>::get(collection_id, item_id)?.owner == *subject)
                .then(|| 1),
            CollectionMode::Fungible(_) => Some(<FungibleItemList<T>>::get(collection_id, &subject.as_sub())
                .value),
            CollectionMode::ReFungible => <ReFungibleItemList<T>>::get(collection_id, item_id)?
                .owner
                .iter()
                .find(|i| i.owner == *subject)
                .map(|i| i.fraction),
            CollectionMode::Invalid => None,
        }
    }

    fn is_item_owner(subject: &T::CrossAccountId, target_collection: &CollectionHandle<T>, item_id: TokenId) -> bool {
        match target_collection.mode {
            CollectionMode::Fungible(_) => true,
            _ => Self::owned_amount(&subject, target_collection, item_id).is_some(),
        }
    }

    fn check_white_list(collection: &CollectionHandle<T>, address: &T::CrossAccountId) -> DispatchResult {
        let collection_id = collection.id;

        let mes = Error::<T>::AddresNotInWhiteList;
        ensure!(<WhiteList<T>>::contains_key(collection_id, address.as_sub()), mes);

        Ok(())
    }

    /// Check if token exists. In case of Fungible, check if there is an entry for 
    /// the owner in fungible balances double map
    fn token_exists(
        target_collection: &CollectionHandle<T>,
        item_id: TokenId,
    ) -> DispatchResult {
        let collection_id = target_collection.id;
        let exists = match target_collection.mode
        {
            CollectionMode::NFT => <NftItemList<T>>::contains_key(collection_id, item_id),
            CollectionMode::Fungible(_)  => true,
            CollectionMode::ReFungible  => <ReFungibleItemList<T>>::contains_key(collection_id, item_id),
            _ => false
        };

        ensure!(exists == true, Error::<T>::TokenNotFound);
        Ok(())
    }

    fn transfer_fungible(
        collection: &CollectionHandle<T>,
        value: u128,
        owner: &T::CrossAccountId,
        recipient: &T::CrossAccountId,
    ) -> DispatchResult {
        let collection_id = collection.id;

        let mut balance = <FungibleItemList<T>>::get(collection_id, owner.as_sub());
        ensure!(balance.value >= value, Error::<T>::TokenValueTooLow);

        // Send balance to recipient (updates balanceOf of recipient)
        Self::add_fungible_item(collection, recipient, value)?;

        // update balanceOf of sender
        <Balance<T>>::insert(collection_id, owner.as_sub(), balance.value - value);

        // Reduce or remove sender
        if balance.value == value {
            <FungibleItemList<T>>::remove(collection_id, owner.as_sub());
        }
        else {
            balance.value -= value;
            <FungibleItemList<T>>::insert(collection_id, owner.as_sub(), balance);
        }

        collection.log(
            Vec::from([
                eth::TRANSFER_FUNGIBLE_TOPIC,
                eth::address_to_topic(owner.as_eth()),
                eth::address_to_topic(recipient.as_eth()),
            ]),
            abi_encode!(uint256(value.into())),
        );
        Self::deposit_event(RawEvent::Transfer(collection.id, 1, owner.clone(), recipient.clone(), value));

        Ok(())
    }

    fn transfer_refungible(
        collection: &CollectionHandle<T>,
        item_id: TokenId,
        value: u128,
        owner: T::CrossAccountId,
        new_owner: T::CrossAccountId,
    ) -> DispatchResult {
        let collection_id = collection.id;
        let full_item = <ReFungibleItemList<T>>::get(collection_id, item_id)
            .ok_or(Error::<T>::TokenNotFound)?;

        let item = full_item
            .owner
            .iter()
            .filter(|i| i.owner == owner)
            .next()
            .ok_or(Error::<T>::TokenNotFound)?;
        let amount = item.fraction;

        ensure!(amount >= value, Error::<T>::TokenValueTooLow);

        // update balance
        let balance_old_owner = <Balance<T>>::get(collection_id, item.owner.as_sub())
            .checked_sub(value)
            .ok_or(Error::<T>::NumOverflow)?;
        <Balance<T>>::insert(collection_id, item.owner.as_sub(), balance_old_owner);

        let balance_new_owner = <Balance<T>>::get(collection_id, new_owner.as_sub())
            .checked_add(value)
            .ok_or(Error::<T>::NumOverflow)?;
        <Balance<T>>::insert(collection_id, new_owner.as_sub(), balance_new_owner);

        let old_owner = item.owner.clone();
        let new_owner_has_account = full_item.owner.iter().any(|i| i.owner == new_owner);

        // transfer
        if amount == value && !new_owner_has_account {
            // change owner
            // new owner do not have account
            let mut new_full_item = full_item.clone();
            new_full_item
                .owner
                .iter_mut()
                .find(|i| i.owner == owner)
                .expect("old owner does present in refungible")
                .owner = new_owner.clone();
            <ReFungibleItemList<T>>::insert(collection_id, item_id, new_full_item);

            // update index collection
            Self::move_token_index(collection_id, item_id, &old_owner, &new_owner)?;
        } else {
            let mut new_full_item = full_item.clone();
            new_full_item
                .owner
                .iter_mut()
                .find(|i| i.owner == owner)
                .expect("old owner does present in refungible")
                .fraction -= value;

            // separate amount
            if new_owner_has_account {
                // new owner has account
                new_full_item
                    .owner
                    .iter_mut()
                    .find(|i| i.owner == new_owner)
                    .expect("new owner has account")
                    .fraction += value;
            } else {
                // new owner do not have account
                new_full_item.owner.push(Ownership {
                    owner: new_owner.clone(),
                    fraction: value,
                });
                Self::add_token_index(collection_id, item_id, &new_owner)?;
            }

            <ReFungibleItemList<T>>::insert(collection_id, item_id, new_full_item);
        }

        Self::deposit_event(RawEvent::Transfer(collection.id, item_id, owner, new_owner, amount));

        Ok(())
    }

    fn transfer_nft(
        collection: &CollectionHandle<T>,
        item_id: TokenId,
        sender: T::CrossAccountId,
        new_owner: T::CrossAccountId,
    ) -> DispatchResult {
        let collection_id = collection.id;
        let mut item = <NftItemList<T>>::get(collection_id, item_id)
            .ok_or(Error::<T>::TokenNotFound)?;

        ensure!(
            sender == item.owner,
            Error::<T>::MustBeTokenOwner
        );

        // update balance
        let balance_old_owner = <Balance<T>>::get(collection_id, item.owner.as_sub())
            .checked_sub(1)
            .ok_or(Error::<T>::NumOverflow)?;
        <Balance<T>>::insert(collection_id, item.owner.as_sub(), balance_old_owner);

        let balance_new_owner = <Balance<T>>::get(collection_id, new_owner.as_sub())
            .checked_add(1)
            .ok_or(Error::<T>::NumOverflow)?;
        <Balance<T>>::insert(collection_id, new_owner.as_sub(), balance_new_owner);

        // change owner
        let old_owner = item.owner.clone();
        item.owner = new_owner.clone();
        <NftItemList<T>>::insert(collection_id, item_id, item);

        // update index collection
        Self::move_token_index(collection_id, item_id, &old_owner, &new_owner)?;

        collection.log(
            Vec::from([
                eth::TRANSFER_NFT_TOPIC,
                eth::address_to_topic(sender.as_eth()),
                eth::address_to_topic(new_owner.as_eth()),
            ]),
            abi_encode!(uint256(item_id.into())),
        );
        Self::deposit_event(RawEvent::Transfer(collection.id, item_id, sender, new_owner, 1));

        Ok(())
    }
    
    fn set_re_fungible_variable_data(
        collection: &CollectionHandle<T>,
        item_id: TokenId,
        data: Vec<u8>
    ) -> DispatchResult {
        let collection_id = collection.id;
        let mut item = <ReFungibleItemList<T>>::get(collection_id, item_id)
            .ok_or(Error::<T>::TokenNotFound)?;

        item.variable_data = data;

        <ReFungibleItemList<T>>::insert(collection_id, item_id, item);

        Ok(())
    }

    fn set_nft_variable_data(
        collection: &CollectionHandle<T>,
        item_id: TokenId,
        data: Vec<u8>
    ) -> DispatchResult {
        let collection_id = collection.id;
        let mut item = <NftItemList<T>>::get(collection_id, item_id)
            .ok_or(Error::<T>::TokenNotFound)?;
        
        item.variable_data = data;

        <NftItemList<T>>::insert(collection_id, item_id, item);
        
        Ok(())
    }

    fn init_collection(item: &Collection<T>) {
        // check params
        assert!(
            item.decimal_points <= MAX_DECIMAL_POINTS,
            "decimal_points parameter must be lower than MAX_DECIMAL_POINTS"
        );
        assert!(
            item.name.len() <= 64,
            "Collection name can not be longer than 63 char"
        );
        assert!(
            item.name.len() <= 256,
            "Collection description can not be longer than 255 char"
        );
        assert!(
            item.token_prefix.len() <= 16,
            "Token prefix can not be longer than 15 char"
        );

        // Generate next collection ID
        let next_id = CreatedCollectionCount::get()
            .checked_add(1)
            .unwrap();

        CreatedCollectionCount::put(next_id);
    }

    fn init_nft_token(collection_id: CollectionId, item: &NftItemType<T::CrossAccountId>) {
        let current_index = <ItemListIndex>::get(collection_id)
            .checked_add(1)
            .unwrap();

        Self::add_token_index(collection_id, current_index, &item.owner).unwrap();

        <ItemListIndex>::insert(collection_id, current_index);

        // Update balance
        let new_balance = <Balance<T>>::get(collection_id, item.owner.as_sub())
            .checked_add(1)
            .unwrap();
        <Balance<T>>::insert(collection_id, item.owner.as_sub(), new_balance);
    }

    fn init_fungible_token(collection_id: CollectionId, owner: &T::CrossAccountId, item: &FungibleItemType) {
        let current_index = <ItemListIndex>::get(collection_id)
            .checked_add(1)
            .unwrap();

        Self::add_token_index(collection_id, current_index, owner).unwrap();

        <ItemListIndex>::insert(collection_id, current_index);

        // Update balance
        let new_balance = <Balance<T>>::get(collection_id, owner.as_sub())
            .checked_add(item.value)
            .unwrap();
        <Balance<T>>::insert(collection_id, owner.as_sub(), new_balance);
    }

    fn init_refungible_token(collection_id: CollectionId, item: &ReFungibleItemType<T::CrossAccountId>) {
        let current_index = <ItemListIndex>::get(collection_id)
            .checked_add(1)
            .unwrap();

        let value = item.owner.first().unwrap().fraction;
        let owner = item.owner.first().unwrap().owner.clone();

        Self::add_token_index(collection_id, current_index, &owner).unwrap();

        <ItemListIndex>::insert(collection_id, current_index);

        // Update balance
        let new_balance = <Balance<T>>::get(collection_id, &owner.as_sub())
            .checked_add(value)
            .unwrap();
        <Balance<T>>::insert(collection_id, owner.as_sub(), new_balance);
    }

    fn add_token_index(collection_id: CollectionId, item_index: TokenId, owner: &T::CrossAccountId) -> DispatchResult {
        // add to account limit
        if <AccountItemCount<T>>::contains_key(owner.as_sub()) {

            // bound Owned tokens by a single address
            let count = <AccountItemCount<T>>::get(owner.as_sub());
            ensure!(count < ChainLimit::get().account_token_ownership_limit, Error::<T>::AddressOwnershipLimitExceeded);

            <AccountItemCount<T>>::insert(owner.as_sub(), count
                .checked_add(1)
                .ok_or(Error::<T>::NumOverflow)?);
        }
        else {
            <AccountItemCount<T>>::insert(owner.as_sub(), 1);
        }

        let list_exists = <AddressTokens<T>>::contains_key(collection_id, owner.as_sub());
        if list_exists {
            let mut list = <AddressTokens<T>>::get(collection_id, owner.as_sub());
            let item_contains = list.contains(&item_index.clone());

            if !item_contains {
                list.push(item_index.clone());
            }

            <AddressTokens<T>>::insert(collection_id, owner.as_sub(), list);
        } else {
            let mut itm = Vec::new();
            itm.push(item_index.clone());
            <AddressTokens<T>>::insert(collection_id, owner.as_sub(), itm);
        }

        Ok(())
    }

    fn remove_token_index(
        collection_id: CollectionId,
        item_index: TokenId,
        owner: &T::CrossAccountId,
    ) -> DispatchResult {

        // update counter
        <AccountItemCount<T>>::insert(owner.as_sub(), 
            <AccountItemCount<T>>::get(owner.as_sub())
            .checked_sub(1)
            .ok_or(Error::<T>::NumOverflow)?);


        let list_exists = <AddressTokens<T>>::contains_key(collection_id, owner.as_sub());
        if list_exists {
            let mut list = <AddressTokens<T>>::get(collection_id, owner.as_sub());
            let item_contains = list.contains(&item_index.clone());

            if item_contains {
                list.retain(|&item| item != item_index);
                <AddressTokens<T>>::insert(collection_id, owner.as_sub(), list);
            }
        }

        Ok(())
    }

    fn move_token_index(
        collection_id: CollectionId,
        item_index: TokenId,
        old_owner: &T::CrossAccountId,
        new_owner: &T::CrossAccountId,
    ) -> DispatchResult {
        Self::remove_token_index(collection_id, item_index, old_owner)?;
        Self::add_token_index(collection_id, item_index, new_owner)?;

        Ok(())
    }
    
    fn ensure_contract_owned(account: T::AccountId, contract: &T::AccountId) -> DispatchResult {
        ensure!(<ContractOwner<T>>::get(contract) == Some(account), Error::<T>::NoPermission);

        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Economic models
// #region

/// Fee multiplier.
pub type Multiplier = FixedU128;

type BalanceOf<T> = <<T as pallet_transaction_payment::Config>::OnChargeTransaction as pallet_transaction_payment::OnChargeTransaction<T>>::Balance;

/// Require the transactor pay for themselves and maybe include a tip to gain additional priority
/// in the queue.
#[derive(Encode, Decode, Clone, Eq, PartialEq)]
pub struct ChargeTransactionPayment<T: Config>(#[codec(compact)] BalanceOf<T>);

impl<T: Config + Send + Sync> sp_std::fmt::Debug 
    for ChargeTransactionPayment<T>
{
	#[cfg(feature = "std")]
	fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		write!(f, "ChargeTransactionPayment<{:?}>", self.0)
	}
	#[cfg(not(feature = "std"))]
	fn fmt(&self, _: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		Ok(())
	}
}

impl<T: Config> ChargeTransactionPayment<T>
where
    T::Call: Dispatchable<Info=DispatchInfo, PostInfo=PostDispatchInfo> + IsSubType<Call<T>> + IsSubType<pallet_contracts::Call<T>>,
    BalanceOf<T>: Send + Sync + From<u64> + FixedPointOperand,
    T::AccountId: AsRef<[u8]>,
    T::AccountId: UncheckedFrom<T::Hash>,
{
    fn traditional_fee(
        len: usize,
        info: &DispatchInfoOf<T::Call>,
        tip: BalanceOf<T>,
    ) -> BalanceOf<T>
    where
        T::Call: Dispatchable<Info = DispatchInfo>,
    {
        <pallet_transaction_payment::Module<T>>::compute_fee(len as u32, info, tip)
    }

	fn get_priority(len: usize, info: &DispatchInfoOf<T::Call>, final_fee: BalanceOf<T>) -> TransactionPriority {
        let weight_saturation = T::BlockWeights::get().max_block / info.weight.max(1);
        let max_block_length = *T::BlockLength::get().max.get(DispatchClass::Normal);
        let len_saturation = max_block_length as u64 / (len as u64).max(1);
        let coefficient: BalanceOf<T> = weight_saturation
            .min(len_saturation)
            .saturated_into::<BalanceOf<T>>();
        final_fee
            .saturating_mul(coefficient)
            .saturated_into::<TransactionPriority>()
    }

    fn withdraw_fee(
        &self,
        who: &T::AccountId,
        call: &T::Call,
        info: &DispatchInfoOf<T::Call>,
        len: usize,
	) -> Result<
		(
			BalanceOf<T>,
			<<T as pallet_transaction_payment::Config>::OnChargeTransaction as pallet_transaction_payment::OnChargeTransaction<T>>::LiquidityInfo,
		),
		TransactionValidityError,
	> {
        let tip = self.0;

        let fee = Self::traditional_fee(len, info, tip);

        // Only mess with balances if fee is not zero.
        if fee.is_zero() {
            return <<T as pallet_transaction_payment::Config>::OnChargeTransaction as pallet_transaction_payment::OnChargeTransaction<T>>::withdraw_fee(who, call, info, fee, tip)
			.map(|i| (fee, i));
        }

        // Determine who is paying transaction fee based on ecnomic model
        // Parse call to extract collection ID and access collection sponsor
        let mut sponsor: Option<T::AccountId> = (|| match IsSubType::<Call<T>>::is_sub_type(call) {
            Some(Call::create_item(collection_id, _owner, _properties)) => {
                let collection = <CollectionById<T>>::get(collection_id)?;

                // sponsor timeout
                let block_number = <system::Module<T>>::block_number() as T::BlockNumber;

                let limit = collection.limits.sponsor_transfer_timeout;
                if <CreateItemBasket<T>>::contains_key((collection_id, &who)) {
                    let last_tx_block = <CreateItemBasket<T>>::get((collection_id, &who));
                    let limit_time = last_tx_block + limit.into();
                    if block_number <= limit_time {
                        return None;
                    }
                }
                <CreateItemBasket<T>>::insert((collection_id, who.clone()), block_number);

                // check free create limit
                if collection.limits.sponsored_data_size >= (_properties.len() as u32) {
                    collection.sponsorship.sponsor()
                        .cloned()
                } else {
                    None
                }
            }
            Some(Call::transfer(_new_owner, collection_id, item_id, _value)) => {
                let collection = <CollectionById<T>>::get(collection_id)?;
                
                let mut sponsor_transfer = false;
                if collection.sponsorship.confirmed() {

                    let collection_limits = collection.limits;
                    let collection_mode = collection.mode;
    
                    // sponsor timeout
                    let block_number = <system::Module<T>>::block_number() as T::BlockNumber;
                    sponsor_transfer = match collection_mode {
                        CollectionMode::NFT => {
    
                            // get correct limit
                            let limit: u32 = if collection_limits.sponsor_transfer_timeout > 0 {
                                collection_limits.sponsor_transfer_timeout
                            } else {
                                ChainLimit::get().nft_sponsor_transfer_timeout
                            };
    
                            let mut sponsored = true;
                            if <NftTransferBasket<T>>::contains_key(collection_id, item_id) {
                                let last_tx_block = <NftTransferBasket<T>>::get(collection_id, item_id);
                                let limit_time = last_tx_block + limit.into();
                                if block_number <= limit_time {
                                    sponsored = false;
                                }
                            }
                            if sponsored {
                                <NftTransferBasket<T>>::insert(collection_id, item_id, block_number);
                            }

                            sponsored
                        }
                        CollectionMode::Fungible(_) => {
    
                            // get correct limit
                            let limit: u32 = if collection_limits.sponsor_transfer_timeout > 0 {
                                collection_limits.sponsor_transfer_timeout
                            } else {
                                ChainLimit::get().fungible_sponsor_transfer_timeout
                            };
    
                            let block_number = <system::Module<T>>::block_number() as T::BlockNumber;
                            let mut sponsored = true;
                            if <FungibleTransferBasket<T>>::contains_key(collection_id, who) {
                                let last_tx_block = <FungibleTransferBasket<T>>::get(collection_id, who);
                                let limit_time = last_tx_block + limit.into();
                                if block_number <= limit_time {
                                    sponsored = false;
                                }
                            }
                            if sponsored {
                                <FungibleTransferBasket<T>>::insert(collection_id, who, block_number);
                            }

                            sponsored
                        }
                        CollectionMode::ReFungible => {
    
                            // get correct limit
                            let limit: u32 = if collection_limits.sponsor_transfer_timeout > 0 {
                                collection_limits.sponsor_transfer_timeout
                            } else {
                                ChainLimit::get().refungible_sponsor_transfer_timeout
                            };
    
                            let mut sponsored = true;
                            if <ReFungibleTransferBasket<T>>::contains_key(collection_id, item_id) {
                                let last_tx_block = <ReFungibleTransferBasket<T>>::get(collection_id, item_id);
                                let limit_time = last_tx_block + limit.into();
                                if block_number <= limit_time {
                                    sponsored = false;
                                }
                            }
                            if sponsored {
                                <ReFungibleTransferBasket<T>>::insert(collection_id, item_id, block_number);
                            }

                            sponsored
                        }
                        _ => {
                            false
                        },
                    };
                }

                if !sponsor_transfer {
                    None
                } else {
                    collection.sponsorship.sponsor()
                        .cloned()
                }
            }

            Some(Call::set_variable_meta_data(collection_id, item_id, data)) => {
                let mut sponsor_metadata_changes = false;

                let collection = <CollectionById<T>>::get(collection_id)?;

                if
                    collection.sponsorship.confirmed() &&
                    // Can't sponsor fungible collection, this tx will be rejected
                    // as invalid
                    !matches!(collection.mode, CollectionMode::Fungible(_)) &&
                    data.len() <= collection.limits.sponsored_data_size as usize
                {
                    if let Some(rate_limit) = collection.limits.sponsored_data_rate_limit {
                        let block_number = <system::Module<T>>::block_number() as T::BlockNumber;

                        if <VariableMetaDataBasket<T>>::get(collection_id, item_id)
                            .map(|last_block| block_number - last_block > rate_limit)
                            .unwrap_or(true) 
                        {
                            sponsor_metadata_changes = true;
                            <VariableMetaDataBasket<T>>::insert(collection_id, item_id, block_number);
                        }
                    }
                }

                if !sponsor_metadata_changes {
                    None
                } else {
                    collection.sponsorship.sponsor().cloned()
                }
            }

            _ => None,
        })();

        match IsSubType::<pallet_contracts::Call<T>>::is_sub_type(call) {
            Some(pallet_contracts::Call::call(dest, _value, _gas_limit, _data)) => {

                let called_contract: T::AccountId = T::Lookup::lookup((*dest).clone()).unwrap_or(T::AccountId::default());

                let owned_contract = <ContractOwner<T>>::get(called_contract.clone()).as_ref() == Some(who);
                let white_list_enabled = <ContractWhiteListEnabled<T>>::contains_key(called_contract.clone());
                  
                if !owned_contract && white_list_enabled {
                    if !<ContractWhiteList<T>>::contains_key(called_contract.clone(), who) {
                        return Err(InvalidTransaction::Call.into());
                    }
                }
            },
            _ => {},
        }

        // Sponsor smart contracts
        sponsor = sponsor.or_else(|| match IsSubType::<pallet_contracts::Call<T>>::is_sub_type(call) {

            // On instantiation: set the contract owner
            Some(pallet_contracts::Call::instantiate(_endowment, _gas_limit, code_hash, _data, salt)) => {

                let new_contract_address = <pallet_contracts::Module<T>>::contract_address(
                    &who,
                    code_hash,
                    salt,
                );
                <ContractOwner<T>>::insert(new_contract_address.clone(), who.clone());

                None
            },

            // On instantiation with code: set the contract owner
            Some(pallet_contracts::Call::instantiate_with_code(_endowment, _gas_limit, _code, _data, _salt))  => {

                let new_contract_address = <pallet_contracts::Module<T>>::contract_address(
                    &who,
                    &T::Hashing::hash(&_code),
                    _salt,
                );

                <ContractOwner<T>>::insert(new_contract_address.clone(), who.clone());

                None
            }

            // When the contract is called, check if the sponsoring is enabled and pay fees from contract endowment if it is
            Some(pallet_contracts::Call::call(dest, _value, _gas_limit, _data)) => {

                let called_contract: T::AccountId = T::Lookup::lookup((*dest).clone()).unwrap_or(T::AccountId::default());

                let mut sponsor_transfer = false;
                if <ContractSponsoringRateLimit<T>>::contains_key(called_contract.clone()) {
                    let last_tx_block = <ContractSponsorBasket<T>>::get((&called_contract, &who));
                    let block_number = <system::Module<T>>::block_number() as T::BlockNumber;
                    let rate_limit = <ContractSponsoringRateLimit<T>>::get(&called_contract);
                    let limit_time = last_tx_block + rate_limit;

                    if block_number >= limit_time {
                        <ContractSponsorBasket<T>>::insert((called_contract.clone(), who.clone()), block_number);
                        sponsor_transfer = true;
                    }
                } else {
                    sponsor_transfer = false;
                }
               
                if sponsor_transfer {
                    if <ContractSelfSponsoring<T>>::contains_key(called_contract.clone()) {
                        if <ContractSelfSponsoring<T>>::get(called_contract.clone()) {
                            return Some(called_contract);
                        }
                    }
                }

                None
            },

            _ => None,
        });

        let who_pays_fee = sponsor.unwrap_or_else(|| who.clone());

		<<T as pallet_transaction_payment::Config>::OnChargeTransaction as pallet_transaction_payment::OnChargeTransaction<T>>::withdraw_fee(&who_pays_fee, call, info, fee, tip)
			.map(|i| (fee, i))
    }
}


impl<T: Config + Send + Sync> SignedExtension
    for ChargeTransactionPayment<T>
where
    BalanceOf<T>: Send + Sync + From<u64> + FixedPointOperand,
    T::Call: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo> + IsSubType<Call<T>> + IsSubType<pallet_contracts::Call<T>>,
    T::AccountId: AsRef<[u8]>,
    T::AccountId: UncheckedFrom<T::Hash>,
{
    const IDENTIFIER: &'static str = "ChargeTransactionPayment";
    type AccountId = T::AccountId;
    type Call = T::Call;
    type AdditionalSigned = ();
    type Pre = (
        // tip
        BalanceOf<T>,
        // who pays fee
        Self::AccountId,
		// imbalance resulting from withdrawing the fee
		<<T as pallet_transaction_payment::Config>::OnChargeTransaction as pallet_transaction_payment::OnChargeTransaction<T>>::LiquidityInfo,
    );
    fn additional_signed(&self) -> sp_std::result::Result<(), TransactionValidityError> {
        Ok(())
    }

    fn validate(
        &self,
        who: &Self::AccountId,
        call: &Self::Call,
        info: &DispatchInfoOf<Self::Call>,
        len: usize,
    ) -> TransactionValidity {
		let (fee, _) = self.withdraw_fee(who, call, info, len)?;
		Ok(ValidTransaction {
			priority: Self::get_priority(len, info, fee),
			..Default::default()
		})
    }

    fn pre_dispatch(
        self,
        who: &Self::AccountId,
        call: &Self::Call,
        info: &DispatchInfoOf<Self::Call>,
        len: usize,
    ) -> Result<Self::Pre, TransactionValidityError> {
        let (_fee, imbalance) = self.withdraw_fee(who, call, info, len)?;
        Ok((self.0, who.clone(), imbalance))
    }

    fn post_dispatch(
        pre: Self::Pre,
        info: &DispatchInfoOf<Self::Call>,
        post_info: &PostDispatchInfoOf<Self::Call>,
        len: usize,
        _result: &DispatchResult,
    ) -> Result<(), TransactionValidityError> {
		let (tip, who, imbalance) = pre;
		let actual_fee = pallet_transaction_payment::Module::<T>::compute_actual_fee(
			len as u32,
			info,
			post_info,
			tip,
		);
		<T as pallet_transaction_payment::Config>::OnChargeTransaction::correct_and_deposit_fee(&who, info, post_info, actual_fee, tip, imbalance)?;
		Ok(())
    }
}

// #endregion

sp_api::decl_runtime_apis! {
    pub trait NftApi {
        /// Used for ethereum integration
        fn eth_contract_code(account: H160) -> Option<Vec<u8>>;
    }
}
