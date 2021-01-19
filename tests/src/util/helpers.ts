//
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.
//

import { ApiPromise, Keyring } from '@polkadot/api';
import { Enum, Struct } from '@polkadot/types/codec';
import type { AccountId, EventRecord } from '@polkadot/types/interfaces';
import { u128 } from '@polkadot/types/primitive';
import { IKeyringPair } from '@polkadot/types/types';
import { BigNumber } from 'bignumber.js';
import BN from 'bn.js';
import chai from 'chai';
import chaiAsPromised from 'chai-as-promised';
import { alicesPublicKey, nullPublicKey } from '../accounts';
import privateKey from '../substrate/privateKey';
import { default as usingApi, submitTransactionAsync, submitTransactionExpectFailAsync } from '../substrate/substrate-api';
import { ICollectionInterface } from '../types';
import { hexToStr, strToUTF16, utf16ToStr } from './util';

chai.use(chaiAsPromised);
const expect = chai.expect;

type GenericResult = {
  success: boolean,
};

interface CreateCollectionResult {
  success: boolean;
  collectionId: number;
}

interface CreateItemResult {
  success: boolean;
  collectionId: number;
  itemId: number;
}

interface IReFungibleOwner {
  Fraction: BN;
  Owner: number[];
}

interface ITokenDataType {
  Owner: number[];
  ConstData: number[];
  VariableData: number[];
}

interface IFungibleTokenDataType {
  Value: BN;
}

interface IReFungibleTokenDataType {
  Owner: IReFungibleOwner[];
  ConstData: number[];
  VariableData: number[];
}

export function getGenericResult(events: EventRecord[]): GenericResult {
  const result: GenericResult = {
    success: false,
  };
  events.forEach(({ phase, event: { data, method, section } }) => {
    // console.log(`    ${phase}: ${section}.${method}:: ${data}`);
    if (method === 'ExtrinsicSuccess') {
      result.success = true;
    }
  });
  return result;
}

export function getCreateCollectionResult(events: EventRecord[]): CreateCollectionResult {
  let success = false;
  let collectionId: number = 0;
  events.forEach(({ phase, event: { data, method, section } }) => {
    // console.log(`    ${phase}: ${section}.${method}:: ${data}`);
    if (method == 'ExtrinsicSuccess') {
      success = true;
    } else if ((section == 'nft') && (method == 'Created')) {
      collectionId = parseInt(data[0].toString());
    }
  });
  const result: CreateCollectionResult = {
    success,
    collectionId,
  };
  return result;
}

export function getCreateItemResult(events: EventRecord[]): CreateItemResult {
  let success = false;
  let collectionId: number = 0;
  let itemId: number = 0;
  events.forEach(({ phase, event: { data, method, section } }) => {
    // console.log(`    ${phase}: ${section}.${method}:: ${data}`);
    if (method == 'ExtrinsicSuccess') {
      success = true;
    } else if ((section == 'nft') && (method == 'ItemCreated')) {
      collectionId = parseInt(data[0].toString());
      itemId = parseInt(data[1].toString());
    }
  });
  const result: CreateItemResult = {
    success,
    collectionId,
    itemId,
  };
  return result;
}

interface Invalid {
  type: 'Invalid';
}

interface Nft {
  type: 'NFT';
}

interface Fungible {
  type: 'Fungible';
  decimalPoints: number;
}

interface ReFungible {
  type: 'ReFungible';
  decimalPoints: number;
}

interface Nft {
  type: 'NFT'
}

interface Fungible {
  type: 'Fungible',
  decimalPoints: number
}

interface ReFungible {
  type: 'ReFungible',
  decimalPoints: number
}

type CollectionMode = Nft | Fungible | ReFungible | Invalid;

export type CreateCollectionParams = {
  mode: CollectionMode,
  name: string,
  description: string,
  tokenPrefix: string,
};

const defaultCreateCollectionParams: CreateCollectionParams = {
  description: 'description',
  mode: { type: 'NFT' },
  name: 'name',
  tokenPrefix: 'prefix',
}

