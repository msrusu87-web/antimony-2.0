// atmn-core/src/lib.rs
// ANTIMONY COIN 2.0 - Core Blockchain Implementation

pub mod chain_params;
pub mod consensus;
pub mod network;
pub mod storage;
pub mod transaction;
pub mod block;
pub mod error;
pub mod types;
pub mod miner;
pub mod mempool;
pub mod genesis;

pub use chain_params::ChainParams;
pub use consensus::{Consensus, ProofOfWork};
pub use network::{Node, P2PNetwork};
pub use storage::Storage;
pub use transaction::Transaction;
pub use block::Block;
pub use error::{Error, Result};
pub use miner::{Miner, MinerConfig, BlockTemplate, MiningResult, MiningStats};
pub use mempool::{Mempool, MempoolConfig, MempoolStats};
pub use genesis::{create_genesis_block, initialize_genesis};

/// ATMN Core Library Version
pub const VERSION: &str = "0.1.0";

/// Initialize the Antimony blockchain
pub async fn init_blockchain(config: BlkConfig) -> Result<AtmnyBlockchain> {
    let storage = Storage::new(&config.db_path)?;
    let chain_params = ChainParams::mainnet();
    let consensus = Consensus::new(chain_params.clone());
    let mempool = Mempool::new();
    
    Ok(AtmnyBlockchain {
        storage,
        consensus,
        chain_params,
        mempool,
    })
}

pub struct BlkConfig {
    pub db_path: String,
    pub network: NetworkConfig,
}

pub struct NetworkConfig {
    pub bind_addr: String,
    pub bind_port: u16,
    pub seeds: Vec<String>,
}

pub struct AtmnyBlockchain {
    pub storage: Storage,
    pub consensus: Consensus,
    pub chain_params: ChainParams,
    pub mempool: Mempool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(VERSION, "0.1.0");
    }
}
