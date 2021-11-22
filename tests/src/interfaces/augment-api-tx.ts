// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

import type { EthereumTransactionLegacyTransaction } from './ethereum';
import type { NftDataStructsAccessMode, NftDataStructsCollectionLimits, NftDataStructsCollectionMode, NftDataStructsCreateItemData, NftDataStructsMetaUpdatePermission, NftDataStructsSchemaVersion, PalletCommonAccountBasicCrossAccountIdRepr } from './nft';
import type { CumulusPrimitivesParachainInherentParachainInherentData } from './polkadot';
import type { ApiTypes, SubmittableExtrinsic } from '@polkadot/api/types';
import type { Bytes, Compact, Option, U256, Vec, bool, u128, u16, u32, u64, u8 } from '@polkadot/types';
import type { Extrinsic } from '@polkadot/types/interfaces/extrinsics';
import type { AccountId32, Call, H160, H256, MultiAddress, Perbill } from '@polkadot/types/interfaces/runtime';
import type { PalletVestingVestingInfo, SpCoreChangesTrieChangesTrieConfiguration, XcmV1MultiLocation, XcmV2WeightLimit, XcmVersionedMultiAssets, XcmVersionedMultiLocation, XcmVersionedXcm } from '@polkadot/types/lookup';
import type { AnyNumber, ITuple } from '@polkadot/types/types';

