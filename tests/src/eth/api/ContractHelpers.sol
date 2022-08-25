// SPDX-License-Identifier: OTHER
// This code is automatically generated

pragma solidity >=0.8.0 <0.9.0;

/// @dev common stubs holder
interface Dummy {

}

interface ERC165 is Dummy {
	function supportsInterface(bytes4 interfaceID) external view returns (bool);
}

<<<<<<< HEAD
<<<<<<< HEAD
/// @title Magic contract, which allows users to reconfigure other contracts
/// @dev the ERC-165 identifier for this interface is 0xd77fab70
=======
<<<<<<< HEAD
// Selector: 6073d917
=======
// Selector: 06fc42e9
>>>>>>> path: add stubs
>>>>>>> path: add stubs
=======
// Selector: 6073d917
>>>>>>> misc: update stubs
interface ContractHelpers is Dummy, ERC165 {
	/// Get user, which deployed specified contract
	/// @dev May return zero address in case if contract is deployed
	///  using uniquenetwork evm-migration pallet, or using other terms not
	///  intended by pallet-evm
	/// @dev Returns zero address if contract does not exists
	/// @param contractAddress Contract to get owner of
	/// @return address Owner of contract
	/// @dev EVM selector for this function is: 0x5152b14c,
	///  or in textual repr: contractOwner(address)
	function contractOwner(address contractAddress)
		external
		view
		returns (address);

<<<<<<< HEAD
<<<<<<< HEAD
	/// Set sponsor.
	/// @param contractAddress Contract for which a sponsor is being established.
	/// @param sponsor User address who set as pending sponsor.
	/// @dev EVM selector for this function is: 0xf01fba93,
	///  or in textual repr: setSponsor(address,address)
=======
<<<<<<< HEAD
=======
>>>>>>> misc: update stubs
	// Set sponsor.
	//
	// @param contract_address Contract for which a sponsor is being established.
	// @param sponsor User address who set as pending sponsor.
	//
	// Selector: setSponsor(address,address) f01fba93
>>>>>>> path: add stubs
	function setSponsor(address contractAddress, address sponsor) external;

	/// Set contract as self sponsored.
	///
	/// @param contractAddress Contract for which a self sponsoring is being enabled.
	/// @dev EVM selector for this function is: 0x89f7d9ae,
	///  or in textual repr: selfSponsoredEnable(address)
	function selfSponsoredEnable(address contractAddress) external;

	/// Remove sponsor.
	///
	/// @param contractAddress Contract for which a sponsorship is being removed.
	/// @dev EVM selector for this function is: 0xef784250,
	///  or in textual repr: removeSponsor(address)
	function removeSponsor(address contractAddress) external;

	/// Confirm sponsorship.
	///
	/// @dev Caller must be same that set via [`setSponsor`].
	///
	/// @param contractAddress Сontract for which need to confirm sponsorship.
	/// @dev EVM selector for this function is: 0xabc00001,
	///  or in textual repr: confirmSponsorship(address)
	function confirmSponsorship(address contractAddress) external;

<<<<<<< HEAD
	/// Get current sponsor.
	///
	/// @param contractAddress The contract for which a sponsor is requested.
	/// @return Tuble with sponsor address and his substrate mirror. If there is no confirmed sponsor error "Contract has no sponsor" throw.
	/// @dev EVM selector for this function is: 0x743fc745,
	///  or in textual repr: getSponsor(address)
=======
	// Get current sponsor.
	//
	// @param contract_address The contract for which a sponsor is requested.
	// @return Tuble with sponsor address and his substrate mirror. If there is no confirmed sponsor error "Contract has no sponsor" throw.
	//
	// Selector: getSponsor(address) 743fc745
>>>>>>> path: add stubs
	function getSponsor(address contractAddress)
		external
		view
		returns (Tuple0 memory);

<<<<<<< HEAD
<<<<<<< HEAD
	/// Check tat contract has confirmed sponsor.
	///
	/// @param contractAddress The contract for which the presence of a confirmed sponsor is checked.
	/// @return **true** if contract has confirmed sponsor.
	/// @dev EVM selector for this function is: 0x97418603,
	///  or in textual repr: hasSponsor(address)
	function hasSponsor(address contractAddress) external view returns (bool);

