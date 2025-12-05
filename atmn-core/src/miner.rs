// atmn-core/src/miner.rs
// Proof-of-Work Mining System for ATMN

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::block::{Block, BlockHeader};
use crate::transaction::Transaction;
use crate::consensus::{sha256d, ProofOfWork};
use crate::types::{BlockHash, Nonce, Timestamp};
use crate::error::Result;

/// Mining configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinerConfig {
    /// Maximum nonce to try before giving up
    pub max_nonce: u32,
    /// Thread count for parallel mining
    pub thread_count: usize,
    /// Update mining template every N milliseconds
    pub update_interval_ms: u64,
}

impl Default for MinerConfig {
    fn default() -> Self {
        MinerConfig {
            max_nonce: u32::MAX,
            thread_count: num_cpus::get(),
            update_interval_ms: 1000,
        }
    }
}

/// Mining statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningStats {
    /// Hashes computed
    pub hashes_computed: u64,
    /// Hash rate (hashes per second)
    pub hash_rate: f64,
    /// Blocks found
    pub blocks_found: u64,
    /// Last block found timestamp
    pub last_block_time: Option<Timestamp>,
    /// Current difficulty bits
    pub difficulty_bits: u32,
}

impl Default for MiningStats {
    fn default() -> Self {
        MiningStats {
            hashes_computed: 0,
            hash_rate: 0.0,
            blocks_found: 0,
            last_block_time: None,
            difficulty_bits: 0x1d00ffff,
        }
    }
}

/// Block template for mining
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockTemplate {
    /// Previous block hash
    pub prev_block_hash: BlockHash,
    /// Merkle root of transactions
    pub merkle_root: BlockHash,
    /// Block height
    pub height: u64,
    /// Transactions to include
    pub transactions: Vec<Transaction>,
    /// Current difficulty target (bits format)
    pub difficulty_bits: u32,
    /// Version for this block
    pub version: u32,
    /// Template timestamp
    pub template_time: Timestamp,
}

/// Result of mining attempt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningResult {
    /// Found valid block
    pub block: Option<Block>,
    /// Hashes attempted
    pub hashes_attempted: u32,
    /// Success
    pub success: bool,
}

/// Main Miner struct
#[derive(Debug)]
pub struct Miner {
    config: MinerConfig,
    stats: MiningStats,
}

impl Miner {
    /// Create new miner with default config
    pub fn new() -> Self {
        Miner {
            config: MinerConfig::default(),
            stats: MiningStats::default(),
        }
    }

    /// Create new miner with custom config
    pub fn with_config(config: MinerConfig) -> Self {
        Miner {
            config,
            stats: MiningStats::default(),
        }
    }

    /// Get current mining stats
    pub fn stats(&self) -> MiningStats {
        self.stats.clone()
    }

    /// Mine a block template
    /// Returns (block, hashes_tried) if successful
    pub fn mine_block(&mut self, template: BlockTemplate) -> Result<MiningResult> {
        let pow = ProofOfWork::new(template.difficulty_bits);
        let target = pow.target;

        // Build block header
        let header_base = BlockHeader {
            version: template.version,
            prev_block_hash: template.prev_block_hash.clone(),
            merkle_root: template.merkle_root.clone(),
            timestamp: current_timestamp(),
            bits: template.difficulty_bits,
            nonce: 0,
        };

        let mut nonce: Nonce = 0;
        let mut hashes: u64 = 0;
        let start_time = SystemTime::now();

        // Iterate through nonce values until valid proof-of-work found
        while nonce < self.config.max_nonce {
            // Create header with current nonce
            let mut current_header = header_base.clone();
            current_header.nonce = nonce;

            // Hash the header
            let header_bytes = serialize_block_header(&current_header)?;
            let block_hash = sha256d(&header_bytes);

            hashes += 1;

            // Check if hash meets difficulty target
            if verify_hash_difficulty(&block_hash, &target) {
                // Found valid proof-of-work!
                let block = Block {
                    header: current_header,
                    transactions: template.transactions.clone(),
                    height: template.height,
                };

                // Update stats
                self.stats.blocks_found += 1;
                self.stats.last_block_time = Some(current_timestamp());

                if let Ok(elapsed) = start_time.elapsed() {
                    let secs = elapsed.as_secs_f64();
                    if secs > 0.0 {
                        self.stats.hash_rate = hashes as f64 / secs;
                    }
                }

                return Ok(MiningResult {
                    block: Some(block),
                    hashes_attempted: (hashes & 0xFFFFFFFF) as u32,
                    success: true,
                });
            }

            nonce += 1;

            // Periodically update hash rate
            if nonce % 1_000_000 == 0 {
                if let Ok(elapsed) = start_time.elapsed() {
                    let secs = elapsed.as_secs_f64();
                    if secs > 0.0 {
                        self.stats.hash_rate = hashes as f64 / secs;
                    }
                }
            }
        }

        // Max nonce reached without finding valid block
        self.stats.hashes_computed += hashes;

        if let Ok(elapsed) = start_time.elapsed() {
            let secs = elapsed.as_secs_f64();
            if secs > 0.0 {
                self.stats.hash_rate = hashes as f64 / secs;
            }
        }

        Ok(MiningResult {
            block: None,
            hashes_attempted: (hashes & 0xFFFFFFFF) as u32,
            success: false,
        })
    }

