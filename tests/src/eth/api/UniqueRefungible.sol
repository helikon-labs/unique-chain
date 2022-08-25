// SPDX-License-Identifier: OTHER
// This code is automatically generated

pragma solidity >=0.8.0 <0.9.0;

<<<<<<< HEAD
/// @dev common stubs holder
=======
// Anonymous struct
struct Tuple0 {
	uint256 field_0;
	string field_1;
}

// Anonymous struct
struct Tuple1 {
	address field_0;
	uint256 field_1;
}

// Common stubs holder
>>>>>>> misk: Update stubs
interface Dummy {

}

interface ERC165 is Dummy {
	function supportsInterface(bytes4 interfaceID) external view returns (bool);
}

/// @title A contract that allows to set and delete token properties and change token property permissions.
/// @dev the ERC-165 identifier for this interface is 0x41369377
interface TokenProperties is Dummy, ERC165 {
	/// @notice Set permissions for token property.
	/// @dev Throws error if `msg.sender` is not admin or owner of the collection.
	/// @param key Property key.
	/// @param isMutable Permission to mutate property.
	/// @param collectionAdmin Permission to mutate property by collection admin if property is mutable.
	/// @param tokenOwner Permission to mutate property by token owner if property is mutable.
	/// @dev EVM selector for this function is: 0x222d97fa,
	///  or in textual repr: setTokenPropertyPermission(string,bool,bool,bool)
	function setTokenPropertyPermission(
		string memory key,
		bool isMutable,
		bool collectionAdmin,
		bool tokenOwner
	) external;

	/// @notice Set token property value.
	/// @dev Throws error if `msg.sender` has no permission to edit the property.
	/// @param tokenId ID of the token.
	/// @param key Property key.
	/// @param value Property value.
	/// @dev EVM selector for this function is: 0x1752d67b,
	///  or in textual repr: setProperty(uint256,string,bytes)
	function setProperty(
		uint256 tokenId,
		string memory key,
		bytes memory value
	) external;

	/// @notice Delete token property value.
	/// @dev Throws error if `msg.sender` has no permission to edit the property.
	/// @param tokenId ID of the token.
	/// @param key Property key.
	/// @dev EVM selector for this function is: 0x066111d1,
	///  or in textual repr: deleteProperty(uint256,string)
	function deleteProperty(uint256 tokenId, string memory key) external;

	/// @notice Get token property value.
	/// @dev Throws error if key not found
	/// @param tokenId ID of the token.
	/// @param key Property key.
	/// @return Property value bytes
	/// @dev EVM selector for this function is: 0x7228c327,
	///  or in textual repr: property(uint256,string)
	function property(uint256 tokenId, string memory key)
		external
		view
		returns (bytes memory);
}

/// @title A contract that allows you to work with collections.
/// @dev the ERC-165 identifier for this interface is 0xffe4da23
interface Collection is Dummy, ERC165 {
	/// Set collection property.
	///
	/// @param key Property key.
	/// @param value Propery value.
	/// @dev EVM selector for this function is: 0x2f073f66,
	///  or in textual repr: setCollectionProperty(string,bytes)
	function setCollectionProperty(string memory key, bytes memory value)
		external;

	/// Delete collection property.
	///
	/// @param key Property key.
	/// @dev EVM selector for this function is: 0x7b7debce,
	///  or in textual repr: deleteCollectionProperty(string)
	function deleteCollectionProperty(string memory key) external;

	/// Get collection property.
	///
	/// @dev Throws error if key not found.
	///
	/// @param key Property key.
	/// @return bytes The property corresponding to the key.
	/// @dev EVM selector for this function is: 0xcf24fd6d,
	///  or in textual repr: collectionProperty(string)
	function collectionProperty(string memory key)
		external
		view
		returns (bytes memory);

	/// Set the sponsor of the collection.
	///
	/// @dev In order for sponsorship to work, it must be confirmed on behalf of the sponsor.
	///
	/// @param sponsor Address of the sponsor from whose account funds will be debited for operations with the contract.
	/// @dev EVM selector for this function is: 0x7623402e,
	///  or in textual repr: setCollectionSponsor(address)
	function setCollectionSponsor(address sponsor) external;

	/// Collection sponsorship confirmation.
	///
	/// @dev After setting the sponsor for the collection, it must be confirmed with this function.
	/// @dev EVM selector for this function is: 0x3c50e97a,
	///  or in textual repr: confirmCollectionSponsorship()
	function confirmCollectionSponsorship() external;

