// atmn-core/src/block.rs
// Block structure and validation

use serde::{Deserialize, Serialize};
use crate::types::{BlockHash, BlockHeight, Amount, Timestamp, Difficulty, Nonce};
use crate::transaction::Transaction;
use crate::error::Result;

/// Block Header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub version: u32,
    pub prev_block_hash: BlockHash,
    pub merkle_root: BlockHash,
    pub timestamp: Timestamp,
    pub bits: u32,
    pub nonce: Nonce,
}

/// Block - Header + Transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub height: BlockHeight,
}

impl Block {
    pub fn new(
        version: u32,
        prev_block_hash: BlockHash,
        transactions: Vec<Transaction>,
        timestamp: Timestamp,
        bits: u32,
        height: BlockHeight,
    ) -> Self {
        // Calculate merkle root
        let merkle_root = Self::calculate_merkle_root(&transactions);
        
        Block {
            header: BlockHeader {
                version,
                prev_block_hash,
                merkle_root,
                timestamp,
                bits,
                nonce: 0,
            },
            transactions,
            height,
        }
    }

    pub fn hash(&self) -> BlockHash {
        // Double SHA256 of header
        BlockHash::zero()  // TODO: Implement hashing
    }

    pub fn calculate_merkle_root(transactions: &[Transaction]) -> BlockHash {
        // TODO: Implement merkle tree calculation
        BlockHash::zero()
    }

    pub fn is_valid(&self) -> Result<()> {
        // TODO: Implement block validation
        Ok(())
    }

    pub fn get_block_reward(&self) -> Amount {
        // TODO: Get from chain params
        50 * 100_000_000  // 50 ATMN
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation() {
        let block = Block::new(
            1,
            BlockHash::zero(),
            vec![],
            1704067200,
            0x1d00ffff,
            0,
        );
        assert_eq!(block.height, 0);
    }
}
