use sqlx::SqlitePool;
use crate::models::{Wallet, Transaction, MiningWorker, MasterWallet};
use uuid::Uuid;
use chrono::Utc;
use anyhow::Result;

pub async fn init_db(_pool: &SqlitePool) -> Result<()> {
    Ok(())
}

// Wallet operations
pub async fn create_wallet(pool: &SqlitePool, address: &str, private_key_hash: &str) -> Result<Wallet> {
    let wallet = sqlx::query_as::<_, Wallet>(
        "INSERT INTO wallets (wallet_address, private_key_hash, balance, created_at) 
         VALUES (?, ?, 0, ?) RETURNING wallet_address, private_key_hash, balance, created_at, last_accessed"
    )
    .bind(address)
    .bind(private_key_hash)
    .bind(Utc::now())
    .fetch_one(pool)
    .await?;
    
    Ok(wallet)
}

pub async fn get_wallet(pool: &SqlitePool, address: &str) -> Result<Wallet> {
    let wallet = sqlx::query_as::<_, Wallet>(
        "SELECT wallet_address, private_key_hash, balance, created_at, last_accessed FROM wallets WHERE wallet_address = ?"
    )
    .bind(address)
    .fetch_one(pool)
    .await?;
    
    Ok(wallet)
}

pub async fn update_wallet_balance(pool: &SqlitePool, address: &str, balance: f64) -> Result<()> {
    sqlx::query("UPDATE wallets SET balance = ?, last_accessed = ? WHERE wallet_address = ?")
        .bind(balance)
        .bind(Utc::now())
        .bind(address)
        .execute(pool)
        .await?;
    
    Ok(())
}

// Transaction operations
pub async fn create_transaction(
    pool: &SqlitePool,
    from: &str,
    to: &str,
    amount: f64,
) -> Result<Transaction> {
    let tx_hash = Uuid::new_v4().to_string();
    
    let transaction = sqlx::query_as::<_, Transaction>(
        "INSERT INTO transactions (tx_hash, from_address, to_address, amount, status, timestamp) 
         VALUES (?, ?, ?, ?, 'pending', ?) RETURNING tx_hash, from_address, to_address, amount, status, timestamp, confirmed_at"
    )
    .bind(&tx_hash)
    .bind(from)
    .bind(to)
    .bind(amount)
    .bind(Utc::now())
    .fetch_one(pool)
    .await?;
    
    Ok(transaction)
}

pub async fn get_wallet_transactions(pool: &SqlitePool, address: &str) -> Result<Vec<Transaction>> {
    let transactions = sqlx::query_as::<_, Transaction>(
        "SELECT tx_hash, from_address, to_address, amount, status, timestamp, confirmed_at 
         FROM transactions WHERE from_address = ? OR to_address = ? ORDER BY timestamp DESC"
    )
    .bind(address)
    .bind(address)
    .fetch_all(pool)
    .await?;
    
    Ok(transactions)
}

pub async fn get_transaction(pool: &SqlitePool, tx_hash: &str) -> Result<Transaction> {
    let transaction = sqlx::query_as::<_, Transaction>(
        "SELECT tx_hash, from_address, to_address, amount, status, timestamp, confirmed_at FROM transactions WHERE tx_hash = ?"
    )
    .bind(tx_hash)
    .fetch_one(pool)
    .await?;
    
    Ok(transaction)
}

pub async fn confirm_transaction(pool: &SqlitePool, tx_hash: &str, _block_height: i64) -> Result<()> {
    sqlx::query("UPDATE transactions SET status = 'confirmed', confirmed_at = ? WHERE tx_hash = ?")
        .bind(Utc::now())
        .bind(tx_hash)
        .execute(pool)
        .await?;
    
    Ok(())
}

// Master wallet operations
pub async fn get_master_wallet(pool: &SqlitePool) -> Result<MasterWallet> {
    let master = sqlx::query_as::<_, MasterWallet>(
        "SELECT id, balance, last_updated FROM master_wallet LIMIT 1"
    )
    .fetch_one(pool)
    .await?;
    
    Ok(master)
}

