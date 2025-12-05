use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::task::JoinHandle;
use atmn_core::{
    Miner, MinerConfig, BlockTemplate,
    Transaction,
    Mempool, MempoolConfig,
};
use atmn_core::types::{BlockHash, TxHash};
use log::{info, error};

/// Mining state shared across threads
#[derive(Debug, Clone)]
pub struct MiningState {
    pub is_mining: bool,
    pub miner_address: Option<String>,
    pub blocks_found: u64,
    pub hash_rate: f64,
    pub start_time: Option<i64>,
    pub current_height: u64,
}

impl Default for MiningState {
    fn default() -> Self {
        MiningState {
            is_mining: false,
            miner_address: None,
            blocks_found: 0,
            hash_rate: 0.0,
            start_time: None,
            current_height: 0,
        }
    }
}

/// Mining manager that coordinates background mining
pub struct MiningManager {
    state: Arc<Mutex<MiningState>>,
    mempool: Arc<Mutex<Mempool>>,
    mining_task: Arc<Mutex<Option<JoinHandle<()>>>>,
    should_stop: Arc<Mutex<bool>>,
    database_url: String,
}

impl MiningManager {
    pub fn new(database_url: String) -> Self {
        let mempool_config = MempoolConfig {
            max_size: 50_000,
            max_tx_size: 100_000,
            min_fee_per_byte: 1,
            tx_expiration: 3600,
        };

        MiningManager {
            state: Arc::new(Mutex::new(MiningState::default())),
            mempool: Arc::new(Mutex::new(Mempool::with_config(mempool_config))),
            mining_task: Arc::new(Mutex::new(None)),
            should_stop: Arc::new(Mutex::new(false)),
            database_url,
        }
    }

    /// Start mining with given parameters
    pub fn start_mining(
        &self,
        miner_address: String,
        thread_count: usize,
        difficulty_bits: u32,
        prev_block_hash: [u8; 32],
        height: u64,
    ) -> Result<(), String> {
        let mut state = self.state.lock().unwrap();
        
        if state.is_mining {
            return Err("Mining already active".to_string());
        }

        // Update state
        state.is_mining = true;
        state.miner_address = Some(miner_address.clone());
        state.start_time = Some(chrono::Utc::now().timestamp());
        state.current_height = height;

        // Reset stop flag
        *self.should_stop.lock().unwrap() = false;

        // Spawn mining task
        let state_clone = Arc::clone(&self.state);
        let mempool_clone = Arc::clone(&self.mempool);
        let should_stop = Arc::clone(&self.should_stop);

        let database_url = self.database_url.clone();
        let handle = tokio::spawn(async move {
            mining_task(
                state_clone,
                mempool_clone,
                should_stop,
                database_url,
                miner_address,
                thread_count,
                difficulty_bits,
                prev_block_hash,
                height,
            )
            .await;
        });

        *self.mining_task.lock().unwrap() = Some(handle);

        Ok(())
    }

    /// Stop mining
    pub fn stop_mining(&self) -> Result<(u64, Option<String>), String> {
        let mut state = self.state.lock().unwrap();
        
        if !state.is_mining {
            return Err("Mining not active".to_string());
        }

        // Signal stop
        *self.should_stop.lock().unwrap() = true;

        let blocks_found = state.blocks_found;
        let miner_address = state.miner_address.clone();

        // Reset state
        state.is_mining = false;
        state.miner_address = None;
        state.start_time = None;
        state.hash_rate = 0.0;

        Ok((blocks_found, miner_address))
    }

    /// Get current mining state
    pub fn get_state(&self) -> MiningState {
        self.state.lock().unwrap().clone()
    }

    /// Add transaction to mempool
    pub fn add_transaction(&self, tx: Transaction) -> Result<(), String> {
        self.mempool
            .lock()
            .unwrap()
            .add_transaction(tx)
            .map_err(|e| format!("Failed to add transaction: {:?}", e))
    }

    /// Get mempool statistics
    pub fn get_mempool_stats(&self) -> (usize, usize, usize, u64) {
        let mempool = self.mempool.lock().unwrap();
        let stats = mempool.stats();
        (
            stats.transaction_count,
            stats.total_size_bytes,
            50_000,
            1,
        )
    }

    /// Get pending transactions for block template
    pub fn get_pending_transactions(&self, limit: usize) -> Vec<Transaction> {
        self.mempool
            .lock()
            .unwrap()
            .get_ordered_transactions(limit)
    }
}

