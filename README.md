Horizon Sovereign Stack (HSS)High-Performance, P2P Sovereign PayFi Rails for the Agentic Economy.📖 OverviewThe Horizon Sovereign Stack (HSS) is a decentralized infrastructure layer built to power autonomous AI swarms. It bridges the gap between machine intelligence and financial settlement by combining P2P discovery with Soroban-based batch settlement.HSS allows AI agents to operate outside the "Cloud Tax," enabling sub-cent micro-payments and mission-critical resilience in network-isolated environments.🏗️ The "Sovereign" ArchitectureThe stack is organized into three synergistic layers:Liquidity Layer (Xythum): Real-world asset (RWA) representation and liquidity vaulting.Discovery Layer (DRI): A Decentralized Resource Index that maps agent capabilities and compute availability.Settlement Layer (Aether Logic): A high-speed engine using Stellar/Soroban for $0.01 batching and HashKey HSP for institutional-grade compliance.⚡ Key Performance MetricsMetricHorizon Sovereign StackTraditional Cloud APIImprovementM2M Latency< 200ms~2000ms90% ReductionSettlement Cost$0.01 (Batch)$0.15 - $0.5095% CheaperUptime100% (P2P Mesh)Cloud-DependentResilient to Blackouts
Prerequisites
Rust & Cargo (for Soroban contracts)

Stellar CLI

Node.js (for the P2P discovery layer)
git clone https://github.com/[Your-Username]/Horizon-Sovereign-Stack-HSS.git
cd Horizon-Sovereign-Stack-HSS
# Install dependencies for the P2P mesh
cd 03-Sovereign-Settlement && npm install
Each layer contains its own specialized logic, from Soroban smart contracts (Rust) to P2P networking (Node.js).
One-Pager: https://docs.google.com/document/d/1ZfYUoclxfAS2lnqpyPIbajLQVQdOpkFh8DdSNQkDL9c/edit?usp=sharing

Demo Video: 
https://youtu.be/d7ISy4UiMoo

