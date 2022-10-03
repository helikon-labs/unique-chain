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

import {Metadata} from '@polkadot/types';
import {itSub, usingPlaygrounds, expect} from './util/playgrounds';

let metadata: Metadata;

describe('TxVersion is present', () => {
  before(async () => {
    await usingPlaygrounds(async helper => {
      metadata = await helper.api!.rpc.state.getMetadata();
    });
  });

  itSub('Signed extension CheckTxVersion is present', async () => {
    expect(metadata.asLatest.extrinsic.signedExtensions.map(se => se.identifier.toString())).to.include('CheckTxVersion');
  });
});