/// Helper function to get the latest block hash from database
async fn get_latest_block_hash(database_url: &str) -> Result<([u8; 32], u64), String> {
    use sqlx::SqlitePool;
    
    let pool = SqlitePool::connect(database_url)
        .await
        .map_err(|e| format!("Database connection failed: {}", e))?;
    
    let result = sqlx::query!(
        r#"
        SELECT hash, height
        FROM blocks
        ORDER BY height DESC
        LIMIT 1
        "#
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| format!("Query failed: {}", e))?;
    
    pool.close().await;
    
    match result {
        Some(row) => {
            let hash_bytes = hex::decode(&row.hash)
                .map_err(|e| format!("Invalid hash hex: {}", e))?;
            
            if hash_bytes.len() != 32 {
                return Err("Invalid hash length".to_string());
            }
            
            let mut hash_array = [0u8; 32];
            hash_array.copy_from_slice(&hash_bytes);
            
            Ok((hash_array, row.height as u64))
        }
        None => {
            // No blocks in database, return genesis hash
            Ok(([0u8; 32], 0))
        }
    }
}

/// Background mining task
async fn mining_task(
    state: Arc<Mutex<MiningState>>,
    mempool: Arc<Mutex<Mempool>>,
    should_stop: Arc<Mutex<bool>>,
    database_url: String,
    miner_address: String,
    thread_count: usize,
    difficulty_bits: u32,
    mut prev_block_hash: [u8; 32],
    mut height: u64,
) {
    info!(
        "Mining task started: address={}, threads={}, difficulty={}, height={}",
        miner_address, thread_count, difficulty_bits, height
    );

    // Create miner instance
    let config = MinerConfig {
        max_nonce: 100_000,
        thread_count,
        update_interval_ms: 5000,
    };
    let mut miner = Miner::with_config(config);

    let start_time = std::time::Instant::now();

    loop {
        // Check stop signal
        if *should_stop.lock().unwrap() {
            info!("Mining task stopped by request");
            break;
        }

        // Get pending transactions
        let transactions = mempool
            .lock()
            .unwrap()
            .get_ordered_transactions(1000);

        // Calculate total fees
        let total_fees: u64 = transactions
            .iter()
            .map(|tx| {
                let size = tx.inputs.len() * 148 + tx.outputs.len() * 34 + 10;
                size as u64
            })
            .sum();

        // Create coinbase transaction
        let miner_pubkey = hex::decode(&miner_address.replace("ATMN_", ""))
            .unwrap_or_else(|_| miner_address.as_bytes().to_vec());
        
        let coinbase = atmn_core::miner::create_coinbase_transaction(miner_pubkey, height, total_fees);

        // Build transaction list
        let mut all_txs = vec![coinbase];
        all_txs.extend(transactions);

        // Calculate merkle root
        let merkle_root = match atmn_core::miner::calculate_merkle_root(&all_txs) {
            Ok(root) => root,
            Err(e) => {
                error!("Failed to calculate merkle root: {:?}", e);
                break;
            }
        };

        // Create block template
        let template = BlockTemplate {
            version: 1,
            prev_block_hash: BlockHash::from_bytes(prev_block_hash),
            merkle_root,
            height,
            transactions: all_txs.clone(),
            difficulty_bits,
            template_time: chrono::Utc::now().timestamp() as u32,
        };

        // Mine block
        match miner.mine_block(template) {
            Ok(result) => {
                if result.success {
                    if let Some(block) = result.block {
                        info!(
                            "✨ Block found! Height: {}, Hashes: {}",
                            height, result.hashes_attempted
                        );

                        // Update state
                        {
                            let mut state = state.lock().unwrap();
                            state.blocks_found += 1;
                        }
                        
                        // Get new prev_hash from database for continuous mining
                        match get_latest_block_hash(&database_url).await {
                            Ok((new_hash, new_height)) => {
                                prev_block_hash = new_hash;
                                height = new_height + 1;
                                info!("🔄 Continuing mining on height {}", height);
                                
                                // Update state height
                                {
                                    let mut state = state.lock().unwrap();
                                    state.current_height = height;
                                }
                                
                                // Small delay before next block
                                tokio::time::sleep(Duration::from_millis(500)).await;
                                continue;
                            }
                            Err(e) => {
                                error!("Failed to get latest block for continuous mining: {}", e);
                                break;
                            }
                        }
                    }
                } else {
                    // No block found, continue
                }

                // Update hash rate
                let elapsed = start_time.elapsed().as_secs_f64();
                if elapsed > 0.0 {
                    let hash_rate = result.hashes_attempted as f64 / elapsed;
                    state.lock().unwrap().hash_rate = hash_rate;
                }
            }
            Err(e) => {
                error!("Mining error: {:?}", e);
                break;
            }
        }

        // Small delay between attempts
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    // Mark mining as stopped
    let mut state = state.lock().unwrap();
    state.is_mining = false;
    state.hash_rate = 0.0;
    
    let elapsed = start_time.elapsed().as_secs_f64();
    info!(
        "Mining task completed: blocks_found={}, total_time={:.1}s",
        state.blocks_found, elapsed
    );
}
