// atmn-core/bin/submit_transaction.rs
// Submit a transaction to the mempool and optionally mine it

use atmn_core::{Storage, Transaction, Block, MultiThreadedMiner, BlockTemplate};
use atmn_core::tx_builder::TransactionBuilder;
use atmn_core::mempool::Mempool;
use atmn_core::types::BlockHash;
use std::env;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 4 {
        println!("Usage: submit-transaction <from_address> <to_address> <amount_atmn> [fee_atmn] [--mine]");
        println!();
        println!("Options:");
        println!("  --mine    Mine a block immediately with this transaction");
        println!();
        println!("Example:");
        println!("  submit-transaction \\");
        println!("    ATMN_1e6df34f5f50ff6b581b827c5e9dc5b5b787e178 \\");
        println!("    ATMN_recipient123 \\");
        println!("    25.0 \\");
        println!("    0.001 \\");
        println!("    --mine");
        std::process::exit(1);
    }
    
    let from_address = &args[1];
    let to_address = &args[2];
    let amount_atmn: f64 = args[3].parse()?;
    let fee_atmn: f64 = if args.len() > 4 && !args[4].starts_with("--") {
        args[4].parse()?
    } else {
        0.001 // Default fee
    };
    
    let should_mine = args.contains(&"--mine".to_string());
    
    // Convert to satoshis
    let amount = (amount_atmn * 100_000_000.0) as u64;
    let fee = (fee_atmn * 100_000_000.0) as u64;
    
    println!("💸 ATMN Transaction Submission");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("From:   {}", from_address);
    println!("To:     {}", to_address);
    println!("Amount: {} ATMN", amount_atmn);
    println!("Fee:    {} ATMN", fee_atmn);
    if should_mine {
        println!("Mode:   Mine immediately");
    }
    println!();
    
    let db_path = env::var("DB_PATH").unwrap_or_else(|_| "./data/atmn-miner.db".to_string());
    let storage = Storage::new(&db_path)?;
    
    // Check sender balance
    println!("📊 Checking sender balance...");
    let balance = storage.get_balance(from_address)?;
    let balance_atmn = balance as f64 / 100_000_000.0;
    println!("   Balance: {} ATMN", balance_atmn);
    
    let total_needed = amount + fee;
    if balance < total_needed {
        println!();
        println!("❌ Insufficient funds!");
        println!("   Need:      {} ATMN", total_needed as f64 / 100_000_000.0);
        println!("   Available: {} ATMN", balance_atmn);
        std::process::exit(1);
    }
    
    // Create transaction
    println!();
    println!("📝 Creating transaction...");
    let builder = TransactionBuilder::new(storage.clone());
    let tx = builder.create_payment(from_address, to_address, amount, fee)?;
    
    let tx_hash = tx.hash();
    println!("✅ Transaction created!");
    println!("   TX Hash: {:?}", tx_hash);
    
    // Add to mempool
    println!();
    println!("📤 Submitting to mempool...");
    let mut mempool = Mempool::new();
    mempool.add_transaction(tx.clone())?;
    
    let mempool_size = mempool.size();
    println!("✅ Transaction added to mempool!");
    println!("   Mempool size: {} transaction(s)", mempool_size);
    
    if should_mine {
        println!();
        println!("⛏️  Mining block with transaction...");
        
        // Get current height
        let current_height = storage.get_best_height()?.unwrap_or(0);
        let next_height = current_height + 1;
        
        // Get previous block hash
        let prev_block_hash = match storage.get_block(current_height) {
            Ok(Some(prev_block)) => prev_block.hash(),
            _ => BlockHash::zero(),
        };
        
        // Create coinbase transaction
        let block_reward = 5_000_000_000u64; // 50 ATMN
        let coinbase_tx = Block::create_coinbase_tx(next_height, from_address, block_reward);
        
        // Get transactions from mempool
        let pending_txs = mempool.get_transactions(10)?; // Get up to 10 transactions
        
        // Build block with coinbase + pending transactions
        let mut block_txs = vec![coinbase_tx];
        block_txs.extend(pending_txs);
        
        // Create block template
        let template = BlockTemplate {
            prev_block_hash,
            merkle_root: BlockHash::zero(),
            height: next_height,
            transactions: block_txs,
            difficulty_bits: 0x207fffff,
            version: 1,
            template_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as u32,
        };
        
        // Mine the block
        let mut miner = MultiThreadedMiner::new(Some(6));
        match miner.mine_block(template)? {
            result if result.success && result.block.is_some() => {
                let block = result.block.unwrap();
                
                // Store the block
                storage.put_block(next_height, &block)?;
                
                println!("✅ Block #{} mined!", next_height);
                println!("   Hash: {:?}", block.hash());
                println!("   Transactions: {}", block.transactions.len());
                println!("   - 1 coinbase");
                println!("   - {} transfer(s)", block.transactions.len() - 1);
                
                // Check updated balances
                println!();
                println!("💰 Updated Balances:");
                
                let sender_balance = storage.get_balance(from_address)?;
                println!("   Sender:    {} ATMN", sender_balance as f64 / 100_000_000.0);
                
                let recipient_balance = storage.get_balance(to_address)?;
                println!("   Recipient: {} ATMN", recipient_balance as f64 / 100_000_000.0);
            }
            _ => {
                println!("❌ Mining failed!");
                std::process::exit(1);
            }
        }
    } else {
        println!();
        println!("ℹ️  Transaction is waiting in mempool.");
        println!("   Use --mine flag to mine it immediately, or");
        println!("   wait for the next block to be mined.");
    }
    
    Ok(())
}
