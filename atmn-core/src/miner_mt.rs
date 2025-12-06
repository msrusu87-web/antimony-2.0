// Multi-threaded mining implementation for ATMN
// Improves mining performance by 2-4x through parallel nonce search

use std::sync::{Arc, atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering}};
use std::thread;
use std::time::{SystemTime, Duration};
use crate::block::{Block, BlockHeader};
use crate::miner::{BlockTemplate, MiningResult};
use crate::consensus::sha256d;
use crate::error::Result;

/// Multi-threaded miner for improved performance
pub struct MultiThreadedMiner {
    thread_count: usize,
    stop_flag: Arc<AtomicBool>,
    hashes_computed: Arc<AtomicU64>,
}

impl MultiThreadedMiner {
    pub fn new(thread_count: Option<usize>) -> Self {
        let thread_count = thread_count.unwrap_or_else(num_cpus::get);
        
        MultiThreadedMiner {
            thread_count,
            stop_flag: Arc::new(AtomicBool::new(false)),
            hashes_computed: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Mine a block using multiple threads
    /// Each thread searches a different range of nonces
    pub fn mine_block(&mut self, template: BlockTemplate) -> Result<MiningResult> {
        self.stop_flag.store(false, Ordering::Relaxed);
        self.hashes_computed.store(0, Ordering::Relaxed);

        let start_time = SystemTime::now();
        
        // Divide nonce range across threads
        let nonce_range_per_thread = u32::MAX / self.thread_count as u32;
        
        // Shared result holder
        let found_block: Arc<std::sync::Mutex<Option<Block>>> = Arc::new(std::sync::Mutex::new(None));
        let found_nonce = Arc::new(AtomicU32::new(0));
        
        // Spawn mining threads
        let mut handles = vec![];
        
        for thread_id in 0..self.thread_count {
            let template_clone = template.clone();
            let stop_flag = Arc::clone(&self.stop_flag);
            let hashes_computed = Arc::clone(&self.hashes_computed);
            let found_block_clone = Arc::clone(&found_block);
            let found_nonce_clone = Arc::clone(&found_nonce);
            
            let start_nonce = thread_id as u32 * nonce_range_per_thread;
            let end_nonce = if thread_id == self.thread_count - 1 {
                u32::MAX
            } else {
                (thread_id + 1) as u32 * nonce_range_per_thread
            };
            
            let handle = thread::spawn(move || {
                Self::mine_thread(
                    template_clone,
                    start_nonce,
                    end_nonce,
                    stop_flag,
                    hashes_computed,
                    found_block_clone,
                    found_nonce_clone,
                )
            });
            
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().ok();
        }
        
        // Check if we found a block
        let block_option = found_block.lock().unwrap().take();
        let total_hashes = self.hashes_computed.load(Ordering::Relaxed);
        
        // Calculate hash rate
        let elapsed = start_time.elapsed().unwrap_or(Duration::from_secs(1));
        let hash_rate = total_hashes as f64 / elapsed.as_secs_f64();
        
        log::info!("Mining complete: {} hashes, {:.2} H/s with {} threads",
                  total_hashes, hash_rate, self.thread_count);
        
        Ok(MiningResult {
            block: block_option.clone(),
            hashes_attempted: (total_hashes as u32),
            success: block_option.is_some(),
        })
    }
    
    /// Thread worker function
    fn mine_thread(
        template: BlockTemplate,
        start_nonce: u32,
        end_nonce: u32,
        stop_flag: Arc<AtomicBool>,
        hashes_computed: Arc<AtomicU64>,
        found_block: Arc<std::sync::Mutex<Option<Block>>>,
        found_nonce: Arc<AtomicU32>,
    ) {
        let mut header = BlockHeader {
            version: template.version,
            prev_block_hash: template.prev_block_hash,
            merkle_root: template.merkle_root,
            timestamp: template.template_time as u32,
            bits: template.difficulty_bits,
            nonce: start_nonce,
        };
        
        // Calculate target from difficulty bits
        let target = Self::bits_to_target(template.difficulty_bits);
        let mut local_hashes: u64 = 0;
        
        for nonce in start_nonce..end_nonce {
            // Check if another thread found a solution
            if stop_flag.load(Ordering::Relaxed) {
                break;
            }
            
            header.nonce = nonce;
            
            // Hash the header
            let header_bytes = header.serialize();
            let block_hash = sha256d(&header_bytes);
            
            local_hashes += 1;
            
            // Check if hash meets difficulty
            if Self::hash_meets_target(&block_hash, &target) {
                // Found valid block!
                let block = Block {
                    header: header.clone(),
                    transactions: template.transactions.clone(),
                    height: template.height,
                };
                
                // Store result atomically
                *found_block.lock().unwrap() = Some(block);
                found_nonce.store(nonce, Ordering::Relaxed);
                stop_flag.store(true, Ordering::Relaxed);
                
                log::info!("Thread found valid block at nonce {}", nonce);
                break;
            }
            
            // Update global hash counter periodically
            if local_hashes % 10_000 == 0 {
                hashes_computed.fetch_add(10_000, Ordering::Relaxed);
                local_hashes = 0;
            }
        }
        
        // Add remaining hashes
        if local_hashes > 0 {
            hashes_computed.fetch_add(local_hashes, Ordering::Relaxed);
        }
    }
    
    /// Convert difficulty bits to 256-bit target
    fn bits_to_target(bits: u32) -> [u8; 32] {
        let exponent = ((bits >> 24) & 0xff) as usize;
        let mantissa = bits & 0x00ffffff;
        
        let mut target = [0u8; 32];
        if exponent <= 3 {
            let mantissa = mantissa >> (8 * (3 - exponent));
            target[29] = (mantissa >> 16) as u8;
            target[30] = (mantissa >> 8) as u8;
            target[31] = mantissa as u8;
        } else {
            let mantissa_bytes = mantissa.to_be_bytes();
            let offset = 32 - exponent;
            if offset < 29 {
                target[offset] = mantissa_bytes[1];
                target[offset + 1] = mantissa_bytes[2];
                target[offset + 2] = mantissa_bytes[3];
            }
        }
        
        target
    }
    
    /// Check if hash meets target difficulty
    fn hash_meets_target(hash: &crate::types::BlockHash, target: &[u8; 32]) -> bool {
        for i in 0..32 {
            if hash.0[i] < target[i] {
                return true;
            }
            if hash.0[i] > target[i] {
                return false;
            }
        }
        true
    }
    
    /// Stop mining (can be called from another thread)
    pub fn stop(&self) {
        self.stop_flag.store(true, Ordering::Relaxed);
    }
    
    /// Get current hash rate
    pub fn hash_rate(&self, elapsed_secs: f64) -> f64 {
        let hashes = self.hashes_computed.load(Ordering::Relaxed);
        if elapsed_secs > 0.0 {
            hashes as f64 / elapsed_secs
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::BlockHash;
    
    #[test]
    fn test_multi_threaded_miner_creation() {
        let miner = MultiThreadedMiner::new(Some(4));
        assert_eq!(miner.thread_count, 4);
    }
    
    #[test]
    fn test_bits_to_target() {
        let target = MultiThreadedMiner::bits_to_target(0x1d00ffff);
        // Verify target has correct format
        assert!(target[0..3].iter().all(|&x| x == 0));
    }
}
