// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity 0.8.6;

/// @notice Addition2
/// @author andreas <andreas@nascent.xyz>
contract Addition2 {

	uint256 public x;

    function add() public {
        uint256 a = 5;
        uint256 b = 6;
    	x = a + b;
   	}
}