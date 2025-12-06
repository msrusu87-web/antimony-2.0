// atmn-core/bin/sync_to_sqlite.rs
// Syncs blocks from RocksDB to SQLite for API access

use atmn_core::Storage;
use rusqlite::{Connection, params};
use std::env;

fn main() -> anyhow::Result<()> {
    let rocksdb_path = env::var("ROCKSDB_PATH")
        .unwrap_or_else(|_| "./data/atmn-miner.db".to_string());
    
    let sqlite_path = env::var("SQLITE_PATH")
        .unwrap_or_else(|_| "../atmn-api/atmn.db".to_string());
    
    println!("🔄 Syncing blocks from RocksDB to SQLite");
    println!("   RocksDB: {}", rocksdb_path);
    println!("   SQLite:  {}", sqlite_path);
    println!();
    
    // Open RocksDB
    let storage = Storage::new(&rocksdb_path)?;
    let best_height = storage.get_best_height()?;
    
    match best_height {
        Some(height) => {
            println!("✅ Found {} blocks in RocksDB", height + 1);
            
            // Open SQLite
            let conn = Connection::open(&sqlite_path)?;
            
            // Sync each block
            let mut synced = 0;
            let mut skipped = 0;
            
            for h in 0..=height {
                match storage.get_block(h)? {
                    Some(block) => {
                        let hash_hex = hex::encode(block.hash().0);
                        let prev_hash_hex = hex::encode(block.header.prev_block_hash.0);
                        
                        // Check if block already exists
                        let exists: bool = conn.query_row(
                            "SELECT COUNT(*) > 0 FROM blocks WHERE height = ?",
                            params![h],
                            |row| row.get(0)
                        )?;
                        
                        if !exists {
                            // Insert block (miner_address is unknown from RocksDB, using placeholder)
                            conn.execute(
                                "INSERT INTO blocks (height, hash, prev_hash, miner_address, reward, difficulty, nonce, timestamp, status, confirmations) 
                                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 'confirmed', ?9)",
                                params![
                                    h,
                                    hash_hex,
                                    prev_hash_hex,
                                    "ATMN_1e6df34f5f50ff6b581b827c5e9dc5b5b787e178", // Default miner
                                    50.0, // Default reward for now
                                    block.header.bits,
                                    block.header.nonce,
                                    block.header.timestamp as i64, // Store as unix timestamp integer
                                    if h == height { 0 } else { height - h }
                                ],
                            )?;
                            synced += 1;
                        } else {
                            skipped += 1;
                        }
                    }
                    None => {
                        println!("⚠️  Block #{} not found", h);
                    }
                }
            }
            
            println!();
            println!("✅ Sync complete!");
            println!("   Synced:  {} blocks", synced);
            println!("   Skipped: {} blocks (already in database)", skipped);
        }
        None => {
            println!("⚠️  No blocks found in RocksDB");
        }
    }
    
    Ok(())
}