export async function createCollectionExpectSuccess(params: Partial<CreateCollectionParams> = {}): Promise<number> {
  const {name, description, mode, tokenPrefix } = {...defaultCreateCollectionParams, ...params};

  let collectionId: number = 0;
  await usingApi(async (api) => {
    // Get number of collections before the transaction
    const AcollectionCount = parseInt((await api.query.nft.createdCollectionCount()).toString(), 10);

    // Run the CreateCollection transaction
    const alicePrivateKey = privateKey('//Alice');

    let modeprm = {};
    if (mode.type === 'NFT') {
      modeprm = {nft: null};
    } else if (mode.type === 'Fungible') {
      modeprm = {fungible: mode.decimalPoints};
    } else if (mode.type === 'ReFungible') {
      modeprm = {refungible: mode.decimalPoints};
    } else if (mode.type === 'Invalid') {
      modeprm = {invalid: null};
    }

    const tx = api.tx.nft.createCollection(strToUTF16(name), strToUTF16(description), strToUTF16(tokenPrefix), modeprm);
    const events = await submitTransactionAsync(alicePrivateKey, tx);
    const result = getCreateCollectionResult(events);

    // Get number of collections after the transaction
    const BcollectionCount = parseInt((await api.query.nft.createdCollectionCount()).toString(), 10);

    // Get the collection
    const collection: any = (await api.query.nft.collection(result.collectionId)).toJSON();

    // What to expect
    // tslint:disable-next-line:no-unused-expression
    expect(result.success).to.be.true;
    expect(result.collectionId).to.be.equal(BcollectionCount);
    // tslint:disable-next-line:no-unused-expression
    expect(collection).to.be.not.null;
    expect(BcollectionCount).to.be.equal(AcollectionCount + 1, 'Error: NFT collection NOT created.');
    expect(collection.Owner).to.be.equal(alicesPublicKey);
    expect(utf16ToStr(collection.Name)).to.be.equal(name);
    expect(utf16ToStr(collection.Description)).to.be.equal(description);
    expect(hexToStr(collection.TokenPrefix)).to.be.equal(tokenPrefix);

    collectionId = result.collectionId;
  });

  return collectionId;
}

export async function createCollectionExpectFailure(params: Partial<CreateCollectionParams> = {}) {
  const {name, description, mode, tokenPrefix } = {...defaultCreateCollectionParams, ...params};

  let modeprm = {};
  if (mode.type === 'NFT') {
    modeprm = {nft: null};
  } else if (mode.type === 'Fungible') {
    modeprm = {fungible: mode.decimalPoints};
  } else if (mode.type === 'ReFungible') {
    modeprm = {refungible: mode.decimalPoints};
  } else if (mode.type === 'Invalid') {
    modeprm = {invalid: null};
  }

  await usingApi(async (api) => {
    // Get number of collections before the transaction
    const AcollectionCount = parseInt((await api.query.nft.createdCollectionCount()).toString());

    // Run the CreateCollection transaction
    const alicePrivateKey = privateKey('//Alice');
    const tx = api.tx.nft.createCollection(strToUTF16(name), strToUTF16(description), strToUTF16(tokenPrefix), modeprm);
    const events = await expect(submitTransactionExpectFailAsync(alicePrivateKey, tx)).to.be.rejected;
    const result = getCreateCollectionResult(events);

    // Get number of collections after the transaction
    const BcollectionCount = parseInt((await api.query.nft.createdCollectionCount()).toString());

    // What to expect
    // tslint:disable-next-line:no-unused-expression
    expect(result.success).to.be.false;
    expect(BcollectionCount).to.be.equal(AcollectionCount, 'Error: Collection with incorrect data created.');
  });
}

export async function findUnusedAddress(api: ApiPromise): Promise<IKeyringPair> {
  let bal = new BigNumber(0);
  let unused;
  do {
    const randomSeed = 'seed' +  Math.floor(Math.random() * Math.floor(10000));
    const keyring = new Keyring({ type: 'sr25519' });
    unused = keyring.addFromUri(`//${randomSeed}`);
    bal = new BigNumber((await api.query.system.account(unused.address)).data.free.toString());
  } while (bal.toFixed() != '0');
  return unused;
}

function getDestroyResult(events: EventRecord[]): boolean {
  let success: boolean = false;
  events.forEach(({ phase, event: { data, method, section } }) => {
    // console.log(`    ${phase}: ${section}.${method}:: ${data}`);
    if (method == 'ExtrinsicSuccess') {
      success = true;
    }
  });
  return success;
}

export async function destroyCollectionExpectFailure(collectionId: number, senderSeed: string = '//Alice') {
  await usingApi(async (api) => {
    // Run the DestroyCollection transaction
    const alicePrivateKey = privateKey(senderSeed);
    const tx = api.tx.nft.destroyCollection(collectionId);
    await expect(submitTransactionExpectFailAsync(alicePrivateKey, tx)).to.be.rejected;
  });
}

