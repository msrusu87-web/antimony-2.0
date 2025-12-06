// atmn-core/bin/create_transaction.rs
// Create and display a transaction

use atmn_core::{Storage, Transaction};
use atmn_core::tx_builder::TransactionBuilder;
use std::env;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 4 {
        println!("Usage: create-transaction <from_address> <to_address> <amount_atmn> [fee_atmn]");
        println!();
        println!("Example:");
        println!("  create-transaction \\");
        println!("    ATMN_1e6df34f5f50ff6b581b827c5e9dc5b5b787e178 \\");
        println!("    ATMN_recipient123 \\");
        println!("    25.5 \\");
        println!("    0.001");
        std::process::exit(1);
    }
    
    let from_address = &args[1];
    let to_address = &args[2];
    let amount_atmn: f64 = args[3].parse()?;
    let fee_atmn: f64 = if args.len() > 4 {
        args[4].parse()?
    } else {
        0.001 // Default fee
    };
    
    // Convert to satoshis
    let amount = (amount_atmn * 100_000_000.0) as u64;
    let fee = (fee_atmn * 100_000_000.0) as u64;
    
    println!("💸 ATMN Transaction Creator");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("From:   {}", from_address);
    println!("To:     {}", to_address);
    println!("Amount: {} ATMN ({} satoshis)", amount_atmn, amount);
    println!("Fee:    {} ATMN ({} satoshis)", fee_atmn, fee);
    println!();
    
    let db_path = env::var("DB_PATH").unwrap_or_else(|_| "./data/atmn-miner.db".to_string());
    let storage = Storage::new(&db_path)?;
    
    // Check sender balance
    let balance = storage.get_balance(from_address)?;
    let balance_atmn = balance as f64 / 100_000_000.0;
    println!("💰 Sender Balance: {} ATMN ({} satoshis)", balance_atmn, balance);
    
    let total_needed = amount + fee;
    if balance < total_needed {
        println!();
        println!("❌ Insufficient funds!");
        println!("   Need:      {} ATMN", total_needed as f64 / 100_000_000.0);
        println!("   Available: {} ATMN", balance_atmn);
        std::process::exit(1);
    }
    
    println!();
    println!("📝 Creating transaction...");
    
    let builder = TransactionBuilder::new(storage);
    let tx = builder.create_payment(from_address, to_address, amount, fee)?;
    
    println!("✅ Transaction created successfully!");
    println!();
    println!("Transaction Details:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Version:  {}", tx.version);
    println!("Inputs:   {}", tx.inputs.len());
    println!("Outputs:  {}", tx.outputs.len());
    println!("Locktime: {}", tx.locktime);
    println!();
    
    println!("Inputs:");
    for (i, input) in tx.inputs.iter().enumerate() {
        println!("  Input #{}:", i + 1);
        println!("    ├─ Prev TX: {:?}", input.prev_tx_hash);
        println!("    ├─ Prev Index: {}", input.prev_tx_index);
        println!("    └─ Sequence: {}", input.sequence);
    }
    println!();
    
    println!("Outputs:");
    for (i, output) in tx.outputs.iter().enumerate() {
        let output_atmn = output.amount as f64 / 100_000_000.0;
        let recipient = String::from_utf8_lossy(&output.script_pubkey);
        println!("  Output #{}:", i + 1);
        println!("    ├─ Amount: {} ATMN ({} satoshis)", output_atmn, output.amount);
        println!("    └─ Recipient: {}", recipient);
    }
    println!();
    
    // Calculate total outputs
    let total_output: u64 = tx.outputs.iter().map(|o| o.amount).sum();
    let total_input: u64 = tx.inputs.iter().map(|_| balance / tx.inputs.len() as u64).sum();
    let actual_fee = total_input - total_output;
    
    println!("Summary:");
    println!("  ├─ Total Input:  {} ATMN", total_input as f64 / 100_000_000.0);
    println!("  ├─ Total Output: {} ATMN", total_output as f64 / 100_000_000.0);
    println!("  └─ Fee:          {} ATMN", actual_fee as f64 / 100_000_000.0);
    println!();
    
    println!("📌 Transaction Hash: {:?}", tx.hash());
    println!();
    println!("⚠️  Note: Transaction created but not submitted to mempool.");
    println!("   Use submit-transaction command to broadcast it.");
    
    Ok(())
}
