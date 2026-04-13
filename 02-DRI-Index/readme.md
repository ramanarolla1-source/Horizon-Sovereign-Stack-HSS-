# 02-DRI-Index 🔍

The **Decentralized Resource Index (DRI)** is the discovery and mapping layer of the Horizon Sovereign Stack. It serves as a local-first alternative to centralized cloud indexing, allowing AI agents to autonomously discover service capabilities, credentials, and compute availability within the P2P mesh.

## 🌟 The "Cloud-Independent" Advantage
Traditional AI agents rely on centralized registries (like AWS or Google Cloud) to find counterparties. DRI eliminates this dependency, ensuring the **National Trust Layer (NTL)** remains operational even during network isolation.

* **Zero-Trust Discovery:** Every indexed resource is verified against hardware-anchored sovereign credentials.
* **Local-First Mapping:** Capability discovery happens at the edge, reducing lookup latency and increasing privacy.
* **Resilient Schema:** Optimized for low-bandwidth environments where central registry access is impossible.

## 🛠 Technical Features
* **Capability Mapping:** Maps agent specialized tasks (e.g., "Verification," "Settlement") to specific P2P nodes.
* **Credential Indexing:** Stores ZK-identity proofs locally for instant verification without third-party calls.
* **Dynamic Heartbeats:** Real-time monitoring of node availability within the decentralized mesh.

## 💻 Technical Setup
To initialize the indexing service for a local node:

```bash
# Navigate to directory
cd 02-DRI-Index

# Install indexing dependencies
npm install

# Build and Start the Indexer
npm run build
npm start
