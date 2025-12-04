// atmn-core/src/consensus.rs
// Consensus mechanism (PoW + Masternode)

use serde::{Deserialize, Serialize};
use crate::chain_params::ChainParams;
use crate::block::Block;
use crate::error::Result;

/// Proof of Work consensus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofOfWork {
    pub target: [u8; 32],
    pub difficulty: u64,
}

impl ProofOfWork {
    pub fn new(bits: u32) -> Self {
        // TODO: Convert bits to target and difficulty
        ProofOfWork {
            target: [0u8; 32],
            difficulty: 1,
        }
    }

    pub fn verify_block(&self, block: &Block) -> Result<()> {
        // TODO: Verify PoW
        Ok(())
    }
}

/// Consensus engine for ATMN
#[derive(Debug, Clone)]
pub struct Consensus {
    pub chain_params: ChainParams,
}

impl Consensus {
    pub fn new(chain_params: ChainParams) -> Self {
        Consensus { chain_params }
    }

    pub fn verify_block(&self, block: &Block) -> Result<()> {
        // TODO: Implement full block validation
        Ok(())
    }

    pub fn calculate_difficulty(&self, prev_blocks: &[Block]) -> u32 {
        // TODO: Implement difficulty adjustment
        0x1d00ffff
    }

    pub fn get_block_reward(&self, height: u64) -> u64 {
        self.chain_params.get_block_reward(height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consensus_creation() {
        let params = ChainParams::mainnet();
        let consensus = Consensus::new(params.clone());
        let reward = consensus.get_block_reward(0);
        assert_eq!(reward, 50_000_000 * 100_000_000);
    }
}