    /// Verify that a block has valid proof-of-work
    pub fn verify_block_pow(block: &Block, difficulty_bits: u32) -> Result<bool> {
        let pow = ProofOfWork::new(difficulty_bits);
        let target = pow.target;

        // Serialize header and hash
        let header_bytes = serialize_block_header(&block.header)?;
        let block_hash = sha256d(&header_bytes);

        Ok(verify_hash_difficulty(&block_hash, &target))
    }

    /// Get difficulty adjustment for next block
    pub fn difficulty_for_next_block(
        blocks: &[Block],
        current_bits: u32,
    ) -> Result<u32> {
        if blocks.len() < 2 {
            return Ok(current_bits);
        }

        // Check if this is a difficulty adjustment block
        let last_block = &blocks[blocks.len() - 1];
        if last_block.height % 2016 != 0 {
            return Ok(current_bits);
        }

        // Get first and last block of adjustment period
        let first_index = if blocks.len() >= 2016 {
            blocks.len() - 2016
        } else {
            0
        };

        let first_block = &blocks[first_index];
        let last_block = &blocks[blocks.len() - 1];

        // Calculate time span
        let time_span = if last_block.header.timestamp > first_block.header.timestamp {
            last_block.header.timestamp - first_block.header.timestamp
        } else {
            1 // Prevent division by zero
        };

        // Target timespan is 2 weeks (2016 blocks * 10 minutes per block)
        const TARGET_TIMESPAN: u32 = 2 * 7 * 24 * 60 * 60; // 1,209,600 seconds
        const MIN_ADJUSTMENT: u32 = TARGET_TIMESPAN / 4;
        const MAX_ADJUSTMENT: u32 = TARGET_TIMESPAN * 4;

        // Adjust difficulty
        let adjusted_timespan = if time_span < MIN_ADJUSTMENT {
            MIN_ADJUSTMENT
        } else if time_span > MAX_ADJUSTMENT {
            MAX_ADJUSTMENT
        } else {
            time_span
        };

        // new_difficulty = old_difficulty * time_span / target_timespan
        let new_bits = calculate_new_difficulty(current_bits, adjusted_timespan)?;

        Ok(new_bits)
    }
}

impl Default for Miner {
    fn default() -> Self {
        Self::new()
    }
}

/// Serialize block header for hashing
fn serialize_block_header(header: &BlockHeader) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();

    // Version (4 bytes, little-endian)
    bytes.extend_from_slice(&header.version.to_le_bytes());

    // Previous block hash (32 bytes)
    bytes.extend_from_slice(&header.prev_block_hash.0);

    // Merkle root (32 bytes)
    bytes.extend_from_slice(&header.merkle_root.0);

    // Timestamp (8 bytes, little-endian)
    bytes.extend_from_slice(&header.timestamp.to_le_bytes());

    // Difficulty bits (4 bytes, little-endian)
    bytes.extend_from_slice(&header.bits.to_le_bytes());

    // Nonce (4 bytes, little-endian)
    bytes.extend_from_slice(&header.nonce.to_le_bytes());

    Ok(bytes)
}