pub async fn transfer_from_master(
    pool: &SqlitePool,
    to_address: &str,
    amount: f64,
    purpose: &str,
) -> Result<String> {
    let transfer_id = Uuid::new_v4().to_string();
    
    sqlx::query(
        "INSERT INTO master_transfers (transfer_id, to_address, amount, purpose, created_at, status)
         VALUES (?, ?, ?, ?, ?, 'completed')"
    )
    .bind(&transfer_id)
    .bind(to_address)
    .bind(amount)
    .bind(purpose)
    .bind(Utc::now())
    .execute(pool)
    .await?;
    
    update_wallet_balance(pool, to_address, amount).await?;
    
    Ok(transfer_id)
}

// Mining operations
pub async fn register_mining_worker(
    pool: &SqlitePool,
    worker_id: &str,
    wallet_address: &str,
) -> Result<MiningWorker> {
    let worker = sqlx::query_as::<_, MiningWorker>(
        "INSERT INTO mining_workers (worker_id, wallet_address, hashrate, shares_submitted, connected_at)
         VALUES (?, ?, 0, 0, ?) RETURNING worker_id, wallet_address as miner_address, hashrate, shares_submitted as shares_found, connected_at as registered_at, last_share_time as last_share"
    )
    .bind(worker_id)
    .bind(wallet_address)
    .bind(Utc::now())
    .fetch_one(pool)
    .await?;
    
    Ok(worker)
}

pub async fn get_mining_workers(pool: &SqlitePool) -> Result<Vec<MiningWorker>> {
    let workers = sqlx::query_as::<_, MiningWorker>(
        "SELECT worker_id, wallet_address as miner_address, hashrate, shares_submitted as shares_found, connected_at as registered_at, last_share_time as last_share 
         FROM mining_workers"
    )
    .fetch_all(pool)
    .await?;
    
    Ok(workers)
}

pub async fn update_worker_hashrate(pool: &SqlitePool, worker_id: &str, hashrate: f64) -> Result<()> {
    sqlx::query("UPDATE mining_workers SET hashrate = ?, last_share_time = ? WHERE worker_id = ?")
        .bind(hashrate)
        .bind(Utc::now())
        .bind(worker_id)
        .execute(pool)
        .await?;
    
    Ok(())
}

pub async fn get_pool_statistics(pool: &SqlitePool) -> Result<serde_json::Value> {
    let stats = sqlx::query_scalar::<_, String>(
        "SELECT json_object('active_workers', COUNT(DISTINCT worker_id), 'total_hashrate', COALESCE(SUM(hashrate), 0))
         FROM mining_workers WHERE last_share_time > datetime('now', '-1 hour')"
    )
    .fetch_one(pool)
    .await?;
    
    Ok(serde_json::from_str(&stats)?)
}

// UTXO and Double-Spend Prevention
pub async fn check_utxo_exists(
    pool: &SqlitePool,
    tx_hash: &str,
    output_index: i32,
) -> Result<bool> {
    let result = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM utxos WHERE tx_hash = ? AND output_index = ? AND is_spent = 0"
    )
    .bind(tx_hash)
    .bind(output_index)
    .fetch_one(pool)
    .await?;
    
    Ok(result > 0)
}

pub async fn get_utxo_amount(
    pool: &SqlitePool,
    tx_hash: &str,
    output_index: i32,
) -> Result<Option<f64>> {
    let result = sqlx::query_scalar::<_, Option<f64>>(
        "SELECT amount FROM utxos WHERE tx_hash = ? AND output_index = ? AND is_spent = 0"
    )
    .bind(tx_hash)
    .bind(output_index)
    .fetch_one(pool)
    .await?;
    
    Ok(result)
}

