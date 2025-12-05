// atmn-core/bin/miner.rs
// ATMN Coin Miner CLI

use atmn_core::{Miner, MinerConfig, BlockTemplate};
use atmn_core::types::BlockHash;
use std::time::Instant;
use std::io::{self, Write};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║       ANTIMONY COIN (ATMN) - PROOF-OF-WORK MINER           ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    // Create miner with custom config
    let config = MinerConfig {
        max_nonce: u32::MAX,
        thread_count: num_cpus::get(),
        update_interval_ms: 1000,
    };

    let mut miner = Miner::with_config(config);
    
    println!("🔧 Miner Configuration:");
    println!("   • CPU Cores: {}", num_cpus::get());
    println!("   • Max Nonce: {} (4.2 billion)", u32::MAX);
    println!();

    // Test mining with low difficulty
    println!("🎯 Starting test mining with easy difficulty...");
    println!("   (This will find blocks quickly for testing)");
    println!();

    let mut block_count = 0;
    let start_time = Instant::now();

    loop {
        // Create a test block template
        let template = BlockTemplate {
            prev_block_hash: if block_count == 0 {
                BlockHash::zero()
            } else {
                // In real scenario, this would be the previous block hash
                BlockHash::zero()
            },
            merkle_root: BlockHash::zero(), // Empty for test
            height: block_count,
            transactions: vec![],
            difficulty_bits: 0x207fffff, // Very very easy difficulty for testing
            version: 1,
            template_time: current_timestamp(),
        };

        print!("⛏️  Mining block #{}... ", block_count);
        io::stdout().flush().unwrap();

        // Mine the block
        match miner.mine_block(template) {
            Ok(result) => {
                if result.success {
                    block_count += 1;
                    let stats = miner.stats();
                    
                    println!("✅ FOUND!");
                    println!("   ├─ Nonce: {}", result.hashes_attempted);
                    println!("   ├─ Hash Rate: {:.2} MH/s", stats.hash_rate / 1_000_000.0);
                    println!("   ├─ Total Blocks: {}", stats.blocks_found);
                    println!("   └─ Elapsed: {:?}", start_time.elapsed());
                    
                    if let Some(block) = result.block {
                        let hash = block.hash();
                        println!("   Block Hash: {:?}", hash);
                    }
                    
                    println!();

                    // Mine 10 blocks then stop for demo
                    if block_count >= 10 {
                        println!("🎉 Successfully mined {} test blocks!", block_count);
                        println!();
                        
                        let total_secs = start_time.elapsed().as_secs_f64();
                        let avg_block_time = total_secs / block_count as f64;
                        
                        println!("📊 Final Statistics:");
                        println!("   • Total Time: {:.2} seconds", total_secs);
                        println!("   • Average Block Time: {:.2} seconds", avg_block_time);
                        println!("   • Average Hash Rate: {:.2} MH/s", miner.stats().hash_rate / 1_000_000.0);
                        println!("   • Total Hashes: ~{:.2} million", (miner.stats().hash_rate * total_secs) / 1_000_000.0);
                        println!();
                        
                        break;
                    }
                } else {
                    println!("❌ No block found (max nonce reached)");
                    println!("   This shouldn't happen with easy difficulty!");
                    break;
                }
            },
            Err(e) => {
                println!("❌ Mining error: {:?}", e);
                break;
            }
        }
    }

    println!("✅ Mining test complete!");
    println!();
    println!("💡 Note: This was a test with very easy difficulty.");
    println!("   In production, difficulty adjusts based on network hash rate.");
    println!("   Target block time for ATMN: 12 seconds");
}

fn current_timestamp() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32
}