export async function destroyCollectionExpectSuccess(collectionId: number, senderSeed: string = '//Alice') {
  await usingApi(async (api) => {
    // Run the DestroyCollection transaction
    const alicePrivateKey = privateKey(senderSeed);
    const tx = api.tx.nft.destroyCollection(collectionId);
    const events = await submitTransactionAsync(alicePrivateKey, tx);
    const result = getDestroyResult(events);

    // Get the collection
    const collection: any = (await api.query.nft.collection(collectionId)).toJSON();

    // What to expect
    expect(result).to.be.true;
    expect(collection).to.be.not.null;
    expect(collection.Owner).to.be.equal(nullPublicKey);
  });
}

export async function setCollectionSponsorExpectSuccess(collectionId: number, sponsor: string) {
  await usingApi(async (api) => {

    // Run the transaction
    const alicePrivateKey = privateKey('//Alice');
    const tx = api.tx.nft.setCollectionSponsor(collectionId, sponsor);
    const events = await submitTransactionAsync(alicePrivateKey, tx);
    const result = getGenericResult(events);

    // Get the collection
    const collection: any = (await api.query.nft.collection(collectionId)).toJSON();

    // What to expect
    expect(result.success).to.be.true;
    expect(collection.Sponsor.toString()).to.be.equal(sponsor.toString());
    expect(collection.SponsorConfirmed).to.be.false;
  });
}

export async function removeCollectionSponsorExpectSuccess(collectionId: number) {
  await usingApi(async (api) => {

    // Run the transaction
    const alicePrivateKey = privateKey('//Alice');
    const tx = api.tx.nft.removeCollectionSponsor(collectionId);
    const events = await submitTransactionAsync(alicePrivateKey, tx);
    const result = getGenericResult(events);

    // Get the collection
    const collection: any = (await api.query.nft.collection(collectionId)).toJSON();

    // What to expect
    expect(result.success).to.be.true;
    expect(collection.Sponsor).to.be.equal(nullPublicKey);
    expect(collection.SponsorConfirmed).to.be.false;
  });
}

export async function removeCollectionSponsorExpectFailure(collectionId: number) {
  await usingApi(async (api) => {

    // Run the transaction
    const alicePrivateKey = privateKey('//Alice');
    const tx = api.tx.nft.removeCollectionSponsor(collectionId);
    await expect(submitTransactionExpectFailAsync(alicePrivateKey, tx)).to.be.rejected;
  });
}

export async function setCollectionSponsorExpectFailure(collectionId: number, sponsor: string, senderSeed: string = '//Alice') {
  await usingApi(async (api) => {

    // Run the transaction
    const alicePrivateKey = privateKey(senderSeed);
    const tx = api.tx.nft.setCollectionSponsor(collectionId, sponsor);
    await expect(submitTransactionExpectFailAsync(alicePrivateKey, tx)).to.be.rejected;
  });
}

export async function confirmSponsorshipExpectSuccess(collectionId: number, senderSeed: string = '//Alice') {
  await usingApi(async (api) => {

    // Run the transaction
    const sender = privateKey(senderSeed);
    const tx = api.tx.nft.confirmSponsorship(collectionId);
    const events = await submitTransactionAsync(sender, tx);
    const result = getGenericResult(events);

    // Get the collection
    const collection: any = (await api.query.nft.collection(collectionId)).toJSON();

    // What to expect
    expect(result.success).to.be.true;
    expect(collection.Sponsor).to.be.equal(sender.address);
    expect(collection.SponsorConfirmed).to.be.true;
  });
}

export async function confirmSponsorshipExpectFailure(collectionId: number, senderSeed: string = '//Alice') {
  await usingApi(async (api) => {

    // Run the transaction
    const sender = privateKey(senderSeed);
    const tx = api.tx.nft.confirmSponsorship(collectionId);
    await expect(submitTransactionExpectFailAsync(sender, tx)).to.be.rejected;
  });
}

export interface CreateFungibleData extends Struct {
  readonly value: u128;
}

export interface CreateReFungibleData extends Struct {}
export interface CreateNftData extends Struct {}

export interface CreateItemData extends Enum {
  NFT: CreateNftData;
  Fungible: CreateFungibleData;
  ReFungible: CreateReFungibleData;
}

