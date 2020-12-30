//
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.
//

import chai from 'chai';
import chaiAsPromised from 'chai-as-promised';
import { default as usingApi } from "./substrate/substrate-api";
import { createCollectionExpectSuccess, createCollectionExpectFailure, CollectionMode } from "./util/helpers";

chai.use(chaiAsPromised);
const expect = chai.expect;

describe('integration test: ext. createCollection():', () => {
  it('Create new NFT collection', async () => {
    await createCollectionExpectSuccess({name: 'A', description: 'B', tokenPrefix: 'C', mode: 'NFT'});
  });
  it('Create new NFT collection whith collection_name of maximum length (64 bytes)', async () => {
    await createCollectionExpectSuccess({name: 'A'.repeat(64)});
  });
  it('Create new NFT collection whith collection_description of maximum length (256 bytes)', async () => {
    await createCollectionExpectSuccess({description: 'A'.repeat(256)});
  });
  it('Create new NFT collection whith token_prefix of maximum length (16 bytes)', async () => {
    await createCollectionExpectSuccess({tokenPrefix: 'A'.repeat(16)});
  });
  it('Create new Fungible collection', async () => {
    await createCollectionExpectSuccess({mode: 'Fungible'});
  });
  it('Create new ReFungible collection', async () => {
    await createCollectionExpectSuccess({mode: 'ReFungible'});
  });
});

describe('(!negative test!) integration test: ext. createCollection():', () => {
  it('(!negative test!) create new NFT collection whith incorrect data (mode)', async () => {
    await usingApi(async (api) => {
      const AcollectionCount = parseInt((await api.query.nft.collectionCount()).toString());

      const badTransaction = async function () { 
        await createCollectionExpectSuccess({mode: 'BadMode' as CollectionMode});
      };
      expect(badTransaction()).to.be.rejected;

      const BcollectionCount = parseInt((await api.query.nft.collectionCount()).toString());
      expect(BcollectionCount).to.be.equal(AcollectionCount, 'Error: Incorrect collection created.');
    });
  });
  it('(!negative test!) create new NFT collection whith incorrect data (collection_name)', async () => {
    await createCollectionExpectFailure({name: 'A'.repeat(65)});
  });
  it('(!negative test!) create new NFT collection whith incorrect data (collection_description)', async () => {
    await createCollectionExpectFailure({description: 'A'.repeat(257)});
  });
  it('(!negative test!) create new NFT collection whith incorrect data (token_prefix)', async () => {
    await createCollectionExpectFailure({tokenPrefix: 'A'.repeat(17)});
  });
});