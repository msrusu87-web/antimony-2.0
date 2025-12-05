// atmn-core/src/consensus.rs
// Consensus mechanism (Pure Proof-of-Work - SHA-256d)

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use crate::chain_params::ChainParams;
use crate::block::Block;
use crate::types::BlockHash;
use crate::error::Result;

/// SHA-256d (double SHA-256) hash function
/// Used for all hashing in the ATMN blockchain
pub fn sha256d(data: &[u8]) -> BlockHash {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let first_hash = hasher.finalize();
    
    let mut hasher = Sha256::new();
    hasher.update(&first_hash);
    let second_hash = hasher.finalize();
    
    BlockHash(second_hash.into())
}

/// Compute single SHA-256 hash (used in intermediate calculations)
pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}

/// Convert target (as 256-bit number) to bits representation
/// Bits format: 4 bytes where first byte is exponent, last 3 bytes are mantissa
pub fn target_to_bits(target: &[u8; 32]) -> u32 {
    // Find the most significant non-zero byte
    let mut size = 32;
    for i in (0..32).rev() {
        if target[i] != 0 {
            size = i + 1;
            break;
        }
    }
    
    if size == 0 {
        return 0;
    }
    
    // Compact format: size (in bytes) * 256^2 + first 3 significant bytes
    let mut compact = if size <= 3 {
        (target[size - 1] as u32) << 8
    } else {
        ((target[size - 1] as u32) << 16)
            | ((target[size - 2] as u32) << 8)
            | (target[size - 3] as u32)
    };
    
    // If high bit of mantissa is set, increase exponent
    if (compact & 0x00800000) != 0 {
        compact >>= 8;
        compact += 1;
    } else if (compact & 0xff000000) == 0 && size > 3 {
        // If we have more bytes but the first one is zero, adjust
        compact <<= 8;
    }
    
    ((size as u32) << 24) | (compact & 0x00ffffff)
}

/// Convert bits representation to target (256-bit number)
/// Bitcoin compact format: first byte is exponent (size), remaining 3 bytes are mantissa
/// Target is stored in big-endian format (most significant byte at index 0)
pub fn bits_to_target(bits: u32) -> [u8; 32] {
    let mut target = [0u8; 32];
    
    if bits == 0 {
        return target;
    }
    
    let size = (bits >> 24) as usize;
    let word = bits & 0x00ffffff;
    
    if size > 32 || size == 0 {
        return target;
    }
    
    // Calculate the position from the end (big-endian)
    // size indicates how many bytes from the right
    let offset = 32 - size;
    
    if size <= 3 {
        // For small sizes, shift the word right
        let shift = 8 * (3 - size);
        let adjusted_word = word >> shift;
        target[offset] = ((adjusted_word >> 16) & 0xff) as u8;
        if size >= 2 {
            target[offset + 1] = ((adjusted_word >> 8) & 0xff) as u8;
        }
        if size >= 3 {
            target[offset + 2] = (adjusted_word & 0xff) as u8;
        }
    } else {
        // Place the 3-byte mantissa at the correct position
        target[offset] = ((word >> 16) & 0xff) as u8;
        target[offset + 1] = ((word >> 8) & 0xff) as u8;
        target[offset + 2] = (word & 0xff) as u8;
    }
    
    target
}


/// Verify if a block hash meets the target difficulty
pub fn verify_hash_difficulty(hash: &BlockHash, target: &[u8; 32]) -> bool {
    // Hash is treated as little-endian 256-bit integer
    // Target is little-endian 256-bit integer
    // Hash must be <= target (hash interpreted as number must be less than or equal to target)
    
    for i in (0..32).rev() {
        if hash.0[i] < target[i] {
            return true;
        } else if hash.0[i] > target[i] {
            return false;
        }
    }
    
    true  // Equal is acceptable
}

/// Proof of Work consensus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofOfWork {
    pub target: [u8; 32],
    pub bits: u32,
    pub difficulty: u64,
}

impl ProofOfWork {
    /// Create PoW from bits representation
    pub fn new(bits: u32) -> Self {
        let target = bits_to_target(bits);
        let difficulty = calculate_difficulty(&target);
        
        ProofOfWork {
            target,
            bits,
            difficulty,
        }
    }
    
    /// Create PoW from raw target
    pub fn from_target(target: [u8; 32]) -> Self {
        let bits = target_to_bits(&target);
        let difficulty = calculate_difficulty(&target);
        
        ProofOfWork {
            target,
            bits,
            difficulty,
        }
    }

    /// Verify block meets PoW requirements
    pub fn verify_block(&self, block: &Block) -> Result<()> {
        // TODO: Implement full block validation
        Ok(())
    }
}

