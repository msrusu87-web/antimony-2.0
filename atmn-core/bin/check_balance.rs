// atmn-core/bin/check_balance.rs
// Check wallet balance from UTXO set

use atmn_core::Storage;
use std::env;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: check-balance <address>");
        println!("Example: check-balance ATMN_1e6df34f5f50ff6b581b827c5e9dc5b5b787e178");
        std::process::exit(1);
    }
    
    let address = &args[1];
    let db_path = env::var("DB_PATH").unwrap_or_else(|_| "./data/atmn-miner.db".to_string());
    
    println!("💰 ATMN Balance Checker");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Database: {}", db_path);
    println!("Address:  {}", address);
    println!();
    
    let storage = Storage::new(&db_path)?;
    
    // Get UTXOs for address
    let utxos = storage.get_utxos_for_address(address)?;
    
    if utxos.is_empty() {
        println!("❌ No UTXOs found for this address");
        println!("   Balance: 0 ATMN");
    } else {
        println!("✅ Found {} UTXO(s):", utxos.len());
        println!();
        
        let mut total_balance = 0u64;
        
        for (i, utxo) in utxos.iter().enumerate() {
            let amount_atmn = utxo.amount as f64 / 100_000_000.0;
            total_balance += utxo.amount;
            
            println!("UTXO #{}", i + 1);
            println!("  ├─ Amount: {} ATMN ({} satoshis)", amount_atmn, utxo.amount);
            println!("  ├─ Block Height: {}", utxo.block_height);
            println!("  ├─ TX Hash: {:?}", utxo.tx_hash);
            println!("  └─ Output Index: {}", utxo.output_index);
            println!();
        }
        
        let balance_atmn = total_balance as f64 / 100_000_000.0;
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("💰 Total Balance: {} ATMN", balance_atmn);
        println!("   ({} satoshis)", total_balance);
    }
    
    Ok(())
}
