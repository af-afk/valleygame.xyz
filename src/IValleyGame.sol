// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

interface IValleyGame {
    function nonces(address spender) external view returns (uint256);

    function deposit(uint256 amount, address recipient) external view returns (uint256);

    function play(bytes calldata) external;
}