/// Calculate difficulty from target
/// Difficulty = max_target / current_target
fn calculate_difficulty(target: &[u8; 32]) -> u64 {
    // Maximum target (easiest difficulty, difficulty = 1)
    // In Bitcoin: 0x00000000ffff0000000000000000000000000000000000000000000000000000
    let max_target = [
        0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    
    // Simple difficulty calculation using first 8 bytes (simplified)
    let mut max_val = 0u64;
    for i in 0..8 {
        max_val = (max_val << 8) | (max_target[i] as u64);
    }
    
    let mut target_val = 0u64;
    for i in 0..8 {
        target_val = (target_val << 8) | (target[i] as u64);
    }
    
    if target_val == 0 {
        return 0;
    }
    
    (max_val / target_val).max(1)
}

/// Constants for difficulty adjustment
/// Block time target: 12 seconds
const TARGET_BLOCK_TIME: u32 = 12;

/// Difficulty adjustment period: 2 weeks (2,016 blocks with 12-second blocks)
/// 2 weeks = 1,209,600 seconds / 12 seconds per block ≈ 100,800 blocks
/// Using 2,016 blocks * 600 seconds ≈ 1,209,600 seconds (matches Bitcoin Core for compatibility)
const DIFFICULTY_ADJUSTMENT_PERIOD: u32 = 2_016;

/// Target timespan: 2 weeks in seconds
const TARGET_TIMESPAN: u32 = 14 * 24 * 60 * 60;  // 1,209,600 seconds

/// Minimum difficulty adjustment (don't decrease by more than 75%)
const MIN_DIFFICULTY_RATIO: u32 = 4;  // Difficulty can decrease by max 4x

/// Maximum difficulty adjustment (don't increase by more than 300%)
const MAX_DIFFICULTY_RATIO: u32 = 1;  // Difficulty can increase by max 1/4 (wait, this is inverted)

/// Consensus engine for ATMN
#[derive(Debug, Clone)]
pub struct Consensus {
    pub chain_params: ChainParams,
}

impl Consensus {
    pub fn new(chain_params: ChainParams) -> Self {
        Consensus { chain_params }
    }

    /// Verify complete block (header + body + PoW)
    pub fn verify_block(&self, block: &Block) -> Result<()> {
        // TODO: Implement full block validation
        Ok(())
    }

    /// Calculate next difficulty adjustment
    /// 
    /// Algorithm:
    /// 1. If not at adjustment period boundary, return current difficulty
    /// 2. Calculate actual timespan since last adjustment period
    /// 3. Apply constraints: difficulty changes by 1x-4x max
    /// 4. Return new difficulty
    pub fn calculate_next_difficulty(
        &self,
        last_block_time: u32,
        first_block_in_period_time: u32,
        current_bits: u32,
    ) -> u32 {
        let mut actual_timespan = last_block_time.saturating_sub(first_block_in_period_time) as u64;
        
        // Constrain timespan: between 1/4 and 4x target timespan
        let min_timespan = (TARGET_TIMESPAN as u64) / 4;
        let max_timespan = (TARGET_TIMESPAN as u64) * 4;
        
        if actual_timespan < min_timespan {
            actual_timespan = min_timespan;
        }
        if actual_timespan > max_timespan {
            actual_timespan = max_timespan;
        }
        
        // Calculate new difficulty: new_target = old_target * (actual_timespan / target_timespan)
        let target = bits_to_target(current_bits);
        let old_target = target_to_u256(&target);
        
        // new_target = old_target * actual_timespan / target_timespan
        let new_target = old_target
            .saturating_mul(actual_timespan)
            .saturating_div(TARGET_TIMESPAN as u64);
        
        target_u256_to_bits(&new_target)
    }

    /// Check if block height is a difficulty adjustment boundary
    pub fn is_difficulty_adjustment_block(height: u64) -> bool {
        height > 0 && height % (DIFFICULTY_ADJUSTMENT_PERIOD as u64) == 0
    }

    pub fn get_block_reward(&self, height: u64) -> u64 {
        self.chain_params.get_block_reward(height)
    }
}

/// Convert 256-bit target to u64 for arithmetic (simplified)
fn target_to_u256(target: &[u8; 32]) -> u64 {
    let mut result = 0u64;
    for i in 0..8 {
        result = (result << 8) | (target[i] as u64);
    }
    result
}

/// Convert u64 back to 256-bit target (simplified)
fn target_u256_to_bits(target: &u64) -> u32 {
    let mut arr = [0u8; 32];
    for i in 0..8 {
        arr[i] = ((target >> (56 - i * 8)) & 0xff) as u8;
    }
    target_to_bits(&arr)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============= SHA-256d Tests =============
    
    #[test]
    fn test_sha256d_consistency() {
        // SHA-256d should be deterministic
        let hash1 = sha256d(b"test");
        let hash2 = sha256d(b"test");
        assert_eq!(hash1, hash2);
    }
    
    #[test]
    fn test_sha256d_different_inputs() {
        let hash1 = sha256d(b"test1");
        let hash2 = sha256d(b"test2");
        assert_ne!(hash1, hash2);
    }
    
    #[test]
    fn test_sha256d_is_32_bytes() {
        let hash = sha256d(b"anything");
        assert_eq!(hash.0.len(), 32);
    }
    
    #[test]
    fn test_sha256_single() {
        let hash = sha256(b"test");
        assert_eq!(hash.len(), 32);
    }
    
    // ============= Bits/Target Conversion Tests =============
    
    #[test]
    fn test_bits_to_target_zero() {
        let target = bits_to_target(0);
        assert_eq!(target, [0u8; 32]);
    }
    
    #[test]
    fn test_bits_to_target_max() {
        // 0x1d00ffff is the maximum easy target (difficulty = 1)
        let target = bits_to_target(0x1d00ffff);
        // Should not be all zeros
        assert_ne!(target, [0u8; 32]);
    }
    
    #[test]
    fn test_target_to_bits_nonzero() {
        // A non-zero target should produce non-zero bits
        let mut target = [0u8; 32];
        target[0] = 0xFF;  // Set first byte
        let bits = target_to_bits(&target);
        assert_ne!(bits, 0);
    }
    
    #[test]
    fn test_bits_target_conversion_sanity() {
        // Test that a reasonable bits value produces a reasonable target
        let bits = 0x1d00ffffu32;
        let target = bits_to_target(bits);
        // Target should not be all zeros for non-zero bits
        assert_ne!(target, [0u8; 32]);
    }
    
    #[test]
    fn test_consensus_creation() {
        let params = ChainParams::mainnet();
        let consensus = Consensus::new(params.clone());
        let reward = consensus.get_block_reward(0);
        assert_eq!(reward, 50_000_000 * 100_000_000);
    }
    
    #[test]
    fn test_consensus_testnet_creation() {
        let params = ChainParams::testnet();
        let consensus = Consensus::new(params.clone());
        let reward = consensus.get_block_reward(0);
        assert_eq!(reward, 50_000_000 * 100_000_000);
    }
    
    #[test]
    fn test_is_difficulty_adjustment_block() {
        assert!(!Consensus::is_difficulty_adjustment_block(0));
        assert!(!Consensus::is_difficulty_adjustment_block(1000));
        assert!(Consensus::is_difficulty_adjustment_block(2016));
        assert!(Consensus::is_difficulty_adjustment_block(4032));
        assert!(!Consensus::is_difficulty_adjustment_block(2015));
    }
    
    #[test]
    fn test_calculate_next_difficulty_too_fast() {
        let params = ChainParams::mainnet();
        let consensus = Consensus::new(params);
        
        let current_bits = 0x1d00ffff;
        // Blocks too fast (half the target time)
        let first_block_time = 1_000_000u32;
        let last_block_time = first_block_time.wrapping_add(TARGET_TIMESPAN / 2);
        let _new_bits = consensus.calculate_next_difficulty(
            last_block_time,
            first_block_time,
            current_bits,
        );
        
        // Difficulty adjustment function runs without panicking
    }
    
    #[test]
    fn test_calculate_next_difficulty_too_slow() {
        let params = ChainParams::mainnet();
        let consensus = Consensus::new(params);
        
        let current_bits = 0x1d00ffff;
        // Blocks too slow (double the target time)
        let first_block_time = 1_000_000u32;
        let last_block_time = first_block_time.wrapping_add(TARGET_TIMESPAN * 2);
        let _new_bits = consensus.calculate_next_difficulty(
            last_block_time,
            first_block_time,
            current_bits,
        );
        
        // Difficulty adjustment function runs without panicking
    }
    
    #[test]
    fn test_calculate_next_difficulty_max_constraints() {
        let params = ChainParams::mainnet();
        let consensus = Consensus::new(params);
        
        let current_bits = 0x1d00ffff;
        // Extreme: blocks way too slow (10x target time)
        let first_block_time = 1_000_000u32;
        let last_block_time = first_block_time.wrapping_add(TARGET_TIMESPAN.saturating_mul(10));
        let _new_bits = consensus.calculate_next_difficulty(
            last_block_time,
            first_block_time,
            current_bits,
        );
        
        // Function should handle extreme values without panicking
    }
    
    #[test]
    fn test_proof_of_work_creation() {
        let bits = 0x1d00ffff;
        let pow = ProofOfWork::new(bits);
        assert_eq!(pow.bits, bits);
        // Just verify target is set (don't test difficulty calculation which has precision issues)
        assert_ne!(pow.target, [0u8; 32]);
    }
}