	/// Set limits for the collection.
	/// @dev Throws error if limit not found.
	/// @param limit Name of the limit. Valid names:
	/// 	"accountTokenOwnershipLimit",
	/// 	"sponsoredDataSize",
	/// 	"sponsoredDataRateLimit",
	/// 	"tokenLimit",
	/// 	"sponsorTransferTimeout",
	/// 	"sponsorApproveTimeout"
	/// @param value Value of the limit.
	/// @dev EVM selector for this function is: 0x6a3841db,
	///  or in textual repr: setCollectionLimit(string,uint32)
	function setCollectionLimit(string memory limit, uint32 value) external;

	/// Set limits for the collection.
	/// @dev Throws error if limit not found.
	/// @param limit Name of the limit. Valid names:
	/// 	"ownerCanTransfer",
	/// 	"ownerCanDestroy",
	/// 	"transfersEnabled"
	/// @param value Value of the limit.
	/// @dev EVM selector for this function is: 0x993b7fba,
	///  or in textual repr: setCollectionLimit(string,bool)
	function setCollectionLimit(string memory limit, bool value) external;

	/// Get contract address.
	/// @dev EVM selector for this function is: 0xf6b4dfb4,
	///  or in textual repr: contractAddress()
	function contractAddress() external view returns (address);

	/// Add collection admin by substrate address.
	/// @param newAdmin Substrate administrator address.
	/// @dev EVM selector for this function is: 0x5730062b,
	///  or in textual repr: addCollectionAdminSubstrate(uint256)
	function addCollectionAdminSubstrate(uint256 newAdmin) external;

	/// Remove collection admin by substrate address.
	/// @param admin Substrate administrator address.
	/// @dev EVM selector for this function is: 0x4048fcf9,
	///  or in textual repr: removeCollectionAdminSubstrate(uint256)
	function removeCollectionAdminSubstrate(uint256 admin) external;

	/// Add collection admin.
	/// @param newAdmin Address of the added administrator.
	/// @dev EVM selector for this function is: 0x92e462c7,
	///  or in textual repr: addCollectionAdmin(address)
	function addCollectionAdmin(address newAdmin) external;

	/// Remove collection admin.
	///
	/// @param admin Address of the removed administrator.
	/// @dev EVM selector for this function is: 0xfafd7b42,
	///  or in textual repr: removeCollectionAdmin(address)
	function removeCollectionAdmin(address admin) external;

	/// Toggle accessibility of collection nesting.
	///
	/// @param enable If "true" degenerates to nesting: 'Owner' else to nesting: 'Disabled'
	/// @dev EVM selector for this function is: 0x112d4586,
	///  or in textual repr: setCollectionNesting(bool)
	function setCollectionNesting(bool enable) external;

	/// Toggle accessibility of collection nesting.
	///
	/// @param enable If "true" degenerates to nesting: {OwnerRestricted: [1, 2, 3]} else to nesting: 'Disabled'
	/// @param collections Addresses of collections that will be available for nesting.
	/// @dev EVM selector for this function is: 0x64872396,
	///  or in textual repr: setCollectionNesting(bool,address[])
	function setCollectionNesting(bool enable, address[] memory collections)
		external;

	/// Set the collection access method.
	/// @param mode Access mode
	/// 	0 for Normal
	/// 	1 for AllowList
	/// @dev EVM selector for this function is: 0x41835d4c,
	///  or in textual repr: setCollectionAccess(uint8)
	function setCollectionAccess(uint8 mode) external;

	/// Add the user to the allowed list.
	///
	/// @param user Address of a trusted user.
	/// @dev EVM selector for this function is: 0x67844fe6,
	///  or in textual repr: addToCollectionAllowList(address)
	function addToCollectionAllowList(address user) external;

	/// Remove the user from the allowed list.
	///
	/// @param user Address of a removed user.
	/// @dev EVM selector for this function is: 0x85c51acb,
	///  or in textual repr: removeFromCollectionAllowList(address)
	function removeFromCollectionAllowList(address user) external;

	/// Switch permission for minting.
	///
	/// @param mode Enable if "true".
	/// @dev EVM selector for this function is: 0x00018e84,
	///  or in textual repr: setCollectionMintMode(bool)
	function setCollectionMintMode(bool mode) external;

	/// Check that account is the owner or admin of the collection
	///
	/// @param user account to verify
	/// @return "true" if account is the owner or admin
	/// @dev EVM selector for this function is: 0x9811b0c7,
	///  or in textual repr: isOwnerOrAdmin(address)
	function isOwnerOrAdmin(address user) external view returns (bool);

