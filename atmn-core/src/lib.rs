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

pub use chain_params::ChainParams;
pub use consensus::{Consensus, ProofOfWork};
pub use network::{Node, P2PNetwork};
pub use storage::Database;
pub use transaction::Transaction;
pub use block::Block;
pub use error::{Error, Result};

/// ATMN Core Library Version
pub const VERSION: &str = "0.1.0";

/// Initialize the Antimony blockchain
pub async fn init_blockchain(config: BlkConfig) -> Result<AtmnyBlockchain> {
    let db = Database::new(&config.db_path)?;
    let chain_params = ChainParams::mainnet();
    let consensus = Consensus::new(chain_params.clone());
    
    Ok(AtmnyBlockchain {
        db,
        consensus,
        chain_params,
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
    pub db: Database,
    pub consensus: Consensus,
    pub chain_params: ChainParams,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(VERSION, "0.1.0");
    }
}
