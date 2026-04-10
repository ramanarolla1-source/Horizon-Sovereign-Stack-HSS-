# 04 | HashKey HSP & Soroban Bridge

This module acts as the institutional gateway for the **Horizon Sovereign Stack (HSS)**. It facilitates the secure movement of identity-verified assets between the **HashKey Chain (HSP)** and the **Stellar/Soroban** settlement layer.

## 🌉 Core Functionality
- **Institutional Compliance:** Implements the HashKey Settlement Protocol (HSP) standards to ensure all bridged assets meet sovereign regulatory requirements.
- **Soroban Settlement:** Utilizes Rust-based smart contracts on Stellar (Soroban) to execute high-frequency, low-cost settlement ($0.01 per batch).
- **ZK-Proof Verification:** Validates hardware-anchored identity claims before allowing cross-chain state transitions.

## 🛠 Technical Stack
- **Smart Contracts:** Rust (Soroban SDK)
- **Protocol:** HashKey Settlement Protocol (HSP)
- **Security:** Hardware-anchored ZK-Identity proofs

## 📂 Structure
- `/contracts`: Rust-native Soroban contracts for bridge logic and asset vaulting.
- `/scripts`: Deployment and interaction scripts for the HashKey Testnet environment.
- `/docs`: Technical specifications for the HSP-to-Soroban state relay.

## 🚀 Deployment (Dev Mode)
To build the bridge contracts:
1. Ensure you have the `soroban-cli` installed.
2. Run:
   ```bash
   soroban contract build

   To run tests:
   soroban contract test