	/// Check that substrate account is the owner or admin of the collection
	///
	/// @param user account to verify
	/// @return "true" if account is the owner or admin
	/// @dev EVM selector for this function is: 0x68910e00,
	///  or in textual repr: isOwnerOrAdminSubstrate(uint256)
	function isOwnerOrAdminSubstrate(uint256 user) external view returns (bool);

	/// Returns collection type
	///
	/// @return `Fungible` or `NFT` or `ReFungible`
	/// @dev EVM selector for this function is: 0xd34b55b8,
	///  or in textual repr: uniqueCollectionType()
	function uniqueCollectionType() external returns (string memory);

	/// Changes collection owner to another account
	///
	/// @dev Owner can be changed only by current owner
	/// @param newOwner new owner account
	/// @dev EVM selector for this function is: 0x13af4035,
	///  or in textual repr: setOwner(address)
	function setOwner(address newOwner) external;

	/// Changes collection owner to another substrate account
	///
	/// @dev Owner can be changed only by current owner
	/// @param newOwner new owner substrate account
	/// @dev EVM selector for this function is: 0xb212138f,
	///  or in textual repr: setOwnerSubstrate(uint256)
	function setOwnerSubstrate(uint256 newOwner) external;
}

/// @title ERC721 Token that can be irreversibly burned (destroyed).
/// @dev the ERC-165 identifier for this interface is 0x42966c68
interface ERC721Burnable is Dummy, ERC165 {
	/// @notice Burns a specific ERC721 token.
	/// @dev Throws unless `msg.sender` is the current RFT owner, or an authorized
	///  operator of the current owner.
	/// @param tokenId The RFT to approve
	/// @dev EVM selector for this function is: 0x42966c68,
	///  or in textual repr: burn(uint256)
	function burn(uint256 tokenId) external;
}

/// @dev inlined interface
interface ERC721MintableEvents {
	event MintingFinished();
}

/// @title ERC721 minting logic.
/// @dev the ERC-165 identifier for this interface is 0x68ccfe89
interface ERC721Mintable is Dummy, ERC165, ERC721MintableEvents {
	/// @dev EVM selector for this function is: 0x05d2035b,
	///  or in textual repr: mintingFinished()
	function mintingFinished() external view returns (bool);

	/// @notice Function to mint token.
	/// @dev `tokenId` should be obtained with `nextTokenId` method,
	///  unlike standard, you can't specify it manually
	/// @param to The new owner
	/// @param tokenId ID of the minted RFT
	/// @dev EVM selector for this function is: 0x40c10f19,
	///  or in textual repr: mint(address,uint256)
	function mint(address to, uint256 tokenId) external returns (bool);

	/// @notice Function to mint token with the given tokenUri.
	/// @dev `tokenId` should be obtained with `nextTokenId` method,
	///  unlike standard, you can't specify it manually
	/// @param to The new owner
	/// @param tokenId ID of the minted RFT
	/// @param tokenUri Token URI that would be stored in the RFT properties
	/// @dev EVM selector for this function is: 0x50bb4e7f,
	///  or in textual repr: mintWithTokenURI(address,uint256,string)
	function mintWithTokenURI(
		address to,
		uint256 tokenId,
		string memory tokenUri
	) external returns (bool);

	/// @dev Not implemented
	/// @dev EVM selector for this function is: 0x7d64bcb4,
	///  or in textual repr: finishMinting()
	function finishMinting() external returns (bool);
}

<<<<<<< HEAD
<<<<<<< HEAD
/// @title Unique extensions for ERC721.
/// @dev the ERC-165 identifier for this interface is 0x7c3bef89
=======
<<<<<<< HEAD
=======
>>>>>>> misc: update stubs
// Selector: 780e9d63
interface ERC721Enumerable is Dummy, ERC165 {
	// @notice Enumerate valid RFTs
	// @param index A counter less than `totalSupply()`
	// @return The token identifier for the `index`th NFT,
	//  (sort order not specified)
	//
	// Selector: tokenByIndex(uint256) 4f6ccce7
	function tokenByIndex(uint256 index) external view returns (uint256);

	// Not implemented
	//
	// Selector: tokenOfOwnerByIndex(address,uint256) 2f745c59
	function tokenOfOwnerByIndex(address owner, uint256 index)
		external
		view
		returns (uint256);

