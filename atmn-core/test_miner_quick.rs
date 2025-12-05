// Quick test of mining logic
use atmn_core::miner::*;
use atmn_core::types::BlockHash;

fn main() {
    println!("Testing mining with extremely easy difficulty...");
    
    let mut miner = Miner::new();
    let template = BlockTemplate {
        prev_block_hash: BlockHash::zero(),
        merkle_root: BlockHash::zero(),
        height: 0,
        transactions: vec![],
        difficulty_bits: 0x20ffffff,  // Extremely easy
        version: 1,
        template_time: 1704067200,
    };
    
    println!("Starting mine...");
    match miner.mine_block(template) {
        Ok(result) => {
            if result.success {
                println!("✅ Mining successful!");
                println!("Hashes: {}", result.hashes_attempted);
                println!("Hash rate: {:.2} MH/s", miner.stats().hash_rate / 1_000_000.0);
            } else {
                println!("❌ Mining failed");
            }
        }
        Err(e) => {
            println!("❌ Error: {:?}", e);
        }
    }
}