export async function
approveExpectSuccess(collectionId: number,
                     tokenId: number, owner: IKeyringPair, approved: IKeyringPair, amount: number = 1) {
  await usingApi(async (api: ApiPromise) => {
    const allowanceBefore =
      await api.query.nft.allowances(collectionId, [tokenId, owner.address, approved.address]) as unknown as BN;
    const approveNftTx = await api.tx.nft.approve(approved.address, collectionId, tokenId, amount);
    const events = await submitTransactionAsync(owner, approveNftTx);
    const result = getCreateItemResult(events);
    // tslint:disable-next-line:no-unused-expression
    expect(result.success).to.be.true;
    const allowanceAfter =
      await api.query.nft.allowances(collectionId, [tokenId, owner.address, approved.address]) as unknown as BN;
    expect(allowanceAfter.toNumber() - allowanceBefore.toNumber()).to.be.equal(amount);
  });
}

export async function
transferFromExpectSuccess(collectionId: number,
                          tokenId: number,
                          accountApproved: IKeyringPair,
                          accountFrom: IKeyringPair,
                          accountTo: IKeyringPair,
                          value: number = 1,
                          type: string = 'NFT') {
  await usingApi(async (api: ApiPromise) => {
    let balanceBefore = new BN(0);
    if (type === 'Fungible') {
      balanceBefore = await api.query.nft.balance(collectionId, accountTo.address) as unknown as BN;
    }
    const transferFromTx = await api.tx.nft.transferFrom(
      accountFrom.address, accountTo.address, collectionId, tokenId, value);
    const events = await submitTransactionAsync(accountFrom, transferFromTx);
    const result = getCreateItemResult(events);
    // tslint:disable-next-line:no-unused-expression
    expect(result.success).to.be.true;
    if (type === 'NFT') {
      const nftItemData = await api.query.nft.nftItemList(collectionId, tokenId) as unknown as ITokenDataType;
      expect(nftItemData.Owner.toString()).to.be.equal(accountTo.address);
    }
    if (type === 'Fungible') {
      const balanceAfter = await api.query.nft.balance(collectionId, accountTo.address) as unknown as BN;
      expect(balanceAfter.sub(balanceBefore).toNumber()).to.be.equal(value);
    }
    if (type === 'ReFungible') {
      const nftItemData =
        await api.query.nft.reFungibleItemList(collectionId, tokenId) as unknown as IReFungibleTokenDataType;
      expect(nftItemData.Owner[0].Owner.toString()).to.be.equal(accountTo.address);
      expect(nftItemData.Owner[0].Fraction.toNumber()).to.be.equal(value);
    }
  });
}

export async function
transferFromExpectFail(collectionId: number,
                       tokenId: number,
                       accountApproved: IKeyringPair,
                       accountFrom: IKeyringPair,
                       accountTo: IKeyringPair,
                       value: number = 1) {
  await usingApi(async (api: ApiPromise) => {
    const transferFromTx = await api.tx.nft.transferFrom(
      accountFrom.address, accountTo.address, collectionId, tokenId, value);
    const events = await expect(submitTransactionExpectFailAsync(accountApproved, transferFromTx)).to.be.rejected;
    const result = getCreateCollectionResult(events);
    // tslint:disable-next-line:no-unused-expression
    expect(result.success).to.be.false;
  });
}

export async function
transferExpectSuccess(collectionId: number,
                      tokenId: number,
                      sender: IKeyringPair,
                      recipient: IKeyringPair,
                      value: number = 1,
                      type: string = 'NFT') {
  await usingApi(async (api: ApiPromise) => {
    let balanceBefore = new BN(0);
    if (type === 'Fungible') {
      balanceBefore = await api.query.nft.balance(collectionId, recipient.address) as unknown as BN;
    }
    const transferTx = await api.tx.nft.transfer(recipient.address, collectionId, tokenId, value);
    const events = await submitTransactionAsync(sender, transferTx);
    const result = getCreateItemResult(events);
    // tslint:disable-next-line:no-unused-expression
    expect(result.success).to.be.true;
    if (type === 'NFT') {
      const nftItemData = await api.query.nft.nftItemList(collectionId, tokenId) as unknown as ITokenDataType;
      expect(nftItemData.Owner.toString()).to.be.equal(recipient.address);
    }
    if (type === 'Fungible') {
      const balanceAfter = await api.query.nft.balance(collectionId, recipient.address) as unknown as BN;
      expect(balanceAfter.sub(balanceBefore).toNumber()).to.be.equal(value);
    }
    if (type === 'ReFungible') {
      const nftItemData =
        await api.query.nft.reFungibleItemList(collectionId, tokenId) as unknown as IReFungibleTokenDataType;
      expect(nftItemData.Owner[0].Owner.toString()).to.be.equal(recipient.address);
      expect(nftItemData.Owner[0].Fraction.toNumber()).to.be.equal(value);
    }
  });
}