pub async fn mark_utxo_spent(
    pool: &SqlitePool,
    tx_hash: &str,
    output_index: i32,
    spending_tx_hash: &str,
) -> Result<()> {
    sqlx::query(
        "UPDATE utxos SET is_spent = 1, spent_in_tx = ?, spent_at = CURRENT_TIMESTAMP 
         WHERE tx_hash = ? AND output_index = ?"
    )
    .bind(spending_tx_hash)
    .bind(tx_hash)
    .bind(output_index)
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn create_utxo(
    pool: &SqlitePool,
    tx_hash: &str,
    output_index: i32,
    address: &str,
    amount: f64,
    block_height: i64,
) -> Result<()> {
    sqlx::query(
        "INSERT INTO utxos (tx_hash, output_index, address, amount, block_height, is_spent)
         VALUES (?, ?, ?, ?, ?, 0)"
    )
    .bind(tx_hash)
    .bind(output_index)
    .bind(address)
    .bind(amount)
    .bind(block_height)
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn verify_transaction_inputs(
    pool: &SqlitePool,
    tx_hash: &str,
) -> Result<bool> {
    // Get all inputs for this transaction
    let inputs: Vec<(String, i32)> = sqlx::query_as(
        "SELECT prev_tx_hash, prev_output_index FROM transaction_inputs WHERE tx_hash = ?"
    )
    .bind(tx_hash)
    .fetch_all(pool)
    .await?;
    
    // Check if this is a coinbase transaction (empty prev_tx_hash)
    if inputs.len() == 1 && inputs[0].0 == "0000000000000000000000000000000000000000000000000000000000000000" {
        return Ok(true); // Coinbase transactions are always valid
    }
    
    // Verify each input references an unspent UTXO
    for (prev_tx, prev_idx) in inputs {
        if !check_utxo_exists(pool, &prev_tx, prev_idx).await? {
            return Ok(false); // Input references non-existent or already spent UTXO
        }
    }
    
    Ok(true)
}

pub async fn check_transaction_exists(
    pool: &SqlitePool,
    tx_hash: &str,
) -> Result<bool> {
    let result = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM transactions WHERE tx_hash = ?"
    )
    .bind(tx_hash)
    .fetch_one(pool)
    .await?;
    
    Ok(result > 0)
}

pub async fn process_block_transactions(
    pool: &SqlitePool,
    block_height: i64,
    tx_hashes: &[String],
) -> Result<()> {
    for tx_hash in tx_hashes {
        // Skip if transaction already processed
        if check_transaction_exists(pool, tx_hash).await? {
            continue;
        }
        
        // Verify all inputs are valid
        if !verify_transaction_inputs(pool, tx_hash).await? {
            return Err(anyhow::anyhow!("Invalid transaction inputs in {}", tx_hash));
        }
        
        // Mark input UTXOs as spent
        let inputs: Vec<(String, i32)> = sqlx::query_as(
            "SELECT prev_tx_hash, prev_output_index FROM transaction_inputs WHERE tx_hash = ?"
        )
        .bind(tx_hash)
        .fetch_all(pool)
        .await?;
        
        for (prev_tx, prev_idx) in inputs {
            // Skip coinbase inputs
            if prev_tx == "0000000000000000000000000000000000000000000000000000000000000000" {
                continue;
            }
            mark_utxo_spent(pool, &prev_tx, prev_idx, tx_hash).await?;
        }
        
        // Create new UTXOs for outputs
        let outputs: Vec<(i32, String, f64)> = sqlx::query_as(
            "SELECT output_index, address, amount FROM transaction_outputs WHERE tx_hash = ?"
        )
        .bind(tx_hash)
        .fetch_all(pool)
        .await?;
        
        for (output_idx, address, amount) in outputs {
            create_utxo(pool, tx_hash, output_idx, &address, amount, block_height).await?;
        }
    }
    
    Ok(())
}

// Blockchain query operations

/// Get the current blockchain height (highest block number)
pub async fn get_current_height(pool: &SqlitePool) -> Result<u64> {
    let result = sqlx::query_scalar::<_, i64>(
        "SELECT COALESCE(MAX(height), 0) FROM blocks"
    )
    .fetch_one(pool)
    .await?;
    
    Ok(result as u64)
}

/// Get block by height
pub async fn get_block_by_height(pool: &SqlitePool, height: u64) -> Result<serde_json::Value> {
    let result: (String, i64, i64, i64, i64, Option<String>, Option<String>) = sqlx::query_as(
        "SELECT hash, height, timestamp, difficulty, nonce, prev_hash, merkle_root 
         FROM blocks WHERE height = ?"
    )
    .bind(height as i64)
    .fetch_one(pool)
    .await?;
    
    Ok(serde_json::json!({
        "hash": result.0,
        "height": result.1,
        "timestamp": result.2,
        "difficulty": result.3,
        "nonce": result.4,
        "prev_hash": result.5,
        "merkle_root": result.6
    }))
}


/// Calculate total fees in a set of transactions
pub async fn calculate_transaction_fees(pool: &SqlitePool, tx_hashes: &[String]) -> Result<f64> {
    let mut total_fees = 0.0;
    
    for tx_hash in tx_hashes {
        // Get transaction inputs total
        let input_total: Option<f64> = sqlx::query_scalar(
            "SELECT COALESCE(SUM(amount), 0) FROM transaction_inputs WHERE tx_hash = ?"
        )
        .bind(tx_hash)
        .fetch_one(pool)
        .await?;
        
        // Get transaction outputs total
        let output_total: Option<f64> = sqlx::query_scalar(
            "SELECT COALESCE(SUM(amount), 0) FROM transaction_outputs WHERE tx_hash = ?"
        )
        .bind(tx_hash)
        .fetch_one(pool)
        .await?;
        
        // Fee = inputs - outputs
        let fee = input_total.unwrap_or(0.0) - output_total.unwrap_or(0.0);
        if fee > 0.0 {
            total_fees += fee;
        }
    }
    
    Ok(total_fees)
}

/// Extract coinbase transaction amount from block
pub async fn get_coinbase_amount(pool: &SqlitePool, block_hash: &str) -> Result<f64> {
    // Get the first transaction in the block (coinbase)
    let amount: Option<f64> = sqlx::query_scalar(
        "SELECT COALESCE(SUM(amount), 0) FROM transaction_outputs 
         WHERE tx_hash IN (
             SELECT tx_hash FROM transactions WHERE block_hash = ? ORDER BY tx_index LIMIT 1
         )"
    )
    .bind(block_hash)
    .fetch_one(pool)
    .await?;
    
    Ok(amount.unwrap_or(0.0))
}

/// Get block timestamp by height for difficulty adjustment
pub async fn get_block_timestamp(pool: &SqlitePool, height: u64) -> Result<u32> {
    let timestamp: Option<i64> = sqlx::query_scalar(
        "SELECT timestamp FROM blocks WHERE height = ?"
    )
    .bind(height as i64)
    .fetch_optional(pool)
    .await?;
    
    Ok(timestamp.unwrap_or(0) as u32)
}

/// Get blocks for difficulty calculation (last 2016 blocks)
pub async fn get_blocks_for_difficulty(pool: &SqlitePool, end_height: u64) -> Result<Vec<(u64, u32)>> {
    let start_height = end_height.saturating_sub(2015);
    
    let blocks: Vec<(i64, i64)> = sqlx::query_as(
        "SELECT height, timestamp FROM blocks 
         WHERE height >= ? AND height <= ?
         ORDER BY height ASC"
    )
    .bind(start_height as i64)
    .bind(end_height as i64)
    .fetch_all(pool)
    .await?;
    
    Ok(blocks.iter().map(|(h, t)| (*h as u64, *t as u32)).collect())
}

// Batch operations for performance
pub async fn batch_create_utxos(
    pool: &SqlitePool,
    utxos: Vec<(String, String, u32, f64, i64)>,
) -> Result<()> {
    let mut tx = pool.begin().await?;
    
    for (tx_hash, address, output_index, amount, block_height) in utxos {
        sqlx::query(
            "INSERT INTO utxos (tx_hash, address, output_index, amount, block_height, spent) 
             VALUES (?, ?, ?, ?, ?, 0)"
        )
        .bind(&tx_hash)
        .bind(&address)
        .bind(output_index)
        .bind(amount)
        .bind(block_height)
        .execute(&mut *tx)
        .await?;
    }
    
    tx.commit().await?;
    Ok(())
}

pub async fn batch_spend_utxos(
    pool: &SqlitePool,
    utxos: Vec<(String, u32)>,
) -> Result<()> {
    let mut tx = pool.begin().await?;
    
    for (tx_hash, output_index) in utxos {
        sqlx::query(
            "UPDATE utxos SET spent = 1 WHERE tx_hash = ? AND output_index = ?"
        )
        .bind(&tx_hash)
        .bind(output_index)
        .execute(&mut *tx)
        .await?;
    }
    
    tx.commit().await?;
    Ok(())
}

pub async fn get_utxos_for_address(
    pool: &SqlitePool,
    address: &str,
    limit: i64,
) -> Result<Vec<(String, u32, f64)>> {
    let utxos = sqlx::query_as::<_, (String, u32, f64)>(
        "SELECT tx_hash, output_index, amount 
         FROM utxos 
         WHERE address = ? AND spent = 0 
         ORDER BY amount DESC 
         LIMIT ?"
    )
    .bind(address)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    
    Ok(utxos)
}
