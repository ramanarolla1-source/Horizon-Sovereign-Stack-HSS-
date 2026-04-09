#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Map, symbol_short, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentMetadata {
    pub service_type: String,  // e.g., "GPU_COMPUTE"
    pub price_per_task: i128, // In sub-cents
    pub reputation_score: u32, // 0-1000
    pub active: bool,
}

const REGISTRY: Symbol = symbol_short!("REG");

#[contract]
pub struct DRIIndex;

#[contractimpl]
impl DRIIndex {
    // Allows an agent to list themselves on the Sovereign Stack
    pub fn register_agent(env: Env, agent: Address, service: String, price: i128) {
        agent.require_auth();

        let metadata = AgentMetadata {
            service_type: service,
            price_per_task: price,
            reputation_score: 1000, // New agents start with perfect rep
            active: true,
        };

        // Store in persistent storage
        let mut registry: Map<Address, AgentMetadata> = env.storage().persistent()
            .get(&REGISTRY)
            .unwrap_or(Map::new(&env));
            
        registry.set(agent, metadata);
        env.storage().persistent().set(&REGISTRY, &registry);
    }

    // Used by AetherBridge to find a service provider for a swarm task
    pub fn get_agent_info(env: Env, agent: Address) -> Option<AgentMetadata> {
        let registry: Map<Address, AgentMetadata> = env.storage().persistent()
            .get(&REGISTRY)
            .unwrap_or(Map::new(&env));
            
        registry.get(agent)
    }

    // Updates reputation after a successful P2P settlement (The "PayFi" hook)
    pub fn update_reputation(env: Env, agent: Address, success: bool) {
        // In production, this would be restricted to the Settlement Layer
        let mut registry: Map<Address, AgentMetadata> = env.storage().persistent()
            .get(&REGISTRY)
            .unwrap_or(Map::new(&env));

        if let Some(mut meta) = registry.get(agent.clone()) {
            if success && meta.reputation_score < 1000 {
                meta.reputation_score += 1;
            } else if !success && meta.reputation_score > 0 {
                meta.reputation_score -= 10;
            }
            registry.set(agent, meta);
            env.storage().persistent().set(&REGISTRY, &registry);
        }
    }
}