	// @notice Count RFTs tracked by this contract
	// @return A count of valid RFTs tracked by this contract, where each one of
	//  them has an assigned and queryable owner not equal to the zero address
	//
	// Selector: totalSupply() 18160ddd
	function totalSupply() external view returns (uint256);
}

// Selector: 7c3bef89
>>>>>>> misk: Update stubs
interface ERC721UniqueExtensions is Dummy, ERC165 {
	/// @notice Transfer ownership of an RFT
	/// @dev Throws unless `msg.sender` is the current owner. Throws if `to`
	///  is the zero address. Throws if `tokenId` is not a valid RFT.
	///  Throws if RFT pieces have multiple owners.
	/// @param to The new owner
	/// @param tokenId The RFT to transfer
	/// @dev EVM selector for this function is: 0xa9059cbb,
	///  or in textual repr: transfer(address,uint256)
	function transfer(address to, uint256 tokenId) external;

	/// @notice Burns a specific ERC721 token.
	/// @dev Throws unless `msg.sender` is the current owner or an authorized
	///  operator for this RFT. Throws if `from` is not the current owner. Throws
	///  if `to` is the zero address. Throws if `tokenId` is not a valid RFT.
	///  Throws if RFT pieces have multiple owners.
	/// @param from The current owner of the RFT
	/// @param tokenId The RFT to transfer
	/// @dev EVM selector for this function is: 0x79cc6790,
	///  or in textual repr: burnFrom(address,uint256)
	function burnFrom(address from, uint256 tokenId) external;

	/// @notice Returns next free RFT ID.
	/// @dev EVM selector for this function is: 0x75794a3c,
	///  or in textual repr: nextTokenId()
	function nextTokenId() external view returns (uint256);

	/// @notice Function to mint multiple tokens.
	/// @dev `tokenIds` should be an array of consecutive numbers and first number
	///  should be obtained with `nextTokenId` method
	/// @param to The new owner
	/// @param tokenIds IDs of the minted RFTs
	/// @dev EVM selector for this function is: 0x44a9945e,
	///  or in textual repr: mintBulk(address,uint256[])
	function mintBulk(address to, uint256[] memory tokenIds)
		external
		returns (bool);

	/// @notice Function to mint multiple tokens with the given tokenUris.
	/// @dev `tokenIds` is array of pairs of token ID and token URI. Token IDs should be consecutive
	///  numbers and first number should be obtained with `nextTokenId` method
	/// @param to The new owner
	/// @param tokens array of pairs of token ID and token URI for minted tokens
	/// @dev EVM selector for this function is: 0x36543006,
	///  or in textual repr: mintBulkWithTokenURI(address,(uint256,string)[])
	function mintBulkWithTokenURI(address to, Tuple8[] memory tokens)
		external
		returns (bool);

	/// Returns EVM address for refungible token
	///
	/// @param token ID of the token
	/// @dev EVM selector for this function is: 0xab76fac6,
	///  or in textual repr: tokenContractAddress(uint256)
	function tokenContractAddress(uint256 token)
		external
		view
		returns (address);
}

<<<<<<< HEAD
<<<<<<< HEAD
/// @dev anonymous struct
struct Tuple8 {
	uint256 field_0;
	string field_1;
}
=======
// Selector: ffe4da23
=======
// Selector: 765e2fae
>>>>>>> misk: Update stubs
=======
// Selector: e54be640
>>>>>>> misc: update stubs
interface Collection is Dummy, ERC165 {
	// Set collection property.
	//
	// @param key Property key.
	// @param value Propery value.
	//
	// Selector: setCollectionProperty(string,bytes) 2f073f66
	function setCollectionProperty(string memory key, bytes memory value)
		external;
>>>>>>> misk: Update stubs

/// @title ERC-721 Non-Fungible Token Standard, optional enumeration extension
/// @dev See https://eips.ethereum.org/EIPS/eip-721
/// @dev the ERC-165 identifier for this interface is 0x780e9d63
interface ERC721Enumerable is Dummy, ERC165 {
	/// @notice Enumerate valid RFTs
	/// @param index A counter less than `totalSupply()`
	/// @return The token identifier for the `index`th NFT,
	///  (sort order not specified)
	/// @dev EVM selector for this function is: 0x4f6ccce7,
	///  or in textual repr: tokenByIndex(uint256)
	function tokenByIndex(uint256 index) external view returns (uint256);

	/// Not implemented
	/// @dev EVM selector for this function is: 0x2f745c59,
	///  or in textual repr: tokenOfOwnerByIndex(address,uint256)
	function tokenOfOwnerByIndex(address owner, uint256 index)
		external
		view
<<<<<<< HEAD
		returns (uint256);
=======
		returns (bytes memory);

