// atmn-core/bin/mempool_manager.rs
// Persistent mempool manager for testing transaction batching

use atmn_core::{Storage, Transaction, Mempool};
use atmn_core::tx_builder::TransactionBuilder;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }
    
    let command = &args[1];
    let db_path = "./data/atmn-miner.db";
    let storage = Storage::new(db_path)?;
    let mut mempool = Mempool::new();
    
    match command.as_str() {
        "add" => {
            if args.len() < 6 {
                eprintln!("Usage: mempool-manager add <from> <to> <amount> <fee>");
                return Ok(());
            }
            
            let from = &args[2];
            let to = &args[3];
            let amount: u64 = (args[4].parse::<f64>()? * 100_000_000.0) as u64;
            let fee: u64 = (args[5].parse::<f64>()? * 100_000_000.0) as u64;
            
            // Check balance
            let balance = storage.get_balance(from)?;
            if balance < amount + fee {
                eprintln!("❌ Insufficient funds!");
                eprintln!("   Need: {} ATMN", (amount + fee) as f64 / 100_000_000.0);
                eprintln!("   Have: {} ATMN", balance as f64 / 100_000_000.0);
                return Ok(());
            }
            
            // Create transaction
            let builder = TransactionBuilder::new(storage.clone());
            let tx = builder.create_payment(from, to, amount, fee)?;
            
            // Add to mempool
            mempool.add_transaction(tx.clone())?;
            
            println!("✅ Transaction added to mempool");
            println!("   From: {}", from);
            println!("   To: {}", to);
            println!("   Amount: {} ATMN", amount as f64 / 100_000_000.0);
            println!("   Fee: {} ATMN", fee as f64 / 100_000_000.0);
            println!("   Mempool size: {}", mempool.size());
        }
        
        "list" => {
            let limit = if args.len() > 2 {
                args[2].parse()?
            } else {
                10
            };
            
            let txs = mempool.get_transactions(limit)?;
            
            println!("📋 Mempool Contents");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("Total transactions: {}", txs.len());
            println!();
            
            for (i, tx) in txs.iter().enumerate() {
                println!("Transaction #{}", i + 1);
                println!("   Inputs: {}", tx.inputs.len());
                println!("   Outputs: {}", tx.outputs.len());
                
                let total_out: u64 = tx.outputs.iter().map(|o| o.amount).sum();
                println!("   Total: {} ATMN", total_out as f64 / 100_000_000.0);
                println!();
            }
        }
        
        "stats" => {
            let stats = mempool.stats();
            
            println!("📊 Mempool Statistics");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("Transaction count: {}", stats.transaction_count);
            println!("Total size: {} bytes", stats.total_size_bytes);
            println!("Max size: {} transactions", stats.max_size);
            
            if stats.transaction_count > 0 {
                let avg_size = stats.total_size_bytes / stats.transaction_count;
                println!("Average tx size: {} bytes", avg_size);
            }
        }
        
        "clear" => {
            mempool.clear();
            println!("✅ Mempool cleared");
        }
        
        _ => {
            print_usage();
        }
    }
    
    Ok(())
}

fn print_usage() {
    println!("Mempool Manager");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("Commands:");
    println!("  add <from> <to> <amount> <fee>  - Add transaction to mempool");
    println!("  list [limit]                     - List pending transactions");
    println!("  stats                            - Show mempool statistics");
    println!("  clear                            - Clear all transactions");
    println!();
    println!("Examples:");
    println!("  mempool-manager add ATMN_sender ATMN_recipient 10.0 0.01");
    println!("  mempool-manager list 20");
    println!("  mempool-manager stats");
}
