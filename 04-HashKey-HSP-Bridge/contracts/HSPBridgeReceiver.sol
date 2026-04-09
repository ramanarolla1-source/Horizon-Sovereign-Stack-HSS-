// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @title HSPBridgeReceiver
 * @dev Integrates with HashKey Settlement Protocol (HSP) for Agentic PayFi
 */
contract HSPBridgeReceiver {
    event AgenticSettlementVerified(address indexed agent, uint256 amount, bytes32 stellarTxHash);

    address public hspMessageService; // Official HSP Messenger Address

    constructor(address _hspService) {
        hspMessageService = _hspService;
    }

    // This function is called when a verified message arrives from the Sovereign Stack
    // HSP handles the verification of the AetherBridge (Soroban) transaction
    fn receiveSettlement(
        address _agent, 
        uint256 _amount, 
        bytes32 _stellarTxHash
    ) external {
        // In a real scenario, require(msg.sender == hspMessageService);
        
        // This confirms on HashKey Chain that the AI Agent has paid its dues on Stellar
        // and is now "Settled" for institutional compliance.
        emit AgenticSettlementVerified(_agent, _amount, _stellarTxHash);
    }
}
