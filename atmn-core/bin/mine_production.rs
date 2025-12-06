// atmn-core/bin/mine_production.rs
// Production miner that connects to database and mines real blocks

use atmn_core::{MultiThreadedMiner, BlockTemplate, Storage};
use atmn_core::types::BlockHash;
use std::time::Instant;
use std::env;

fn main() -> anyhow::Result<()> {
    // Simple logging setup
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║    ANTIMONY COIN - PRODUCTION MINER (Multi-threaded)       ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    // Get miner address from args or use default
    let args: Vec<String> = env::args().collect();
    let miner_address = if args.len() > 1 {
        args[1].clone()
    } else {
        "ATMN_default_miner_address".to_string()
    };

    // Get thread count from args or use all CPUs
    let thread_count = if args.len() > 2 {
        args[2].parse().unwrap_or(num_cpus::get())
    } else {
        num_cpus::get()
    };

    // Get target blocks from args or mine indefinitely
    let target_blocks = if args.len() > 3 {
        Some(args[3].parse().unwrap_or(100))
    } else {
        None
    };

    println!("⚙️  Configuration:");
    println!("   • Miner Address: {}", miner_address);
    println!("   • CPU Threads: {}", thread_count);
    println!("   • Target Blocks: {}", target_blocks.map_or("∞".to_string(), |n| n.to_string()));
    println!();

    // Open database
    let db_path = env::var("DB_PATH").unwrap_or_else(|_| "./data/atmn-miner.db".to_string());
    let storage = Storage::new(&db_path)?;
    
    println!("📦 Database opened: {}", db_path);
    
    // Get current blockchain height
    let current_height = storage.get_best_height()?.unwrap_or(0);
    println!("   Current blockchain height: {}", current_height);
    println!();

    // Create multi-threaded miner
    let mut miner = MultiThreadedMiner::new(Some(thread_count));
    
    let mut blocks_mined: u64 = 0;
    let total_start = Instant::now();

    loop {
        // Get the latest block hash
        let prev_block_hash = if current_height + blocks_mined == 0 {
            BlockHash::zero()
        } else {
            // Get the last block from storage
            let prev_height = current_height + blocks_mined;
            match storage.get_block(prev_height) {
                Ok(Some(prev_block)) => prev_block.hash(),
                _ => BlockHash::zero(), // Fallback to genesis
            }
        };

        // Create block template
        let next_height = current_height + blocks_mined + 1;
        
        // Create coinbase transaction (50 ATMN = 5,000,000,000 satoshis)
        let block_reward = 5_000_000_000u64; // 50 ATMN
        let coinbase_tx = atmn_core::Block::create_coinbase_tx(next_height, &miner_address, block_reward);
        
        let template = BlockTemplate {
            prev_block_hash,
            merkle_root: BlockHash::zero(), // TODO: Calculate from transactions
            height: next_height,
            transactions: vec![coinbase_tx],
            difficulty_bits: 0x207fffff, // Much easier difficulty for testing (was 0x1d00ffff)
            version: 1,
            template_time: current_timestamp(),
        };

        println!("⛏️  Mining block #{} ...", next_height);
        let block_start = Instant::now();

        // Mine the block
        match miner.mine_block(template) {
            Ok(result) => {
                if result.success && result.block.is_some() {
                    let block = result.block.unwrap();
                    let block_time = block_start.elapsed();
                    
                    // Store the block
                    match storage.put_block(next_height, &block) {
                        Ok(_) => {
                            blocks_mined += 1;
                            
                            println!("✅ Block #{} mined!", next_height);
                            println!("   ├─ Hash: {:?}", block.hash());
                            println!("   ├─ Nonce: {}", block.header.nonce);
                            println!("   ├─ Time: {:.2}s", block_time.as_secs_f64());
                            println!("   ├─ Hashes: {}", result.hashes_attempted);
                            println!("   └─ Hash Rate: {:.2} MH/s", result.hashes_attempted as f64 / block_time.as_secs_f64() / 1_000_000.0);
                            println!();

                            // Check if we've reached target
                            if let Some(target) = target_blocks {
                                if blocks_mined >= target {
                                    break;
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("❌ Error storing block: {:?}", e);
                            break;
                        }
                    }
                } else {
                    println!("❌ Mining failed - no valid block found");
                    break;
                }
            }
            Err(e) => {
                eprintln!("❌ Mining error: {:?}", e);
                break;
            }
        }
    }

    let total_time = total_start.elapsed();
    println!("🎉 Mining session complete!");
    println!();
    println!("📊 Final Statistics:");
    println!("   • Blocks Mined: {}", blocks_mined);
    println!("   • Total Time: {:.2}s", total_time.as_secs_f64());
    println!("   • Average Time per Block: {:.2}s", total_time.as_secs_f64() / blocks_mined as f64);
    println!("   • Final Height: {}", current_height + blocks_mined);
    println!();

    Ok(())
}

fn current_timestamp() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32
}
