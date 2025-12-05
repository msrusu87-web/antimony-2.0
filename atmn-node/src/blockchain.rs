use sqlx::{SqlitePool, Row};
use chrono::{DateTime, Utc};
use crate::protocol::{BlockResponseMessage, TransactionData};
use crate::error::NodeError;

pub struct Blockchain {
    pool: SqlitePool,
}

impl Blockchain {
    pub async fn new(database_path: &str) -> Result<Self, NodeError> {
        let pool = SqlitePool::connect(&format!("sqlite:{}", database_path)).await?;
        
        // Create tables if they don't exist
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS blocks (
                block_hash TEXT PRIMARY KEY,
                block_height INTEGER NOT NULL,
                previous_hash TEXT NOT NULL,
                merkle_root TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                difficulty_target INTEGER NOT NULL,
                nonce INTEGER NOT NULL,
                miner_address TEXT NOT NULL,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            )"
        )
        .execute(&pool)
        .await?;
        
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_block_height ON blocks(block_height)"
        )
        .execute(&pool)
        .await?;
        
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS transactions (
                tx_hash TEXT PRIMARY KEY,
                block_hash TEXT NOT NULL,
                from_address TEXT NOT NULL,
                to_address TEXT NOT NULL,
                amount REAL NOT NULL,
                fee REAL NOT NULL,
                timestamp TEXT NOT NULL,
                signature TEXT NOT NULL,
                FOREIGN KEY (block_hash) REFERENCES blocks(block_hash)
            )"
        )
        .execute(&pool)
        .await?;
        
        log::info!("Blockchain database initialized");
        
        Ok(Self { pool })
    }
    
    /// Get current blockchain height
    pub async fn get_height(&self) -> Result<u64, NodeError> {
        let row = sqlx::query("SELECT MAX(block_height) as height FROM blocks")
            .fetch_one(&self.pool)
            .await?;
        
        let height: Option<i64> = row.try_get("height")?;
        Ok(height.unwrap_or(0) as u64)
    }
    
    /// Get block by hash
    pub async fn get_block(&self, block_hash: &str) -> Result<Option<BlockResponseMessage>, NodeError> {
        let row = sqlx::query(
            "SELECT * FROM blocks WHERE block_hash = ?"
        )
        .bind(block_hash)
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                let block_hash: String = row.try_get("block_hash")?;
                let transactions = self.get_block_transactions(&block_hash).await?;
                
                Ok(Some(BlockResponseMessage {
                    block_hash: row.try_get("block_hash")?,
                    block_height: row.try_get::<i64, _>("block_height")? as u64,
                    previous_hash: row.try_get("previous_hash")?,
                    merkle_root: row.try_get("merkle_root")?,
                    timestamp: row.try_get::<String, _>("timestamp")?.parse()?,
                    difficulty_target: row.try_get::<i64, _>("difficulty_target")? as u64,
                    nonce: row.try_get::<i64, _>("nonce")? as u64,
                    miner_address: row.try_get("miner_address")?,
                    transactions,
                }))
            }
            None => Ok(None),
        }
    }
    
    /// Get block by height
    pub async fn get_block_by_height(&self, height: u64) -> Result<Option<BlockResponseMessage>, NodeError> {
        let row = sqlx::query(
            "SELECT block_hash FROM blocks WHERE block_height = ?"
        )
        .bind(height as i64)
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                let block_hash: String = row.try_get("block_hash")?;
                self.get_block(&block_hash).await
            }
            None => Ok(None),
        }
    }
    
    /// Get transactions for a block
    async fn get_block_transactions(&self, block_hash: &str) -> Result<Vec<TransactionData>, NodeError> {
        let rows = sqlx::query(
            "SELECT * FROM transactions WHERE block_hash = ?"
        )
        .bind(block_hash)
        .fetch_all(&self.pool)
        .await?;
        
        let mut transactions = Vec::new();
        for row in rows {
            transactions.push(TransactionData {
                tx_hash: row.try_get("tx_hash")?,
                from_address: row.try_get("from_address")?,
                to_address: row.try_get("to_address")?,
                amount: row.try_get("amount")?,
                fee: row.try_get("fee")?,
                timestamp: row.try_get::<String, _>("timestamp")?.parse()?,
                signature: row.try_get("signature")?,
            });
        }
        
        Ok(transactions)
    }
    
    /// Add block to blockchain
    pub async fn add_block(&self, block: &BlockResponseMessage) -> Result<(), NodeError> {
        // Insert block
        sqlx::query(
            "INSERT INTO blocks (block_hash, block_height, previous_hash, merkle_root, 
             timestamp, difficulty_target, nonce, miner_address)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&block.block_hash)
        .bind(block.block_height as i64)
        .bind(&block.previous_hash)
        .bind(&block.merkle_root)
        .bind(block.timestamp.to_rfc3339())
        .bind(block.difficulty_target as i64)
        .bind(block.nonce as i64)
        .bind(&block.miner_address)
        .execute(&self.pool)
        .await?;
        
        // Insert transactions
        for tx in &block.transactions {
            sqlx::query(
                "INSERT INTO transactions (tx_hash, block_hash, from_address, to_address, 
                 amount, fee, timestamp, signature)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(&tx.tx_hash)
            .bind(&block.block_hash)
            .bind(&tx.from_address)
            .bind(&tx.to_address)
            .bind(tx.amount)
            .bind(tx.fee)
            .bind(tx.timestamp.to_rfc3339())
            .bind(&tx.signature)
            .execute(&self.pool)
            .await?;
        }
        
        log::info!("Added block {} at height {}", block.block_hash, block.block_height);
        Ok(())
    }
    
    /// Get best block hash
    pub async fn get_best_block_hash(&self) -> Result<String, NodeError> {
        let row = sqlx::query(
            "SELECT block_hash FROM blocks ORDER BY block_height DESC LIMIT 1"
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(row.try_get("block_hash")?)
    }
}