/// Check if a hash meets the target difficulty
fn verify_hash_difficulty(hash: &BlockHash, target: &[u8; 32]) -> bool {
    // Hash should be less than target
    // Hashes are compared as little-endian numbers
    hash.0 <= *target
}

/// Calculate new difficulty from timespan
fn calculate_new_difficulty(bits: u32, timespan: u32) -> Result<u32> {
    const TARGET_TIMESPAN: u32 = 2 * 7 * 24 * 60 * 60; // 1,209,600 seconds

    // Parse difficulty from bits
    let size = (bits >> 24) as usize;
    if size == 0 || size > 32 {
        return Ok(bits);
    }

    let mut target = [0u8; 32];
    let mantissa = (bits & 0xffffff) as usize;

    if size <= 3 {
        target[0] = (mantissa >> 8) as u8;
        target[1] = (mantissa & 0xff) as u8;
    } else {
        for i in 0..3 {
            target[32 - size + 3 - i] = ((mantissa >> (8 * i)) & 0xff) as u8;
        }
    }

    // Multiply target by timespan / target_timespan
    // new_target = old_target * timespan / target_timespan
    let ratio = timespan as f64 / TARGET_TIMESPAN as f64;
    
    // Convert target to integer
    let mut target_int = 0u128;
    for &byte in target.iter() {
        target_int = (target_int << 8) | (byte as u128);
    }

    // Apply ratio
    let new_target_int = (target_int as f64 * ratio) as u128;

    // Convert back to bytes
    let mut new_target = [0u8; 32];
    let mut value = new_target_int;
    for i in (0..32).rev() {
        new_target[i] = (value & 0xff) as u8;
        value >>= 8;
    }

    // Convert target back to bits
    let mut new_size = 32;
    for i in (0..32).rev() {
        if new_target[i] != 0 {
            new_size = i + 1;
            break;
        }
    }

    if new_size == 0 {
        return Ok(0);
    }

    let mut new_bits = if new_size <= 3 {
        (new_target[new_size - 1] as u32) << 8
    } else {
        ((new_target[new_size - 1] as u32) << 16)
            | ((new_target[new_size - 2] as u32) << 8)
            | (new_target[new_size - 3] as u32)
    };

    if (new_bits & 0x00800000) != 0 {
        new_bits >>= 8;
        new_bits += 1;
    }

    Ok(((new_size as u32) << 24) | (new_bits & 0x00ffffff))
}

/// Get current timestamp
fn current_timestamp() -> Timestamp {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_miner_creation() {
        let miner = Miner::new();
        assert_eq!(miner.stats.blocks_found, 0);
        assert_eq!(miner.stats.hashes_computed, 0);
    }

    #[test]
    fn test_miner_with_config() {
        let config = MinerConfig {
            max_nonce: 100_000,
            thread_count: 4,
            update_interval_ms: 500,
        };
        let miner = Miner::with_config(config);
        assert_eq!(miner.config.max_nonce, 100_000);
        assert_eq!(miner.config.thread_count, 4);
    }

    #[test]
    fn test_current_timestamp() {
        let ts = current_timestamp();
        assert!(ts > 1704067200); // After 2024-01-01
        assert!(ts < 2000000000); // Before 2033
    }

    #[test]
    fn test_serialize_block_header() {
        let header = BlockHeader {
            version: 1,
            prev_block_hash: BlockHash::zero(),
            merkle_root: BlockHash::zero(),
            timestamp: 1704067200,
            bits: 0x1d00ffff,

            nonce: 0,
        };
        let bytes = serialize_block_header(&header).unwrap();
        assert_eq!(bytes.len(), 80); // 4+32+32+4+4+4 = 80 bytes
    }
}

/// Create coinbase transaction for block reward
pub fn create_coinbase_transaction(
    miner_pubkey_script: Vec<u8>,
    block_height: u64,
    additional_fees: u64,
) -> Transaction {
    use crate::transaction::{TxInput, TxOutput};
    use crate::types::TxHash;
    
    // Block reward: 50 ATMN = 50 * 100_000_000 satoshis
    const BLOCK_REWARD_SATOSHIS: u64 = 50 * 100_000_000;
    
    // Total reward = block reward + transaction fees
    let total_reward = BLOCK_REWARD_SATOSHIS + additional_fees;
    
    // Coinbase input (prev_tx_hash is zero, script contains block height)
    let coinbase_input = TxInput {
        prev_tx_hash: TxHash::from_bytes([0u8; 32]),
        prev_tx_index: 0xFFFFFFFF,
        script: block_height.to_le_bytes().to_vec(),
        sequence: 0xFFFFFFFF,
    };
    
    // Coinbase output (reward to miner)
    let coinbase_output = TxOutput {
        amount: total_reward,
        script_pubkey: miner_pubkey_script,
    };
    
    Transaction {
        version: 1,
        inputs: vec![coinbase_input],
        outputs: vec![coinbase_output],
        locktime: 0,
    }
}