export async function
approveExpectFail(collectionId: number,
                  tokenId: number, owner: IKeyringPair, approved: IKeyringPair, amount: number = 1) {
  await usingApi(async (api: ApiPromise) => {
    const approveNftTx = await api.tx.nft.approve(approved.address, collectionId, tokenId, amount);
    const events = await expect(submitTransactionExpectFailAsync(owner, approveNftTx)).to.be.rejected;
    const result = getCreateCollectionResult(events);
    // tslint:disable-next-line:no-unused-expression
    expect(result.success).to.be.false;
  });
}

export async function createItemExpectSuccess(sender: IKeyringPair, collectionId: number, createMode: string, owner: string = '') {
  let newItemId: number = 0;
  await usingApi(async (api) => {
    const AItemCount = parseInt((await api.query.nft.itemListIndex(collectionId)).toString(), 10);
    const Aitem: any = (await api.query.nft.fungibleItemList(collectionId, owner)).toJSON();
    const AItemBalance = new BigNumber(Aitem.Value);

    if (owner === '') {
      owner = sender.address;
    }

    let tx;
    if (createMode === 'Fungible') {
      const createData = {fungible: {value: 10}};
      tx = api.tx.nft.createItem(collectionId, owner, createData);
    } else {
      tx = api.tx.nft.createItem(collectionId, owner, createMode);
    }
    const events = await submitTransactionAsync(sender, tx);
    const result = getCreateItemResult(events);

    const BItemCount = parseInt((await api.query.nft.itemListIndex(collectionId)).toString(), 10);
    const Bitem: any = (await api.query.nft.fungibleItemList(collectionId, owner)).toJSON();
    const BItemBalance = new BigNumber(Bitem.Value);

    // What to expect
    // tslint:disable-next-line:no-unused-expression
    expect(result.success).to.be.true;
    if (createMode === 'Fungible') {
      expect(BItemBalance.minus(AItemBalance).toNumber()).to.be.equal(10);
    } else {
      expect(BItemCount).to.be.equal(AItemCount + 1);
    }
    expect(collectionId).to.be.equal(result.collectionId);
    expect(BItemCount).to.be.equal(result.itemId);
    newItemId = result.itemId;
  });
  return newItemId;
}

export async function enableWhiteListExpectSuccess(sender: IKeyringPair, collectionId: number) {
  await usingApi(async (api) => {

    // Run the transaction
    const tx = api.tx.nft.setPublicAccessMode(collectionId, 'WhiteList');
    const events = await submitTransactionAsync(sender, tx);
    const result = getGenericResult(events);

    // Get the collection
    const collection: any = (await api.query.nft.collection(collectionId)).toJSON();

    // What to expect
    expect(result.success).to.be.true;
    expect(collection.Access).to.be.equal('WhiteList');
  });
}

export async function enablePublicMintingExpectSuccess(sender: IKeyringPair, collectionId: number) {
  await usingApi(async (api) => {

    // Run the transaction
    const tx = api.tx.nft.setMintPermission(collectionId, true);
    const events = await submitTransactionAsync(sender, tx);
    const result = getGenericResult(events);

    // Get the collection
    const collection: any = (await api.query.nft.collection(collectionId)).toJSON();

    // What to expect
    expect(result.success).to.be.true;
    expect(collection.MintMode).to.be.equal(true);
  });
}

export async function addToWhiteListExpectSuccess(sender: IKeyringPair, collectionId: number, address: string) {
  await usingApi(async (api) => {

    // Run the transaction
    const tx = api.tx.nft.addToWhiteList(collectionId, address);
    const events = await submitTransactionAsync(sender, tx);
    const result = getGenericResult(events);

    // Get the collection
    const collection: any = (await api.query.nft.collection(collectionId)).toJSON();

    // What to expect
    expect(result.success).to.be.true;
    expect(collection.MintMode).to.be.equal(true);
  });
}

export const getDetailedCollectionInfo = async (api: ApiPromise, collectionId: number)
  : Promise<ICollectionInterface | null> => {
  return await api.query.nft.collection(collectionId) as unknown as ICollectionInterface;
};

export const getCreatedCollectionCount = async (api: ApiPromise): Promise<number> => {
  // set global object - collectionsCount
  return (await api.query.nft.createdCollectionCount() as unknown as BN).toNumber();
};