	// Set the sponsor of the collection.
	//
	// @dev In order for sponsorship to work, it must be confirmed on behalf of the sponsor.
	//
	// @param sponsor Address of the sponsor from whose account funds will be debited for operations with the contract.
	//
	// Selector: setCollectionSponsor(address) 7623402e
	function setCollectionSponsor(address sponsor) external;

	// Set the substrate sponsor of the collection.
	//
	// @dev In order for sponsorship to work, it must be confirmed on behalf of the sponsor.
	//
	// @param sponsor Substrate address of the sponsor from whose account funds will be debited for operations with the contract.
	//
	// Selector: setCollectionSponsorSubstrate(uint256) c74d6751
	function setCollectionSponsorSubstrate(uint256 sponsor) external;

	// Selector: hasCollectionPendingSponsor() 058ac185
	function hasCollectionPendingSponsor() external view returns (bool);

	// Collection sponsorship confirmation.
	//
	// @dev After setting the sponsor for the collection, it must be confirmed with this function.
	//
	// Selector: confirmCollectionSponsorship() 3c50e97a
	function confirmCollectionSponsorship() external;

	// Remove collection sponsor.
	//
	// Selector: removeCollectionSponsor() 6e0326a3
	function removeCollectionSponsor() external;

	// Get current sponsor.
	//
	// @return Tuble with sponsor address and his substrate mirror. If there is no confirmed sponsor error "Contract has no sponsor" throw.
	//
	// Selector: getCollectionSponsor() b66bbc14
	function getCollectionSponsor() external view returns (Tuple1 memory);

	// Set limits for the collection.
	// @dev Throws error if limit not found.
	// @param limit Name of the limit. Valid names:
	// 	"accountTokenOwnershipLimit",
	// 	"sponsoredDataSize",
	// 	"sponsoredDataRateLimit",
	// 	"tokenLimit",
	// 	"sponsorTransferTimeout",
	// 	"sponsorApproveTimeout"
	// @param value Value of the limit.
	//
	// Selector: setCollectionLimit(string,uint32) 6a3841db
	function setCollectionLimit(string memory limit, uint32 value) external;

	// Set limits for the collection.
	// @dev Throws error if limit not found.
	// @param limit Name of the limit. Valid names:
	// 	"ownerCanTransfer",
	// 	"ownerCanDestroy",
	// 	"transfersEnabled"
	// @param value Value of the limit.
	//
	// Selector: setCollectionLimit(string,bool) 993b7fba
	function setCollectionLimit(string memory limit, bool value) external;

	// Get contract address.
	//
	// Selector: contractAddress() f6b4dfb4
	function contractAddress() external view returns (address);

	// Add collection admin by substrate address.
	// @param new_admin Substrate administrator address.
	//
	// Selector: addCollectionAdminSubstrate(uint256) 5730062b
	function addCollectionAdminSubstrate(uint256 newAdmin) external;
>>>>>>> misk: Update stubs

	/// @notice Count RFTs tracked by this contract
	/// @return A count of valid RFTs tracked by this contract, where each one of
	///  them has an assigned and queryable owner not equal to the zero address
	/// @dev EVM selector for this function is: 0x18160ddd,
	///  or in textual repr: totalSupply()
	function totalSupply() external view returns (uint256);
}

/// @dev the ERC-165 identifier for this interface is 0x5b5e139f
interface ERC721Metadata is Dummy, ERC165 {
	/// @notice A descriptive name for a collection of RFTs in this contract
	/// @dev EVM selector for this function is: 0x06fdde03,
	///  or in textual repr: name()
	function name() external view returns (string memory);

	/// @notice An abbreviated name for RFTs in this contract
	/// @dev EVM selector for this function is: 0x95d89b41,
	///  or in textual repr: symbol()
	function symbol() external view returns (string memory);