/// Calculate merkle root from transactions
pub fn calculate_merkle_root(transactions: &[Transaction]) -> Result<BlockHash> {
    use crate::error::Error;
    
    if transactions.is_empty() {
        return Ok(BlockHash::zero());
    }
    
    // Serialize and hash each transaction
    let mut hashes: Vec<BlockHash> = transactions
        .iter()
        .map(|tx| {
            let tx_bytes = bincode::serialize(tx)
                .map_err(|_| Error::SerializationError)?;
            Ok(sha256d(&tx_bytes))
        })
        .collect::<Result<Vec<BlockHash>>>()?;
    
    // Build merkle tree
    while hashes.len() > 1 {
        let mut next_level = Vec::new();
        
        for i in (0..hashes.len()).step_by(2) {
            let left = &hashes[i];
            let right = if i + 1 < hashes.len() {
                &hashes[i + 1]
            } else {
                left // Duplicate last hash if odd number
            };
            
            // Concatenate and hash
            let mut combined = Vec::new();
            combined.extend_from_slice(&left.0);
            combined.extend_from_slice(&right.0);
            next_level.push(sha256d(&combined));
        }
        
        hashes = next_level;
    }
    
    Ok(hashes[0].clone())
}

#[cfg(test)]
mod coinbase_tests {
    use super::*;
    use crate::transaction::{TxInput, TxOutput};
    use crate::types::TxHash;
    
    #[test]
    fn test_create_coinbase_transaction() {
        let miner_script = vec![0x76, 0xa9, 0x14]; // OP_DUP OP_HASH160 ...
        let coinbase = create_coinbase_transaction(miner_script.clone(), 100, 5000);
        
        assert_eq!(coinbase.version, 1);
        assert_eq!(coinbase.inputs.len(), 1);
        assert_eq!(coinbase.outputs.len(), 1);
        assert!(coinbase.is_coinbase());
        assert_eq!(coinbase.outputs[0].amount, 5000005000); // 50 ATMN + 5000 sats fee
        assert_eq!(coinbase.outputs[0].script_pubkey, miner_script);
    }
    
    #[test]
    fn test_merkle_root_single_tx() {
        let tx = Transaction {
            version: 1,
            inputs: vec![],
            outputs: vec![],
            locktime: 0,
        };
        
        let merkle_root = calculate_merkle_root(&[tx]).unwrap();
        assert_ne!(merkle_root, BlockHash::zero());
    }
    
    #[test]
    fn test_merkle_root_multiple_txs() {
        let tx1 = Transaction {
            version: 1,
            inputs: vec![],
            outputs: vec![TxOutput {
                amount: 1000000,
                script_pubkey: vec![],
            }],
            locktime: 0,
        };
        
        let tx2 = Transaction {
            version: 1,
            inputs: vec![],
            outputs: vec![TxOutput {
                amount: 2000000,
                script_pubkey: vec![],
            }],
            locktime: 0,
        };
        
        let merkle_root = calculate_merkle_root(&[tx1, tx2]).unwrap();
        assert_ne!(merkle_root, BlockHash::zero());
    }
    
    #[test]
    fn test_merkle_root_empty() {
        let merkle_root = calculate_merkle_root(&[]).unwrap();
        assert_eq!(merkle_root, BlockHash::zero());
    }
    
    #[test]
    fn test_coinbase_block_height() {
        let miner_script = vec![0x76];
        let coinbase = create_coinbase_transaction(miner_script, 12345, 0);
        
        // Block height should be in the input script
        let height_bytes = 12345u64.to_le_bytes();
        assert_eq!(coinbase.inputs[0].script, height_bytes.to_vec());
    }
}
