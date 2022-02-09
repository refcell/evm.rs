// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity 0.8.11;

/// @notice Sample Contract
/// @author andreas <andreas@nascent.xyz>
contract SampleContract{
	uint256 public x;

    function add(uint256 a, uint256 b) external {
        x = a + b;
    }
}
