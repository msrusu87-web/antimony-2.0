// atmn-core/bin/mine_to_height.rs
// Continuous miner with difficulty adjustment monitoring

use atmn_core::{Block, Storage, MultiThreadedMiner, BlockTemplate};
use atmn_core::types::BlockHash;
use std::time::{SystemTime, UNIX_EPOCH, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_height: u64 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(2016);
    
    let db_path = "./data/atmn-miner.db";
    let miner_address = "ATMN_1e6df34f5f50ff6b581b827c5e9dc5b5b787e178";
    
    println!("🎯 Mining to height {}", target_height);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let storage = Storage::new(db_path)?;
    let mut miner = MultiThreadedMiner::new(Some(6));
    
    let start_height = storage.get_best_height()?.unwrap_or(0);
    let start_time = Instant::now();
    
    println!("📊 Start height: {}", start_height);
    println!("📊 Blocks to mine: {}", target_height - start_height);
    println!("📊 Difficulty: 0x207fffff (testing)");
    println!();
    
    let mut blocks_mined = 0;
    let mut last_difficulty_bits = 0x207fffff;
    
    loop {
        let current_height = storage.get_best_height()?.unwrap_or(0);
        
        if current_height >= target_height {
            break;
        }
        
        let next_height = current_height + 1;
        
        // Get previous block hash
        let prev_block_hash = if current_height == 0 {
            BlockHash::zero()
        } else {
            match storage.get_block(current_height) {
                Ok(Some(prev_block)) => prev_block.hash(),
                _ => BlockHash::zero(),
            }
        };
        
        // Create coinbase transaction
        let block_reward = 5_000_000_000u64; // 50 ATMN
        let coinbase_tx = Block::create_coinbase_tx(next_height, miner_address, block_reward);
        
        // Check for difficulty adjustment
        let difficulty_bits = if next_height % 2016 == 0 && next_height > 0 {
            // Calculate new difficulty
            let adjustment_start = next_height.saturating_sub(2016);
            let start_block = storage.get_block(adjustment_start)?;
            let end_block = storage.get_block(current_height)?;
            
            if let (Some(start), Some(end)) = (start_block, end_block) {
                let actual_time = end.header.timestamp.saturating_sub(start.header.timestamp);
                let target_time = 2016 * 12; // 2016 blocks * 12 seconds
                
                // Calculate adjustment ratio (with 4x bounds)
                let ratio = (actual_time as f64) / (target_time as f64);
                let bounded_ratio = ratio.max(0.25).min(4.0);
                
                // Adjust difficulty (higher bits = easier, lower bits = harder)
                let old_difficulty = last_difficulty_bits as f64;
                let new_difficulty = (old_difficulty * bounded_ratio) as u32;
                
                println!();
                println!("🎉 DIFFICULTY ADJUSTMENT at block {}", next_height);
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                println!("   Actual time:  {}s ({:.1}m)", actual_time, actual_time as f64 / 60.0);
                println!("   Target time:  {}s ({:.1}m)", target_time, target_time as f64 / 60.0);
                println!("   Ratio:        {:.4}x", ratio);
                println!("   Bounded:      {:.4}x", bounded_ratio);
                println!("   Old bits:     0x{:08x}", last_difficulty_bits);
                println!("   New bits:     0x{:08x}", new_difficulty);
                println!();
                
                last_difficulty_bits = new_difficulty;
                new_difficulty
            } else {
                last_difficulty_bits
            }
        } else {
            last_difficulty_bits
        };
        
        // Create block template
        let template = BlockTemplate {
            prev_block_hash,
            merkle_root: BlockHash::zero(),
            height: next_height,
            transactions: vec![coinbase_tx],
            difficulty_bits,
            version: 1,
            template_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as u32,
        };
        
        // Mine the block
        match miner.mine_block(template)? {
            result if result.success && result.block.is_some() => {
                let block = result.block.unwrap();
                storage.put_block(next_height, &block)?;
                blocks_mined += 1;
                
                // Progress report every 10 blocks
                if next_height % 10 == 0 {
                    let elapsed = start_time.elapsed().as_secs();
                    let rate = if elapsed > 0 {
                        blocks_mined as f64 / elapsed as f64
                    } else {
                        0.0
                    };
                    let remaining = target_height - next_height;
                    let eta = if rate > 0.0 {
                        remaining as f64 / rate
                    } else {
                        0.0
                    };
                    
                    println!(
                        "   Block {} mined ({:.2} blocks/sec, {} remaining, ETA: {:.0}s)",
                        next_height, rate, remaining, eta
                    );
                }
            }
            _ => {
                eprintln!("❌ Failed to mine block {}", next_height);
                return Err("Mining failed".into());
            }
        }
    }
    
    let total_time = start_time.elapsed().as_secs();
    let minutes = total_time / 60;
    let seconds = total_time % 60;
    
    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🎉 Mining Complete!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Final height: {}", target_height);
    println!("Blocks mined: {}", blocks_mined);
    println!("Time taken: {}m {}s", minutes, seconds);
    println!("Avg rate: {:.2} blocks/sec", blocks_mined as f64 / total_time as f64);
    
    Ok(())
}
