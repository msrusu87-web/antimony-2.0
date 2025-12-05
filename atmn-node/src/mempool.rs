use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use crate::protocol::TransactionMessage;

/// Maximum transactions in mempool
pub const MAX_MEMPOOL_SIZE: usize = 10000;

/// Minimum fee (ATMN per transaction)
pub const MIN_TRANSACTION_FEE: f64 = 0.0001;

#[derive(Debug, Clone)]
pub struct MempoolTransaction {
    pub tx: TransactionMessage,
    pub received_at: DateTime<Utc>,
    pub priority: f64, // Fee per byte
}

impl MempoolTransaction {
    pub fn new(tx: TransactionMessage) -> Self {
        // Simple priority calculation: fee amount
        let priority = tx.fee;
        Self {
            tx,
            received_at: Utc::now(),
            priority,
        }
    }
}

/// Transaction mempool for unconfirmed transactions
pub struct Mempool {
    transactions: Arc<RwLock<HashMap<String, MempoolTransaction>>>,
}

impl Mempool {
    pub fn new() -> Self {
        Self {
            transactions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Add transaction to mempool
    pub async fn add_transaction(&self, tx: TransactionMessage) -> Result<(), String> {
        // Validate transaction
        if tx.amount <= 0.0 {
            return Err("Invalid transaction amount".to_string());
        }
        
        if tx.fee < MIN_TRANSACTION_FEE {
            return Err(format!("Fee too low. Minimum: {} ATMN", MIN_TRANSACTION_FEE));
        }
        
        let mut txs = self.transactions.write().await;
        
        // Check if already in mempool
        if txs.contains_key(&tx.tx_hash) {
            return Err("Transaction already in mempool".to_string());
        }
        
        // Check mempool size
        if txs.len() >= MAX_MEMPOOL_SIZE {
            // Remove lowest priority transaction
            if let Some((lowest_hash, _)) = txs.iter()
                .min_by(|(_, a), (_, b)| a.priority.partial_cmp(&b.priority).unwrap())
            {
                let lowest_hash = lowest_hash.clone();
                txs.remove(&lowest_hash);
                log::info!("Mempool full, removed lowest priority transaction: {}", lowest_hash);
            }
        }
        
        let tx_hash = tx.tx_hash.clone();
        let mempool_tx = MempoolTransaction::new(tx);
        txs.insert(tx_hash.clone(), mempool_tx);
        
        log::info!("Added transaction to mempool: {} (fee: {})", tx_hash, txs[&tx_hash].tx.fee);
        Ok(())
    }
    
    /// Remove transaction from mempool (when included in block)
    pub async fn remove_transaction(&self, tx_hash: &str) -> bool {
        let mut txs = self.transactions.write().await;
        txs.remove(tx_hash).is_some()
    }
    
    /// Get transaction from mempool
    pub async fn get_transaction(&self, tx_hash: &str) -> Option<TransactionMessage> {
        let txs = self.transactions.read().await;
        txs.get(tx_hash).map(|mt| mt.tx.clone())
    }
    
    /// Get all transactions sorted by priority (highest first)
    pub async fn get_all_transactions(&self) -> Vec<TransactionMessage> {
        let txs = self.transactions.read().await;
        let mut tx_list: Vec<_> = txs.values().cloned().collect();
        tx_list.sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap());
        tx_list.into_iter().map(|mt| mt.tx).collect()
    }
    
    /// Get top N transactions by priority for block inclusion
    pub async fn get_top_transactions(&self, n: usize) -> Vec<TransactionMessage> {
        let mut all_txs = self.get_all_transactions().await;
        all_txs.truncate(n);
        all_txs
    }
    
    /// Get mempool size
    pub async fn size(&self) -> usize {
        let txs = self.transactions.read().await;
        txs.len()
    }
    
    /// Clear all transactions
    pub async fn clear(&self) {
        let mut txs = self.transactions.write().await;
        txs.clear();
        log::info!("Mempool cleared");
    }
    
    /// Remove transactions older than specified minutes
    pub async fn cleanup_old_transactions(&self, max_age_minutes: i64) {
        let mut txs = self.transactions.write().await;
        let now = Utc::now();
        let threshold = chrono::Duration::minutes(max_age_minutes);
        
        txs.retain(|tx_hash, mt| {
            let age = now - mt.received_at;
            let should_keep = age < threshold;
            if !should_keep {
                log::info!("Removing old transaction from mempool: {} (age: {} minutes)", 
                    tx_hash, age.num_minutes());
            }
            should_keep
        });
    }
    
    /// Get mempool statistics
    pub async fn get_stats(&self) -> MempoolStats {
        let txs = self.transactions.read().await;
        let total_count = txs.len();
        let total_fees: f64 = txs.values().map(|mt| mt.tx.fee).sum();
        let avg_fee = if total_count > 0 { total_fees / total_count as f64 } else { 0.0 };
        
        MempoolStats {
            total_count,
            total_fees,
            avg_fee,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MempoolStats {
    pub total_count: usize,
    pub total_fees: f64,
    pub avg_fee: f64,
}