	/// Check tat contract has pending sponsor.
	///
	/// @param contractAddress The contract for which the presence of a pending sponsor is checked.
	/// @return **true** if contract has pending sponsor.
	/// @dev EVM selector for this function is: 0x39b9b242,
	///  or in textual repr: hasPendingSponsor(address)
=======
<<<<<<< HEAD
=======
>>>>>>> misc: update stubs
	// Check tat contract has confirmed sponsor.
	//
	// @param contract_address The contract for which the presence of a confirmed sponsor is checked.
	// @return **true** if contract has confirmed sponsor.
	//
	// Selector: hasSponsor(address) 97418603
	function hasSponsor(address contractAddress) external view returns (bool);

	// Check tat contract has pending sponsor.
	//
	// @param contract_address The contract for which the presence of a pending sponsor is checked.
	// @return **true** if contract has pending sponsor.
	//
	// Selector: hasPendingSponsor(address) 39b9b242
>>>>>>> path: add stubs
	function hasPendingSponsor(address contractAddress)
		external
		view
		returns (bool);

	/// @dev EVM selector for this function is: 0x6027dc61,
	///  or in textual repr: sponsoringEnabled(address)
	function sponsoringEnabled(address contractAddress)
		external
		view
		returns (bool);

	/// @dev EVM selector for this function is: 0xfde8a560,
	///  or in textual repr: setSponsoringMode(address,uint8)
	function setSponsoringMode(address contractAddress, uint8 mode) external;

	/// Get current contract sponsoring rate limit
	/// @param contractAddress Contract to get sponsoring mode of
	/// @return uint32 Amount of blocks between two sponsored transactions
	/// @dev EVM selector for this function is: 0x610cfabd,
	///  or in textual repr: getSponsoringRateLimit(address)
	function getSponsoringRateLimit(address contractAddress)
		external
		view
		returns (uint32);

	/// Set contract sponsoring rate limit
	/// @dev Sponsoring rate limit - is a minimum amount of blocks that should
	///  pass between two sponsored transactions
	/// @param contractAddress Contract to change sponsoring rate limit of
	/// @param rateLimit Target rate limit
	/// @dev Only contract owner can change this setting
	/// @dev EVM selector for this function is: 0x77b6c908,
	///  or in textual repr: setSponsoringRateLimit(address,uint32)
	function setSponsoringRateLimit(address contractAddress, uint32 rateLimit)
		external;

	/// Is specified user present in contract allow list
	/// @dev Contract owner always implicitly included
	/// @param contractAddress Contract to check allowlist of
	/// @param user User to check
	/// @return bool Is specified users exists in contract allowlist
	/// @dev EVM selector for this function is: 0x5c658165,
	///  or in textual repr: allowed(address,address)
	function allowed(address contractAddress, address user)
		external
		view
		returns (bool);

	/// Toggle user presence in contract allowlist
	/// @param contractAddress Contract to change allowlist of
	/// @param user Which user presence should be toggled
	/// @param isAllowed `true` if user should be allowed to be sponsored
	///  or call this contract, `false` otherwise
	/// @dev Only contract owner can change this setting
	/// @dev EVM selector for this function is: 0x4706cc1c,
	///  or in textual repr: toggleAllowed(address,address,bool)
	function toggleAllowed(
		address contractAddress,
		address user,
		bool isAllowed
	) external;

	/// Is this contract has allowlist access enabled
	/// @dev Allowlist always can have users, and it is used for two purposes:
	///  in case of allowlist sponsoring mode, users will be sponsored if they exist in allowlist
	///  in case of allowlist access enabled, only users from allowlist may call this contract
	/// @param contractAddress Contract to get allowlist access of
	/// @return bool Is specified contract has allowlist access enabled
	/// @dev EVM selector for this function is: 0xc772ef6c,
	///  or in textual repr: allowlistEnabled(address)
	function allowlistEnabled(address contractAddress)
		external
		view
		returns (bool);

	/// Toggle contract allowlist access
	/// @param contractAddress Contract to change allowlist access of
	/// @param enabled Should allowlist access to be enabled?
	/// @dev EVM selector for this function is: 0x36de20f5,
	///  or in textual repr: toggleAllowlist(address,bool)
	function toggleAllowlist(address contractAddress, bool enabled) external;
}

/// @dev anonymous struct
struct Tuple0 {
	address field_0;
	uint256 field_1;
}
