# 04-HashKey-HSP-Bridge: Institutional Contracts 🏛️

This directory contains the Solidity implementation of the HSS Bridge, designed to synchronize high-speed settlement results with the HashKey Chain while maintaining institutional-grade compliance.

## ⚖️ Compliance & Standards
The contracts are engineered to adhere to the **HashKey Settlement Protocol (HSP)** standards, ensuring that all bridged transactions meet regulatory requirements before finality.

* **Identity Verification:** Integrates hardware-anchored identity claims to ensure only verified agents can trigger bridge events.
* **State Validation:** Uses a secure receiver logic to validate state proofs coming from the AetherBridge (Soroban) layer.
* **Institutional Gateway:** Designed to bridge sub-cent agentic micro-payments to institutional-grade liquidity pools.

## 🛠 Tech Stack
* **Language:** Solidity ^0.8.24
* **Network:** HashKey Chain (Chain ID 177)
* **Framework:** Hardhat / Foundry
* **Standards:** HSP-20 Compliance

## 🔄 Technical Flow
1. **Source Finality:** A transaction is batched and settled on the Soroban AetherBridge.
2. **Proof Generation:** A hardware-anchored proof of the settlement is generated.
3. **EVM Sync:** The `HSPBridge.sol` contract receives the proof on HashKey Chain.
4. **Validation:** The contract verifies the proof and the agent's ZK-identity.
5. **Finality:** Assets are released or state is updated on the HashKey global ledger.
