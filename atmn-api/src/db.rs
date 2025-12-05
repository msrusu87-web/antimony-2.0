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