	/// @notice A distinct Uniform Resource Identifier (URI) for a given asset.
	///
	/// @dev If the token has a `url` property and it is not empty, it is returned.
	///  Else If the collection does not have a property with key `schemaName` or its value is not equal to `ERC721Metadata`, it return an error `tokenURI not set`.
	///  If the collection property `baseURI` is empty or absent, return "" (empty string)
	///  otherwise, if token property `suffix` present and is non-empty, return concatenation of baseURI and suffix
	///  otherwise, return concatenation of `baseURI` and stringified token id (decimal stringifying, without paddings).
	///
	/// @return token's const_metadata
	/// @dev EVM selector for this function is: 0xc87b56dd,
	///  or in textual repr: tokenURI(uint256)
	function tokenURI(uint256 tokenId) external view returns (string memory);
}

/// @dev inlined interface
interface ERC721Events {
	event Transfer(
		address indexed from,
		address indexed to,
		uint256 indexed tokenId
	);
	event Approval(
		address indexed owner,
		address indexed approved,
		uint256 indexed tokenId
	);
	event ApprovalForAll(
		address indexed owner,
		address indexed operator,
		bool approved
	);
}

/// @title ERC-721 Non-Fungible Token Standard
/// @dev See https://github.com/ethereum/EIPs/blob/master/EIPS/eip-721.md
/// @dev the ERC-165 identifier for this interface is 0x58800161
interface ERC721 is Dummy, ERC165, ERC721Events {
	/// @notice Count all RFTs assigned to an owner
	/// @dev RFTs assigned to the zero address are considered invalid, and this
	///  function throws for queries about the zero address.
	/// @param owner An address for whom to query the balance
	/// @return The number of RFTs owned by `owner`, possibly zero
	/// @dev EVM selector for this function is: 0x70a08231,
	///  or in textual repr: balanceOf(address)
	function balanceOf(address owner) external view returns (uint256);

	/// @notice Find the owner of an RFT
	/// @dev RFTs assigned to zero address are considered invalid, and queries
	///  about them do throw.
	///  Returns special 0xffffffffffffffffffffffffffffffffffffffff address for
	///  the tokens that are partially owned.
	/// @param tokenId The identifier for an RFT
	/// @return The address of the owner of the RFT
	/// @dev EVM selector for this function is: 0x6352211e,
	///  or in textual repr: ownerOf(uint256)
	function ownerOf(uint256 tokenId) external view returns (address);

	/// @dev Not implemented
	/// @dev EVM selector for this function is: 0x60a11672,
	///  or in textual repr: safeTransferFromWithData(address,address,uint256,bytes)
	function safeTransferFromWithData(
		address from,
		address to,
		uint256 tokenId,
		bytes memory data
	) external;

	/// @dev Not implemented
	/// @dev EVM selector for this function is: 0x42842e0e,
	///  or in textual repr: safeTransferFrom(address,address,uint256)
	function safeTransferFrom(
		address from,
		address to,
		uint256 tokenId
	) external;

	/// @notice Transfer ownership of an RFT -- THE CALLER IS RESPONSIBLE
	///  TO CONFIRM THAT `to` IS CAPABLE OF RECEIVING NFTS OR ELSE
	///  THEY MAY BE PERMANENTLY LOST
	/// @dev Throws unless `msg.sender` is the current owner or an authorized
	///  operator for this RFT. Throws if `from` is not the current owner. Throws
	///  if `to` is the zero address. Throws if `tokenId` is not a valid RFT.
	///  Throws if RFT pieces have multiple owners.
	/// @param from The current owner of the NFT
	/// @param to The new owner
	/// @param tokenId The NFT to transfer
	/// @dev EVM selector for this function is: 0x23b872dd,
	///  or in textual repr: transferFrom(address,address,uint256)
	function transferFrom(
		address from,
		address to,
		uint256 tokenId
	) external;

	/// @dev Not implemented
	/// @dev EVM selector for this function is: 0x095ea7b3,
	///  or in textual repr: approve(address,uint256)
	function approve(address approved, uint256 tokenId) external;

	/// @dev Not implemented
	/// @dev EVM selector for this function is: 0xa22cb465,
	///  or in textual repr: setApprovalForAll(address,bool)
	function setApprovalForAll(address operator, bool approved) external;

	/// @dev Not implemented
	/// @dev EVM selector for this function is: 0x081812fc,
	///  or in textual repr: getApproved(uint256)
	function getApproved(uint256 tokenId) external view returns (address);

	/// @dev Not implemented
	/// @dev EVM selector for this function is: 0xe985e9c5,
	///  or in textual repr: isApprovedForAll(address,address)
	function isApprovedForAll(address owner, address operator)
		external
		view
		returns (address);
}

interface UniqueRefungible is
	Dummy,
	ERC165,
	ERC721,
	ERC721Metadata,
	ERC721Enumerable,
	ERC721UniqueExtensions,
	ERC721Mintable,
	ERC721Burnable,
	Collection,
	TokenProperties
{}
