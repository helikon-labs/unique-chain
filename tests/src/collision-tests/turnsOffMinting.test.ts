import { IKeyringPair } from '@polkadot/types/types';
import chai from 'chai';
import chaiAsPromised from 'chai-as-promised';
import privateKey from '../substrate/privateKey';
import usingApi, { submitTransactionAsync } from '../substrate/substrate-api';
import waitNewBlocks from '../substrate/wait-new-blocks';
import {
  addToWhiteListExpectSuccess,
  createCollectionExpectSuccess,
  setMintPermissionExpectSuccess,
} from '../util/helpers';

chai.use(chaiAsPromised);
const expect = chai.expect;
let Alice: IKeyringPair;
let Bob: IKeyringPair;
let Ferdie: IKeyringPair;

before(async () => {
  await usingApi(async () => {
    Alice = privateKey('//Alice');
    Bob = privateKey('//Bob');
    Ferdie = privateKey('//Ferdie');
  });
});

describe('Turns off minting mode: ', () => {
  // tslint:disable-next-line: max-line-length
  it('The collection owner turns off minting mode and there are minting transactions in the same block ', async () => {
    await usingApi(async (api) => {
      const collectionId = await createCollectionExpectSuccess();
      await setMintPermissionExpectSuccess(Alice, collectionId, true);
      await addToWhiteListExpectSuccess(Alice, collectionId, Ferdie.address);
      //
      const mintItem = api.tx.nft.createItem(collectionId, Ferdie.address, 'NFT');
      const offMinting = api.tx.nft.setMintPermission(collectionId, false);
      await Promise.all
      ([
        mintItem.signAndSend(Ferdie),
        offMinting.signAndSend(Alice),
      ]);
      const itemList: any = await api.query.nft.nftItemList(collectionId, mintItem);
      expect(itemList.Owner.toString()).to.be.eq('5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuyUpnhM');
      const blockHash = await api.query.system.number();
      console.log(`blockHash: ${blockHash}`);
    });
  });
});
