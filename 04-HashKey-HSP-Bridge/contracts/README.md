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

## 📂 Core Contracts

| File | Role | Status |
| :--- | :--- | :--- |
| `HSPBridge.sol` | Cross-chain state receiver and finalizer | **Mainnet Ready** |
| `IdentityVerifier.sol` | Validates hardware-anchored ZK-claims | **Audited Logic** |
| `CompliantVault.sol` | Secure asset management for HSP-20 tokens | **Tested** |

## 🧪 Security & Validation
The bridge contracts have been rigorously tested against common cross-chain attack vectors, including replay attacks and unauthorized state injections.

### Test Results Summary:
* **[PASS]** Cross-chain state synchronization via AetherBridge proof.
* **[PASS]** Unauthorized agent rejection (ZK-Identity failure).
* **[PASS]** Replay attack prevention (Nonce-based validation).
* **[PASS]** HSP-20 compliance check during asset release.
