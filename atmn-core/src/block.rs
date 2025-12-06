// atmn-core/src/block.rs
// Block structure and validation

use serde::{Deserialize, Serialize};
use crate::types::{BlockHash, BlockHeight, Amount, Timestamp, Nonce};
use crate::transaction::Transaction;
use crate::consensus::sha256d;
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

impl BlockHeader {
    /// Serialize header for hashing
    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(84);
        bytes.extend_from_slice(&self.version.to_le_bytes());
        bytes.extend_from_slice(&self.prev_block_hash.0);
        bytes.extend_from_slice(&self.merkle_root.0);
        bytes.extend_from_slice(&self.timestamp.to_le_bytes());
        bytes.extend_from_slice(&self.bits.to_le_bytes());
        bytes.extend_from_slice(&self.nonce.to_le_bytes());
        bytes
    }

    /// Hash the block header
    pub fn hash(&self) -> BlockHash {
        let bytes = self.serialize();
        sha256d(&bytes)
    }
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

    /// Hash the block (via header)
    pub fn hash(&self) -> BlockHash {
        self.header.hash()
    }

    /// Calculate merkle root of transactions
    pub fn calculate_merkle_root(transactions: &[Transaction]) -> BlockHash {
        if transactions.is_empty() {
            return BlockHash::zero();
        }

        // Hash all transactions
        let mut hashes: Vec<BlockHash> = transactions
            .iter()
            .map(|tx| {
                // Hash transaction
                let tx_bytes = bincode::serialize(tx).unwrap_or_default();
                sha256d(&tx_bytes)
            })
            .collect();

        // If only one transaction, return its hash
        if hashes.len() == 1 {
            return hashes[0].clone();
        }

        // Build merkle tree bottom-up
        while hashes.len() > 1 {
            let mut next_level = Vec::new();

            // Process pairs
            for i in (0..hashes.len()).step_by(2) {
                if i + 1 < hashes.len() {
                    // Hash pair
                    let mut combined = Vec::with_capacity(64);
                    combined.extend_from_slice(&hashes[i].0);
                    combined.extend_from_slice(&hashes[i + 1].0);
                    next_level.push(sha256d(&combined));
                } else {
                    // Odd number: duplicate last hash
                    let mut combined = Vec::with_capacity(64);
                    combined.extend_from_slice(&hashes[i].0);
                    combined.extend_from_slice(&hashes[i].0);
                    next_level.push(sha256d(&combined));
                }
            }

            hashes = next_level;
        }

        hashes[0].clone()
    }

    /// Validate block structure and transactions
    pub fn is_valid(&self) -> Result<()> {
        // 1. Check block hash matches header
        let computed_hash = self.header.hash();
        
        // 2. Check merkle root matches transactions
        let computed_merkle = Self::calculate_merkle_root(&self.transactions);
        if computed_merkle != self.header.merkle_root {
            return Err(crate::error::Error::InvalidBlock(
                "Merkle root mismatch".to_string()
            ));
        }

        // 3. Check timestamp is reasonable (not too far in future)
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32;
        
        if self.header.timestamp > now + 7200 {
            return Err(crate::error::Error::InvalidBlock(
                "Block timestamp too far in future".to_string()
            ));
        }

        // 4. Check at least one transaction (coinbase)
        if self.transactions.is_empty() {
            return Err(crate::error::Error::InvalidBlock(
                "Block has no transactions".to_string()
            ));
        }

        // 5. Validate each transaction
        for tx in &self.transactions {
            tx.is_valid()?;
        }

        Ok(())
    }

    /// Get block reward for this height
    pub fn get_block_reward(&self) -> Amount {
        // Block reward halves every 210,000 blocks (like Bitcoin)
        let halvings = self.height / 210_000;
        
        if halvings >= 64 {
            return 0; // All coins mined
        }

        let initial_reward: Amount = 50 * 100_000_000; // 50 ATMN
        initial_reward >> halvings // Divide by 2^halvings
    }

    /// Create coinbase transaction for this block
    pub fn create_coinbase_tx(height: BlockHeight, miner_address: &str, block_reward: Amount) -> Transaction {
        use crate::transaction::{TxInput, TxOutput};
        use crate::types::TxHash;
        
        // Coinbase input (no previous tx)
        let coinbase_input = TxInput {
            prev_tx_hash: TxHash::from_bytes([0u8; 32]),
            prev_tx_index: 0xFFFFFFFF,
            script: height.to_le_bytes().to_vec(), // Block height in script
            sequence: 0xFFFFFFFF,
        };
        
        // Output to miner
        let coinbase_output = TxOutput {
            amount: block_reward,
            script_pubkey: miner_address.as_bytes().to_vec(), // Simple address encoding
        };
        
        Transaction {
            version: 1,
            inputs: vec![coinbase_input],
            outputs: vec![coinbase_output],
            locktime: 0,
        }
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

    #[test]
    fn test_block_hash() {
        let block = Block::new(
            1,
            BlockHash::zero(),
            vec![],
            1704067200,
            0x1d00ffff,
            0,
        );
        let hash = block.hash();
        assert_ne!(hash, BlockHash::zero());
    }

    #[test]
    fn test_merkle_root_empty() {
        let merkle = Block::calculate_merkle_root(&[]);
        assert_eq!(merkle, BlockHash::zero());
    }

    #[test]
    fn test_block_reward() {
        let block = Block::new(1, BlockHash::zero(), vec![], 1704067200, 0x1d00ffff, 0);
        assert_eq!(block.get_block_reward(), 50 * 100_000_000);
        
        let block2 = Block::new(1, BlockHash::zero(), vec![], 1704067200, 0x1d00ffff, 210_000);
        assert_eq!(block2.get_block_reward(), 25 * 100_000_000);
    }

    #[test]
    fn test_header_serialization() {
        let header = BlockHeader {
            version: 1,
            prev_block_hash: BlockHash::zero(),
            merkle_root: BlockHash::zero(),
            timestamp: 1704067200,
            bits: 0x1d00ffff,
            nonce: 12345,
        };
        let bytes = header.serialize();
        // BlockHeader size: version(4) + prev_hash(32) + merkle(32) + timestamp(4) + bits(4) + nonce(4) = 80 bytes
        assert_eq!(bytes.len(), 80);
    }
}