declare module '@polkadot/api/types/submittable' {
  export interface AugmentedSubmittables<ApiType> {
    balances: {
      /**
       * Exactly as `transfer`, except the origin must be root and the source account may be
       * specified.
       * # <weight>
       * - Same as transfer, but additional read and write because the source account is not
       * assumed to be in the overlay.
       * # </weight>
       **/
      forceTransfer: AugmentedSubmittable<(source: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, dest: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, value: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, MultiAddress, Compact<u128>]>;
      /**
       * Unreserve some balance from a user by force.
       * 
       * Can only be called by ROOT.
       **/
      forceUnreserve: AugmentedSubmittable<(who: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, amount: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, u128]>;
      /**
       * Set the balances of a given account.
       * 
       * This will alter `FreeBalance` and `ReservedBalance` in storage. it will
       * also decrease the total issuance of the system (`TotalIssuance`).
       * If the new free or reserved balance is below the existential deposit,
       * it will reset the account nonce (`frame_system::AccountNonce`).
       * 
       * The dispatch origin for this call is `root`.
       * 
       * # <weight>
       * - Independent of the arguments.
       * - Contains a limited number of reads and writes.
       * ---------------------
       * - Base Weight:
       * - Creating: 27.56 µs
       * - Killing: 35.11 µs
       * - DB Weight: 1 Read, 1 Write to `who`
       * # </weight>
       **/
      setBalance: AugmentedSubmittable<(who: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, newFree: Compact<u128> | AnyNumber | Uint8Array, newReserved: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, Compact<u128>, Compact<u128>]>;
      /**
       * Transfer some liquid free balance to another account.
       * 
       * `transfer` will set the `FreeBalance` of the sender and receiver.
       * It will decrease the total issuance of the system by the `TransferFee`.
       * If the sender's account is below the existential deposit as a result
       * of the transfer, the account will be reaped.
       * 
       * The dispatch origin for this call must be `Signed` by the transactor.
       * 
       * # <weight>
       * - Dependent on arguments but not critical, given proper implementations for input config
       * types. See related functions below.
       * - It contains a limited number of reads and writes internally and no complex
       * computation.
       * 
       * Related functions:
       * 
       * - `ensure_can_withdraw` is always called internally but has a bounded complexity.
       * - Transferring balances to accounts that did not exist before will cause
       * `T::OnNewAccount::on_new_account` to be called.
       * - Removing enough funds from an account will trigger `T::DustRemoval::on_unbalanced`.
       * - `transfer_keep_alive` works the same way as `transfer`, but has an additional check
       * that the transfer will not kill the origin account.
       * ---------------------------------
       * - Base Weight: 73.64 µs, worst case scenario (account created, account removed)
       * - DB Weight: 1 Read and 1 Write to destination account
       * - Origin account is already in memory, so no DB operations for them.
       * # </weight>
       **/
      transfer: AugmentedSubmittable<(dest: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, value: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, Compact<u128>]>;
      /**
       * Transfer the entire transferable balance from the caller account.
       * 
       * NOTE: This function only attempts to transfer _transferable_ balances. This means that
       * any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be
       * transferred by this function. To ensure that this function results in a killed account,
       * you might need to prepare the account by removing any reference counters, storage
       * deposits, etc...
       * 
       * The dispatch origin of this call must be Signed.
       * 
       * - `dest`: The recipient of the transfer.
       * - `keep_alive`: A boolean to determine if the `transfer_all` operation should send all
       * of the funds the account has, causing the sender account to be killed (false), or
       * transfer everything except at least the existential deposit, which will guarantee to
       * keep the sender account alive (true). # <weight>
       * - O(1). Just like transfer, but reading the user's transferable balance first.
       * #</weight>
       **/
      transferAll: AugmentedSubmittable<(dest: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, keepAlive: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, bool]>;
      /**
       * Same as the [`transfer`] call, but with a check that the transfer will not kill the
       * origin account.
       * 
       * 99% of the time you want [`transfer`] instead.
       * 
       * [`transfer`]: struct.Pallet.html#method.transfer
       * # <weight>
       * - Cheaper than transfer because account cannot be killed.
       * - Base Weight: 51.4 µs
       * - DB Weight: 1 Read and 1 Write to dest (sender is in overlay already)
       * #</weight>
       **/
      transferKeepAlive: AugmentedSubmittable<(dest: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, value: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, Compact<u128>]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    charging: {
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    cumulusXcm: {
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    dmpQueue: {
      /**
       * Service a single overweight message.
       * 
       * - `origin`: Must pass `ExecuteOverweightOrigin`.
       * - `index`: The index of the overweight message to service.
       * - `weight_limit`: The amount of weight that message execution may take.
       * 
       * Errors:
       * - `Unknown`: Message of `index` is unknown.
       * - `OverLimit`: Message execution may use greater than `weight_limit`.
       * 
       * Events:
       * - `OverweightServiced`: On success.
       **/
      serviceOverweight: AugmentedSubmittable<(index: u64 | AnyNumber | Uint8Array, weightLimit: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u64, u64]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    ethereum: {
      /**
       * Transact an Ethereum transaction.
       **/
      transact: AugmentedSubmittable<(transaction: EthereumTransactionLegacyTransaction | { nonce?: any; gasPrice?: any; gasLimit?: any; action?: any; value?: any; input?: any; signature?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [EthereumTransactionLegacyTransaction]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    evm: {
      /**
       * Issue an EVM call operation. This is similar to a message call transaction in Ethereum.
       **/
      call: AugmentedSubmittable<(source: H160 | string | Uint8Array, target: H160 | string | Uint8Array, input: Bytes | string | Uint8Array, value: U256 | AnyNumber | Uint8Array, gasLimit: u64 | AnyNumber | Uint8Array, gasPrice: U256 | AnyNumber | Uint8Array, nonce: Option<U256> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [H160, H160, Bytes, U256, u64, U256, Option<U256>]>;
      /**
       * Issue an EVM create operation. This is similar to a contract creation transaction in
       * Ethereum.
       **/
      create: AugmentedSubmittable<(source: H160 | string | Uint8Array, init: Bytes | string | Uint8Array, value: U256 | AnyNumber | Uint8Array, gasLimit: u64 | AnyNumber | Uint8Array, gasPrice: U256 | AnyNumber | Uint8Array, nonce: Option<U256> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [H160, Bytes, U256, u64, U256, Option<U256>]>;
      /**
       * Issue an EVM create2 operation.
       **/
      create2: AugmentedSubmittable<(source: H160 | string | Uint8Array, init: Bytes | string | Uint8Array, salt: H256 | string | Uint8Array, value: U256 | AnyNumber | Uint8Array, gasLimit: u64 | AnyNumber | Uint8Array, gasPrice: U256 | AnyNumber | Uint8Array, nonce: Option<U256> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [H160, Bytes, H256, U256, u64, U256, Option<U256>]>;
      /**
       * Withdraw balance from EVM into currency/balances pallet.
       **/
      withdraw: AugmentedSubmittable<(address: H160 | string | Uint8Array, value: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [H160, u128]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    evmMigration: {
      begin: AugmentedSubmittable<(address: H160 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [H160]>;
      finish: AugmentedSubmittable<(address: H160 | string | Uint8Array, code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [H160, Bytes]>;
      setData: AugmentedSubmittable<(address: H160 | string | Uint8Array, data: Vec<ITuple<[H256, H256]>> | ([H256 | string | Uint8Array, H256 | string | Uint8Array])[]) => SubmittableExtrinsic<ApiType>, [H160, Vec<ITuple<[H256, H256]>>]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    inflation: {
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    nft: {
      /**
       * Adds an admin of the Collection.
       * NFT Collection can be controlled by multiple admin addresses (some which can also be servers, for example). Admins can issue and burn NFTs, as well as add and remove other admins, but cannot change NFT or Collection ownership.
       * 
       * # Permissions
       * 
       * * Collection Owner.
       * * Collection Admin.
       * 
       * # Arguments
       * 
       * * collection_id: ID of the Collection to add admin for.
       * 
       * * new_admin_id: Address of new admin to add.
       **/
      addCollectionAdmin: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, newAdminId: PalletCommonAccountBasicCrossAccountIdRepr | { Substrate: any } | { Ethereum: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, PalletCommonAccountBasicCrossAccountIdRepr]>;
      /**
       * Add an address to allow list.
       * 
       * # Permissions
       * 
       * * Collection Owner
       * * Collection Admin
       * 
       * # Arguments
       * 
       * * collection_id.
       * 
       * * address.
       **/
      addToAllowList: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, address: PalletCommonAccountBasicCrossAccountIdRepr | { Substrate: any } | { Ethereum: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, PalletCommonAccountBasicCrossAccountIdRepr]>;
      /**
       * Set, change, or remove approved address to transfer the ownership of the NFT.
       * 
       * # Permissions
       * 
       * * Collection Owner
       * * Collection Admin
       * * Current NFT owner
       * 
       * # Arguments
       * 
       * * approved: Address that is approved to transfer this NFT or zero (if needed to remove approval).
       * 
       * * collection_id.
       * 
       * * item_id: ID of the item.
       **/
      approve: AugmentedSubmittable<(spender: PalletCommonAccountBasicCrossAccountIdRepr | { Substrate: any } | { Ethereum: any } | string | Uint8Array, collectionId: u32 | AnyNumber | Uint8Array, itemId: u32 | AnyNumber | Uint8Array, amount: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [PalletCommonAccountBasicCrossAccountIdRepr, u32, u32, u128]>;
      /**
       * Destroys a concrete instance of NFT on behalf of the owner
       * See also: [`approve`]
       * 
       * # Permissions
       * 
       * * Collection Owner.
       * * Collection Admin.
       * * Current NFT Owner.
       * 
       * # Arguments
       * 
       * * collection_id: ID of the collection.
       * 
       * * item_id: ID of NFT to burn.
       * 
       * * from: owner of item
       **/
      burnFrom: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, from: PalletCommonAccountBasicCrossAccountIdRepr | { Substrate: any } | { Ethereum: any } | string | Uint8Array, itemId: u32 | AnyNumber | Uint8Array, value: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, PalletCommonAccountBasicCrossAccountIdRepr, u32, u128]>;
      /**
       * Destroys a concrete instance of NFT.
       * 
       * # Permissions
       * 
       * * Collection Owner.
       * * Collection Admin.
       * * Current NFT Owner.
       * 
       * # Arguments
       * 
       * * collection_id: ID of the collection.
       * 
       * * item_id: ID of NFT to burn.
       **/
      burnItem: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, itemId: u32 | AnyNumber | Uint8Array, value: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, u128]>;
      /**
       * Change the owner of the collection.
       * 
       * # Permissions
       * 
       * * Collection Owner.
       * 
       * # Arguments
       * 
       * * collection_id.
       * 
       * * new_owner.
       **/
      changeCollectionOwner: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, newOwner: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, AccountId32]>;
      /**
       * # Permissions
       * 
       * * Sponsor.
       * 
       * # Arguments
       * 
       * * collection_id.
       **/
      confirmSponsorship: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * This method creates a Collection of NFTs. Each Token may have multiple properties encoded as an array of bytes of certain length. The initial owner and admin of the collection are set to the address that signed the transaction. Both addresses can be changed later.
       * 
       * # Permissions
       * 
       * * Anyone.
       * 
       * # Arguments
       * 
       * * collection_name: UTF-16 string with collection name (limit 64 characters), will be stored as zero-terminated.
       * 
       * * collection_description: UTF-16 string with collection description (limit 256 characters), will be stored as zero-terminated.
       * 
       * * token_prefix: UTF-8 string with token prefix.
       * 
       * * mode: [CollectionMode] collection type and type dependent data.
       **/
      createCollection: AugmentedSubmittable<(collectionName: Vec<u16> | (u16 | AnyNumber | Uint8Array)[], collectionDescription: Vec<u16> | (u16 | AnyNumber | Uint8Array)[], tokenPrefix: Bytes | string | Uint8Array, mode: NftDataStructsCollectionMode | { NFT: any } | { Fungible: any } | { ReFungible: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Vec<u16>, Vec<u16>, Bytes, NftDataStructsCollectionMode]>;
      /**
       * This method creates a concrete instance of NFT Collection created with CreateCollection method.
       * 
       * # Permissions
       * 
       * * Collection Owner.
       * * Collection Admin.
       * * Anyone if
       * * Allow List is enabled, and
       * * Address is added to allow list, and
       * * MintPermission is enabled (see SetMintPermission method)
       * 
       * # Arguments
       * 
       * * collection_id: ID of the collection.
       * 
       * * owner: Address, initial owner of the NFT.
       * 
       * * data: Token data to store on chain.
       **/
      createItem: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, owner: PalletCommonAccountBasicCrossAccountIdRepr | { Substrate: any } | { Ethereum: any } | string | Uint8Array, data: NftDataStructsCreateItemData | { NFT: any } | { Fungible: any } | { ReFungible: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, PalletCommonAccountBasicCrossAccountIdRepr, NftDataStructsCreateItemData]>;
      /**
       * This method creates multiple items in a collection created with CreateCollection method.
       * 
       * # Permissions
       * 
       * * Collection Owner.
       * * Collection Admin.
       * * Anyone if
       * * Allow List is enabled, and
       * * Address is added to allow list, and
       * * MintPermission is enabled (see SetMintPermission method)
       * 
       * # Arguments
       * 
       * * collection_id: ID of the collection.
       * 
       * * itemsData: Array items properties. Each property is an array of bytes itself, see [create_item].
       * 
       * * owner: Address, initial owner of the NFT.
       **/
      createMultipleItems: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, owner: PalletCommonAccountBasicCrossAccountIdRepr | { Substrate: any } | { Ethereum: any } | string | Uint8Array, itemsData: Vec<NftDataStructsCreateItemData> | (NftDataStructsCreateItemData | { NFT: any } | { Fungible: any } | { ReFungible: any } | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [u32, PalletCommonAccountBasicCrossAccountIdRepr, Vec<NftDataStructsCreateItemData>]>;
      /**
       * **DANGEROUS**: Destroys collection and all NFTs within this collection. Users irrecoverably lose their assets and may lose real money.
       * 
       * # Permissions
       * 
       * * Collection Owner.
       * 
       * # Arguments
       * 
       * * collection_id: collection to destroy.
       **/
      destroyCollection: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Remove admin address of the Collection. An admin address can remove itself. List of admins may become empty, in which case only Collection Owner will be able to add an Admin.
       * 
       * # Permissions
       * 
       * * Collection Owner.
       * * Collection Admin.
       * 
       * # Arguments
       * 
       * * collection_id: ID of the Collection to remove admin for.
       * 
       * * account_id: Address of admin to remove.
       **/
      removeCollectionAdmin: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, accountId: PalletCommonAccountBasicCrossAccountIdRepr | { Substrate: any } | { Ethereum: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, PalletCommonAccountBasicCrossAccountIdRepr]>;
      /**
       * Switch back to pay-per-own-transaction model.
       * 
       * # Permissions
       * 
       * * Collection owner.
       * 
       * # Arguments
       * 
       * * collection_id.
       **/
      removeCollectionSponsor: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
      /**
       * Remove an address from allow list.
       * 
       * # Permissions
       * 
       * * Collection Owner
       * * Collection Admin
       * 
       * # Arguments
       * 
       * * collection_id.
       * 
       * * address.
       **/
      removeFromAllowList: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, address: PalletCommonAccountBasicCrossAccountIdRepr | { Substrate: any } | { Ethereum: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, PalletCommonAccountBasicCrossAccountIdRepr]>;
      setCollectionLimits: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, newLimit: NftDataStructsCollectionLimits | { accountTokenOwnershipLimit?: any; sponsoredDataSize?: any; sponsoredDataRateLimit?: any; tokenLimit?: any; sponsorTransferTimeout?: any; ownerCanTransfer?: any; ownerCanDestroy?: any; transfersEnabled?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, NftDataStructsCollectionLimits]>;
      /**
       * # Permissions
       * 
       * * Collection Owner
       * 
       * # Arguments
       * 
       * * collection_id.
       * 
       * * new_sponsor.
       **/
      setCollectionSponsor: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, newSponsor: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, AccountId32]>;
      /**
       * Set const on-chain data schema.
       * 
       * # Permissions
       * 
       * * Collection Owner
       * * Collection Admin
       * 
       * # Arguments
       * 
       * * collection_id.
       * 
       * * schema: String representing the const on-chain data schema.
       **/
      setConstOnChainSchema: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, schema: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, Bytes]>;
      /**
       * Set meta_update_permission value for particular collection
       * 
       * # Permissions
       * 
       * * Collection Owner.
       * 
       * # Arguments
       * 
       * * collection_id: ID of the collection.
       * 
       * * value: New flag value.
       **/
      setMetaUpdatePermissionFlag: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, value: NftDataStructsMetaUpdatePermission | 'ItemOwner' | 'Admin' | 'None' | number | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, NftDataStructsMetaUpdatePermission]>;
      /**
       * Allows Anyone to create tokens if:
       * * Allow List is enabled, and
       * * Address is added to allow list, and
       * * This method was called with True parameter
       * 
       * # Permissions
       * * Collection Owner
       * 
       * # Arguments
       * 
       * * collection_id.
       * 
       * * mint_permission: Boolean parameter. If True, allows minting to Anyone with conditions above.
       **/
      setMintPermission: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, mintPermission: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, bool]>;
      /**
       * Set off-chain data schema.
       * 
       * # Permissions
       * 
       * * Collection Owner
       * * Collection Admin
       * 
       * # Arguments
       * 
       * * collection_id.
       * 
       * * schema: String representing the offchain data schema.
       **/
      setOffchainSchema: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, schema: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, Bytes]>;
      /**
       * Toggle between normal and allow list access for the methods with access for `Anyone`.
       * 
       * # Permissions
       * 
       * * Collection Owner.
       * 
       * # Arguments
       * 
       * * collection_id.
       * 
       * * mode: [AccessMode]
       **/
      setPublicAccessMode: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, mode: NftDataStructsAccessMode | 'Normal' | 'AllowList' | number | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, NftDataStructsAccessMode]>;
      /**
       * Set schema standard
       * ImageURL
       * Unique
       * 
       * # Permissions
       * 
       * * Collection Owner
       * * Collection Admin
       * 
       * # Arguments
       * 
       * * collection_id.
       * 
       * * schema: SchemaVersion: enum
       **/
      setSchemaVersion: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, version: NftDataStructsSchemaVersion | 'ImageURL' | 'Unique' | number | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, NftDataStructsSchemaVersion]>;
      /**
       * Set transfers_enabled value for particular collection
       * 
       * # Permissions
       * 
       * * Collection Owner.
       * 
       * # Arguments
       * 
       * * collection_id: ID of the collection.
       * 
       * * value: New flag value.
       **/
      setTransfersEnabledFlag: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, value: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, bool]>;
      /**
       * Set off-chain data schema.
       * 
       * # Permissions
       * 
       * * Collection Owner
       * * Collection Admin
       * 
       * # Arguments
       * 
       * * collection_id.
       * 
       * * schema: String representing the offchain data schema.
       **/
      setVariableMetaData: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, itemId: u32 | AnyNumber | Uint8Array, data: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32, Bytes]>;
      /**
       * Set variable on-chain data schema.
       * 
       * # Permissions
       * 
       * * Collection Owner
       * * Collection Admin
       * 
       * # Arguments
       * 
       * * collection_id.
       * 
       * * schema: String representing the variable on-chain data schema.
       **/
      setVariableOnChainSchema: AugmentedSubmittable<(collectionId: u32 | AnyNumber | Uint8Array, schema: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, Bytes]>;
      /**
       * Change ownership of the token.
       * 
       * # Permissions
       * 
       * * Collection Owner
       * * Collection Admin
       * * Current NFT owner
       * 
       * # Arguments
       * 
       * * recipient: Address of token recipient.
       * 
       * * collection_id.
       * 
       * * item_id: ID of the item
       * * Non-Fungible Mode: Required.
       * * Fungible Mode: Ignored.
       * * Re-Fungible Mode: Required.
       * 
       * * value: Amount to transfer.
       * * Non-Fungible Mode: Ignored
       * * Fungible Mode: Must specify transferred amount
       * * Re-Fungible Mode: Must specify transferred portion (between 0 and 1)
       **/
      transfer: AugmentedSubmittable<(recipient: PalletCommonAccountBasicCrossAccountIdRepr | { Substrate: any } | { Ethereum: any } | string | Uint8Array, collectionId: u32 | AnyNumber | Uint8Array, itemId: u32 | AnyNumber | Uint8Array, value: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [PalletCommonAccountBasicCrossAccountIdRepr, u32, u32, u128]>;
      /**
       * Change ownership of a NFT on behalf of the owner. See Approve method for additional information. After this method executes, the approval is removed so that the approved address will not be able to transfer this NFT again from this owner.
       * 
       * # Permissions
       * * Collection Owner
       * * Collection Admin
       * * Current NFT owner
       * * Address approved by current NFT owner
       * 
       * # Arguments
       * 
       * * from: Address that owns token.
       * 
       * * recipient: Address of token recipient.
       * 
       * * collection_id.
       * 
       * * item_id: ID of the item.
       * 
       * * value: Amount to transfer.
       **/
      transferFrom: AugmentedSubmittable<(from: PalletCommonAccountBasicCrossAccountIdRepr | { Substrate: any } | { Ethereum: any } | string | Uint8Array, recipient: PalletCommonAccountBasicCrossAccountIdRepr | { Substrate: any } | { Ethereum: any } | string | Uint8Array, collectionId: u32 | AnyNumber | Uint8Array, itemId: u32 | AnyNumber | Uint8Array, value: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [PalletCommonAccountBasicCrossAccountIdRepr, PalletCommonAccountBasicCrossAccountIdRepr, u32, u32, u128]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    nftPayment: {
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    parachainSystem: {
      authorizeUpgrade: AugmentedSubmittable<(codeHash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [H256]>;
      enactAuthorizedUpgrade: AugmentedSubmittable<(code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * Set the current validation data.
       * 
       * This should be invoked exactly once per block. It will panic at the finalization
       * phase if the call was not invoked.
       * 
       * The dispatch origin for this call must be `Inherent`
       * 
       * As a side effect, this function upgrades the current validation function
       * if the appropriate time has come.
       **/
      setValidationData: AugmentedSubmittable<(data: CumulusPrimitivesParachainInherentParachainInherentData | { validationData?: any; relayChainState?: any; downwardMessages?: any; horizontalMessages?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [CumulusPrimitivesParachainInherentParachainInherentData]>;
      sudoSendUpwardMessage: AugmentedSubmittable<(message: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    polkadotXcm: {
      /**
       * Execute an XCM message from a local, signed, origin.
       * 
       * An event is deposited indicating whether `msg` could be executed completely or only
       * partially.
       * 
       * No more than `max_weight` will be used in its attempted execution. If this is less than the
       * maximum amount of weight that the message could take to be executed, then no execution
       * attempt will be made.
       * 
       * NOTE: A successful return to this does *not* imply that the `msg` was executed successfully
       * to completion; only that *some* of it was executed.
       **/
      execute: AugmentedSubmittable<(message: XcmVersionedXcm | { V0: any } | { V1: any } | { V2: any } | string | Uint8Array, maxWeight: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [XcmVersionedXcm, u64]>;
      /**
       * Set a safe XCM version (the version that XCM should be encoded with if the most recent
       * version a destination can accept is unknown).
       * 
       * - `origin`: Must be Root.
       * - `maybe_xcm_version`: The default XCM encoding version, or `None` to disable.
       **/
      forceDefaultXcmVersion: AugmentedSubmittable<(maybeXcmVersion: Option<u32> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Option<u32>]>;
      /**
       * Ask a location to notify us regarding their XCM version and any changes to it.
       * 
       * - `origin`: Must be Root.
       * - `location`: The location to which we should subscribe for XCM version notifications.
       **/
      forceSubscribeVersionNotify: AugmentedSubmittable<(location: XcmVersionedMultiLocation | { V0: any } | { V1: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [XcmVersionedMultiLocation]>;
      /**
       * Require that a particular destination should no longer notify us regarding any XCM
       * version changes.
       * 
       * - `origin`: Must be Root.
       * - `location`: The location to which we are currently subscribed for XCM version
       * notifications which we no longer desire.
       **/
      forceUnsubscribeVersionNotify: AugmentedSubmittable<(location: XcmVersionedMultiLocation | { V0: any } | { V1: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [XcmVersionedMultiLocation]>;
      /**
       * Extoll that a particular destination can be communicated with through a particular
       * version of XCM.
       * 
       * - `origin`: Must be Root.
       * - `location`: The destination that is being described.
       * - `xcm_version`: The latest version of XCM that `location` supports.
       **/
      forceXcmVersion: AugmentedSubmittable<(location: XcmV1MultiLocation | { parents?: any; interior?: any } | string | Uint8Array, xcmVersion: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [XcmV1MultiLocation, u32]>;
      /**
       * Transfer some assets from the local chain to the sovereign account of a destination chain and forward
       * a notification XCM.
       * 
       * Fee payment on the destination side is made from the first asset listed in the `assets` vector.
       * 
       * - `origin`: Must be capable of withdrawing the `assets` and executing XCM.
       * - `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send
       * from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain.
       * - `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be
       * an `AccountId32` value.
       * - `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the
       * `dest` side.
       * - `fee_asset_item`: The index into `assets` of the item which should be used to pay
       * fees.
       * - `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase.
       **/
      limitedReserveTransferAssets: AugmentedSubmittable<(dest: XcmVersionedMultiLocation | { V0: any } | { V1: any } | string | Uint8Array, beneficiary: XcmVersionedMultiLocation | { V0: any } | { V1: any } | string | Uint8Array, assets: XcmVersionedMultiAssets | { V0: any } | { V1: any } | string | Uint8Array, feeAssetItem: u32 | AnyNumber | Uint8Array, weightLimit: XcmV2WeightLimit | { Unlimited: any } | { Limited: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [XcmVersionedMultiLocation, XcmVersionedMultiLocation, XcmVersionedMultiAssets, u32, XcmV2WeightLimit]>;
      /**
       * Teleport some assets from the local chain to some destination chain.
       * 
       * Fee payment on the destination side is made from the first asset listed in the `assets` vector.
       * 
       * - `origin`: Must be capable of withdrawing the `assets` and executing XCM.
       * - `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send
       * from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain.
       * - `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be
       * an `AccountId32` value.
       * - `assets`: The assets to be withdrawn. The first item should be the currency used to to pay the fee on the
       * `dest` side. May not be empty.
       * - `dest_weight`: Equal to the total weight on `dest` of the XCM message
       * `Teleport { assets, effects: [ BuyExecution{..}, DepositAsset{..} ] }`.
       * - `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase.
       **/
      limitedTeleportAssets: AugmentedSubmittable<(dest: XcmVersionedMultiLocation | { V0: any } | { V1: any } | string | Uint8Array, beneficiary: XcmVersionedMultiLocation | { V0: any } | { V1: any } | string | Uint8Array, assets: XcmVersionedMultiAssets | { V0: any } | { V1: any } | string | Uint8Array, feeAssetItem: u32 | AnyNumber | Uint8Array, weightLimit: XcmV2WeightLimit | { Unlimited: any } | { Limited: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [XcmVersionedMultiLocation, XcmVersionedMultiLocation, XcmVersionedMultiAssets, u32, XcmV2WeightLimit]>;
      /**
       * Transfer some assets from the local chain to the sovereign account of a destination chain and forward
       * a notification XCM.
       * 
       * Fee payment on the destination side is made from the first asset listed in the `assets` vector and
       * fee-weight is calculated locally and thus remote weights are assumed to be equal to
       * local weights.
       * 
       * - `origin`: Must be capable of withdrawing the `assets` and executing XCM.
       * - `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send
       * from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain.
       * - `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be
       * an `AccountId32` value.
       * - `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the
       * `dest` side.
       * - `fee_asset_item`: The index into `assets` of the item which should be used to pay
       * fees.
       **/
      reserveTransferAssets: AugmentedSubmittable<(dest: XcmVersionedMultiLocation | { V0: any } | { V1: any } | string | Uint8Array, beneficiary: XcmVersionedMultiLocation | { V0: any } | { V1: any } | string | Uint8Array, assets: XcmVersionedMultiAssets | { V0: any } | { V1: any } | string | Uint8Array, feeAssetItem: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [XcmVersionedMultiLocation, XcmVersionedMultiLocation, XcmVersionedMultiAssets, u32]>;
      send: AugmentedSubmittable<(dest: XcmVersionedMultiLocation | { V0: any } | { V1: any } | string | Uint8Array, message: XcmVersionedXcm | { V0: any } | { V1: any } | { V2: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [XcmVersionedMultiLocation, XcmVersionedXcm]>;
      /**
       * Teleport some assets from the local chain to some destination chain.
       * 
       * Fee payment on the destination side is made from the first asset listed in the `assets` vector and
       * fee-weight is calculated locally and thus remote weights are assumed to be equal to
       * local weights.
       * 
       * - `origin`: Must be capable of withdrawing the `assets` and executing XCM.
       * - `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send
       * from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain.
       * - `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be
       * an `AccountId32` value.
       * - `assets`: The assets to be withdrawn. The first item should be the currency used to to pay the fee on the
       * `dest` side. May not be empty.
       * - `dest_weight`: Equal to the total weight on `dest` of the XCM message
       * `Teleport { assets, effects: [ BuyExecution{..}, DepositAsset{..} ] }`.
       **/
      teleportAssets: AugmentedSubmittable<(dest: XcmVersionedMultiLocation | { V0: any } | { V1: any } | string | Uint8Array, beneficiary: XcmVersionedMultiLocation | { V0: any } | { V1: any } | string | Uint8Array, assets: XcmVersionedMultiAssets | { V0: any } | { V1: any } | string | Uint8Array, feeAssetItem: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [XcmVersionedMultiLocation, XcmVersionedMultiLocation, XcmVersionedMultiAssets, u32]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    scheduler: {
      /**
       * Cancel an anonymously scheduled task.
       * 
       * # <weight>
       * - S = Number of already scheduled calls
       * - Base Weight: 22.15 + 2.869 * S µs
       * - DB Weight:
       * - Read: Agenda
       * - Write: Agenda, Lookup
       * - Will use base weight of 100 which should be good for up to 30 scheduled calls
       * # </weight>
       **/
      cancel: AugmentedSubmittable<(when: u32 | AnyNumber | Uint8Array, index: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32]>;
      /**
       * Cancel a named scheduled task.
       * 
       * # <weight>
       * - S = Number of already scheduled calls
       * - Base Weight: 24.91 + 2.907 * S µs
       * - DB Weight:
       * - Read: Agenda, Lookup
       * - Write: Agenda, Lookup
       * - Will use base weight of 100 which should be good for up to 30 scheduled calls
       * # </weight>
       **/
      cancelNamed: AugmentedSubmittable<(id: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * Anonymously schedule a task.
       * 
       * # <weight>
       * - S = Number of already scheduled calls
       * - Base Weight: 22.29 + .126 * S µs
       * - DB Weight:
       * - Read: Agenda
       * - Write: Agenda
       * - Will use base weight of 25 which should be good for up to 30 scheduled calls
       * # </weight>
       **/
      schedule: AugmentedSubmittable<(when: u32 | AnyNumber | Uint8Array, maybePeriodic: Option<ITuple<[u32, u32]>> | null | object | string | Uint8Array, priority: u8 | AnyNumber | Uint8Array, call: Call) => SubmittableExtrinsic<ApiType>, [u32, Option<ITuple<[u32, u32]>>, u8, Call]>;
      /**
       * Anonymously schedule a task after a delay.
       * 
       * # <weight>
       * Same as [`schedule`].
       * # </weight>
       **/
      scheduleAfter: AugmentedSubmittable<(after: u32 | AnyNumber | Uint8Array, maybePeriodic: Option<ITuple<[u32, u32]>> | null | object | string | Uint8Array, priority: u8 | AnyNumber | Uint8Array, call: Call) => SubmittableExtrinsic<ApiType>, [u32, Option<ITuple<[u32, u32]>>, u8, Call]>;
      /**
       * Schedule a named task.
       * 
       * # <weight>
       * - S = Number of already scheduled calls
       * - Base Weight: 29.6 + .159 * S µs
       * - DB Weight:
       * - Read: Agenda, Lookup
       * - Write: Agenda, Lookup
       * - Will use base weight of 35 which should be good for more than 30 scheduled calls
       * # </weight>
       **/
      scheduleNamed: AugmentedSubmittable<(id: Bytes | string | Uint8Array, when: u32 | AnyNumber | Uint8Array, maybePeriodic: Option<ITuple<[u32, u32]>> | null | object | string | Uint8Array, priority: u8 | AnyNumber | Uint8Array, call: Call) => SubmittableExtrinsic<ApiType>, [Bytes, u32, Option<ITuple<[u32, u32]>>, u8, Call]>;
      /**
       * Schedule a named task after a delay.
       * 
       * # <weight>
       * Same as [`schedule_named`].
       * # </weight>
       **/
      scheduleNamedAfter: AugmentedSubmittable<(id: Bytes | string | Uint8Array, after: u32 | AnyNumber | Uint8Array, maybePeriodic: Option<ITuple<[u32, u32]>> | null | object | string | Uint8Array, priority: u8 | AnyNumber | Uint8Array, call: Call) => SubmittableExtrinsic<ApiType>, [Bytes, u32, Option<ITuple<[u32, u32]>>, u8, Call]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    sudo: {
      /**
       * Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo
       * key.
       * 
       * The dispatch origin for this call must be _Signed_.
       * 
       * # <weight>
       * - O(1).
       * - Limited storage reads.
       * - One DB change.
       * # </weight>
       **/
      setKey: AugmentedSubmittable<(updated: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress]>;
      /**
       * Authenticates the sudo key and dispatches a function call with `Root` origin.
       * 
       * The dispatch origin for this call must be _Signed_.
       * 
       * # <weight>
       * - O(1).
       * - Limited storage reads.
       * - One DB write (event).
       * - Weight of derivative `call` execution + 10,000.
       * # </weight>
       **/
      sudo: AugmentedSubmittable<(call: Call) => SubmittableExtrinsic<ApiType>, [Call]>;
      /**
       * Authenticates the sudo key and dispatches a function call with `Signed` origin from
       * a given account.
       * 
       * The dispatch origin for this call must be _Signed_.
       * 
       * # <weight>
       * - O(1).
       * - Limited storage reads.
       * - One DB write (event).
       * - Weight of derivative `call` execution + 10,000.
       * # </weight>
       **/
      sudoAs: AugmentedSubmittable<(who: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, call: Call) => SubmittableExtrinsic<ApiType>, [MultiAddress, Call]>;
      /**
       * Authenticates the sudo key and dispatches a function call with `Root` origin.
       * This function does not check the weight of the call, and instead allows the
       * Sudo user to specify the weight of the call.
       * 
       * The dispatch origin for this call must be _Signed_.
       * 
       * # <weight>
       * - O(1).
       * - The weight of this call is defined by the caller.
       * # </weight>
       **/
      sudoUncheckedWeight: AugmentedSubmittable<(call: Call, weight: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Call, u64]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    system: {
      /**
       * A dispatch that will fill the block weight up to the given ratio.
       **/
      fillBlock: AugmentedSubmittable<(ratio: Perbill | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Perbill]>;
      /**
       * Kill all storage items with a key that starts with the given prefix.
       * 
       * **NOTE:** We rely on the Root origin to provide us the number of subkeys under
       * the prefix we are removing to accurately calculate the weight of this function.
       * 
       * # <weight>
       * - `O(P)` where `P` amount of keys with prefix `prefix`
       * - `P` storage deletions.
       * - Base Weight: 0.834 * P µs
       * - Writes: Number of subkeys + 1
       * # </weight>
       **/
      killPrefix: AugmentedSubmittable<(prefix: Bytes | string | Uint8Array, subkeys: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes, u32]>;
      /**
       * Kill some items from storage.
       * 
       * # <weight>
       * - `O(IK)` where `I` length of `keys` and `K` length of one key
       * - `I` storage deletions.
       * - Base Weight: .378 * i µs
       * - Writes: Number of items
       * # </weight>
       **/
      killStorage: AugmentedSubmittable<(keys: Vec<Bytes> | (Bytes | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>, [Vec<Bytes>]>;
      /**
       * Make some on-chain remark.
       * 
       * # <weight>
       * - `O(1)`
       * # </weight>
       **/
      remark: AugmentedSubmittable<(remark: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * Make some on-chain remark and emit event.
       * 
       * # <weight>
       * - `O(b)` where b is the length of the remark.
       * - 1 event.
       * # </weight>
       **/
      remarkWithEvent: AugmentedSubmittable<(remark: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * Set the new changes trie configuration.
       * 
       * # <weight>
       * - `O(1)`
       * - 1 storage write or delete (codec `O(1)`).
       * - 1 call to `deposit_log`: Uses `append` API, so O(1)
       * - Base Weight: 7.218 µs
       * - DB Weight:
       * - Writes: Changes Trie, System Digest
       * # </weight>
       **/
      setChangesTrieConfig: AugmentedSubmittable<(changesTrieConfig: Option<SpCoreChangesTrieChangesTrieConfiguration> | null | object | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Option<SpCoreChangesTrieChangesTrieConfiguration>]>;
      /**
       * Set the new runtime code.
       * 
       * # <weight>
       * - `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`
       * - 1 call to `can_set_code`: `O(S)` (calls `sp_io::misc::runtime_version` which is
       * expensive).
       * - 1 storage write (codec `O(C)`).
       * - 1 digest item.
       * - 1 event.
       * The weight of this function is dependent on the runtime, but generally this is very
       * expensive. We will treat this as a full block.
       * # </weight>
       **/
      setCode: AugmentedSubmittable<(code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * Set the new runtime code without doing any checks of the given `code`.
       * 
       * # <weight>
       * - `O(C)` where `C` length of `code`
       * - 1 storage write (codec `O(C)`).
       * - 1 digest item.
       * - 1 event.
       * The weight of this function is dependent on the runtime. We will treat this as a full
       * block. # </weight>
       **/
      setCodeWithoutChecks: AugmentedSubmittable<(code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Bytes]>;
      /**
       * Set the number of pages in the WebAssembly environment's heap.
       * 
       * # <weight>
       * - `O(1)`
       * - 1 storage write.
       * - Base Weight: 1.405 µs
       * - 1 write to HEAP_PAGES
       * - 1 digest item
       * # </weight>
       **/
      setHeapPages: AugmentedSubmittable<(pages: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u64]>;
      /**
       * Set some items of storage.
       * 
       * # <weight>
       * - `O(I)` where `I` length of `items`
       * - `I` storage writes (`O(1)`).
       * - Base Weight: 0.568 * i µs
       * - Writes: Number of items
       * # </weight>
       **/
      setStorage: AugmentedSubmittable<(items: Vec<ITuple<[Bytes, Bytes]>> | ([Bytes | string | Uint8Array, Bytes | string | Uint8Array])[]) => SubmittableExtrinsic<ApiType>, [Vec<ITuple<[Bytes, Bytes]>>]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    timestamp: {
      /**
       * Set the current time.
       * 
       * This call should be invoked exactly once per block. It will panic at the finalization
       * phase, if this call hasn't been invoked by that time.
       * 
       * The timestamp should be greater than the previous one by the amount specified by
       * `MinimumPeriod`.
       * 
       * The dispatch origin for this call must be `Inherent`.
       * 
       * # <weight>
       * - `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)
       * - 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in
       * `on_finalize`)
       * - 1 event handler `on_timestamp_set`. Must be `O(1)`.
       * # </weight>
       **/
      set: AugmentedSubmittable<(now: Compact<u64> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u64>]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    treasury: {
      /**
       * Approve a proposal. At a later time, the proposal will be allocated to the beneficiary
       * and the original deposit will be returned.
       * 
       * May only be called from `T::ApproveOrigin`.
       * 
       * # <weight>
       * - Complexity: O(1).
       * - DbReads: `Proposals`, `Approvals`
       * - DbWrite: `Approvals`
       * # </weight>
       **/
      approveProposal: AugmentedSubmittable<(proposalId: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u32>]>;
      /**
       * Put forward a suggestion for spending. A deposit proportional to the value
       * is reserved and slashed if the proposal is rejected. It is returned once the
       * proposal is awarded.
       * 
       * # <weight>
       * - Complexity: O(1)
       * - DbReads: `ProposalCount`, `origin account`
       * - DbWrites: `ProposalCount`, `Proposals`, `origin account`
       * # </weight>
       **/
      proposeSpend: AugmentedSubmittable<(value: Compact<u128> | AnyNumber | Uint8Array, beneficiary: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u128>, MultiAddress]>;
      /**
       * Reject a proposed spend. The original deposit will be slashed.
       * 
       * May only be called from `T::RejectOrigin`.
       * 
       * # <weight>
       * - Complexity: O(1)
       * - DbReads: `Proposals`, `rejected proposer account`
       * - DbWrites: `Proposals`, `rejected proposer account`
       * # </weight>
       **/
      rejectProposal: AugmentedSubmittable<(proposalId: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [Compact<u32>]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    vesting: {
      /**
       * Force a vested transfer.
       * 
       * The dispatch origin for this call must be _Root_.
       * 
       * - `source`: The account whose funds should be transferred.
       * - `target`: The account that should be transferred the vested funds.
       * - `schedule`: The vesting schedule attached to the transfer.
       * 
       * Emits `VestingCreated`.
       * 
       * NOTE: This will unlock all schedules through the current block.
       * 
       * # <weight>
       * - `O(1)`.
       * - DbWeight: 4 Reads, 4 Writes
       * - Reads: Vesting Storage, Balances Locks, Target Account, Source Account
       * - Writes: Vesting Storage, Balances Locks, Target Account, Source Account
       * # </weight>
       **/
      forceVestedTransfer: AugmentedSubmittable<(source: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, target: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, schedule: PalletVestingVestingInfo | { locked?: any; perBlock?: any; startingBlock?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, MultiAddress, PalletVestingVestingInfo]>;
      /**
       * Merge two vesting schedules together, creating a new vesting schedule that unlocks over
       * the highest possible start and end blocks. If both schedules have already started the
       * current block will be used as the schedule start; with the caveat that if one schedule
       * is finished by the current block, the other will be treated as the new merged schedule,
       * unmodified.
       * 
       * NOTE: If `schedule1_index == schedule2_index` this is a no-op.
       * NOTE: This will unlock all schedules through the current block prior to merging.
       * NOTE: If both schedules have ended by the current block, no new schedule will be created
       * and both will be removed.
       * 
       * Merged schedule attributes:
       * - `starting_block`: `MAX(schedule1.starting_block, scheduled2.starting_block,
       * current_block)`.
       * - `ending_block`: `MAX(schedule1.ending_block, schedule2.ending_block)`.
       * - `locked`: `schedule1.locked_at(current_block) + schedule2.locked_at(current_block)`.
       * 
       * The dispatch origin for this call must be _Signed_.
       * 
       * - `schedule1_index`: index of the first schedule to merge.
       * - `schedule2_index`: index of the second schedule to merge.
       **/
      mergeSchedules: AugmentedSubmittable<(schedule1Index: u32 | AnyNumber | Uint8Array, schedule2Index: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32, u32]>;
      /**
       * Unlock any vested funds of the sender account.
       * 
       * The dispatch origin for this call must be _Signed_ and the sender must have funds still
       * locked under this pallet.
       * 
       * Emits either `VestingCompleted` or `VestingUpdated`.
       * 
       * # <weight>
       * - `O(1)`.
       * - DbWeight: 2 Reads, 2 Writes
       * - Reads: Vesting Storage, Balances Locks, [Sender Account]
       * - Writes: Vesting Storage, Balances Locks, [Sender Account]
       * # </weight>
       **/
      vest: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
      /**
       * Create a vested transfer.
       * 
       * The dispatch origin for this call must be _Signed_.
       * 
       * - `target`: The account receiving the vested funds.
       * - `schedule`: The vesting schedule attached to the transfer.
       * 
       * Emits `VestingCreated`.
       * 
       * NOTE: This will unlock all schedules through the current block.
       * 
       * # <weight>
       * - `O(1)`.
       * - DbWeight: 3 Reads, 3 Writes
       * - Reads: Vesting Storage, Balances Locks, Target Account, [Sender Account]
       * - Writes: Vesting Storage, Balances Locks, Target Account, [Sender Account]
       * # </weight>
       **/
      vestedTransfer: AugmentedSubmittable<(target: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array, schedule: PalletVestingVestingInfo | { locked?: any; perBlock?: any; startingBlock?: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress, PalletVestingVestingInfo]>;
      /**
       * Unlock any vested funds of a `target` account.
       * 
       * The dispatch origin for this call must be _Signed_.
       * 
       * - `target`: The account whose vested funds should be unlocked. Must have funds still
       * locked under this pallet.
       * 
       * Emits either `VestingCompleted` or `VestingUpdated`.
       * 
       * # <weight>
       * - `O(1)`.
       * - DbWeight: 3 Reads, 3 Writes
       * - Reads: Vesting Storage, Balances Locks, Target Account
       * - Writes: Vesting Storage, Balances Locks, Target Account
       * # </weight>
       **/
      vestOther: AugmentedSubmittable<(target: MultiAddress | { Id: any } | { Index: any } | { Raw: any } | { Address32: any } | { Address20: any } | string | Uint8Array) => SubmittableExtrinsic<ApiType>, [MultiAddress]>;
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
    xcmpQueue: {
      /**
       * Generic tx
       **/
      [key: string]: SubmittableExtrinsicFunction<ApiType>;
    };
  }

  export interface SubmittableExtrinsics<ApiType extends ApiTypes> extends AugmentedSubmittables<ApiType> {
    (extrinsic: Call | Extrinsic | Uint8Array | string): SubmittableExtrinsic<ApiType>;
    [key: string]: SubmittableModuleExtrinsics<ApiType>;
  }
}