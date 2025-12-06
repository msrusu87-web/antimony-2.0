// atmn-core/bin/verify_blocks.rs
// Utility to verify blocks in RocksDB storage

use atmn_core::Storage;
use std::env;

fn main() -> anyhow::Result<()> {
    let db_path = env::var("DB_PATH").unwrap_or_else(|_| "./data/atmn-miner.db".to_string());
    
    println!("📦 Opening database: {}", db_path);
    let storage = Storage::new(&db_path)?;
    
    let best_height = storage.get_best_height()?;
    
    match best_height {
        Some(height) => {
            println!("✅ Best block height: {}", height);
            println!();
            
            // Show last 5 blocks
            let start = if height >= 5 { height - 4 } else { 0 };
            println!("Last {} blocks:", (height - start + 1));
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            
            for h in start..=height {
                match storage.get_block(h)? {
                    Some(block) => {
                        println!("Block #{}: hash={:?}", h, block.hash());
                        println!("  └─ timestamp: {}, nonce: {}", block.header.timestamp, block.header.nonce);
                    }
                    None => {
                        println!("Block #{}: NOT FOUND", h);
                    }
                }
            }
        }
        None => {
            println!("⚠️  No blocks found in database");
        }
    }
    
    Ok(())
}
