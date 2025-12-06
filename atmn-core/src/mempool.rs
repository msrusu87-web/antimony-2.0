// atmn-core/src/mempool.rs
// Transaction Memory Pool for ATMN

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;
use crate::transaction::Transaction;
use crate::types::{TxHash, BlockHash};
use crate::error::{Error, Result};

/// Transaction priority for mempool ordering
#[derive(Debug, Clone)]
struct TxPriority {
    /// Fee per byte (higher is better)
    fee_per_byte: u64,
    /// Transaction hash
    tx_hash: TxHash,
    /// Transaction
    transaction: Transaction,
}

impl Eq for TxPriority {}

impl PartialEq for TxPriority {
    fn eq(&self, other: &Self) -> bool {
        self.tx_hash == other.tx_hash
    }
}

impl Ord for TxPriority {
    fn cmp(&self, other: &Self) -> Ordering {
        // Higher fee per byte = higher priority
        self.fee_per_byte.cmp(&other.fee_per_byte)
    }
}

impl PartialOrd for TxPriority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Transaction mempool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MempoolConfig {
    /// Maximum transactions in mempool
    pub max_size: usize,
    /// Maximum transaction size in bytes
    pub max_tx_size: usize,
    /// Minimum fee per byte (in satoshis)
    pub min_fee_per_byte: u64,
    /// Transaction expiration time in seconds
    pub tx_expiration: u64,
}

impl Default for MempoolConfig {
    fn default() -> Self {
        MempoolConfig {
            max_size: 50_000,
            max_tx_size: 100_000,
            min_fee_per_byte: 1,
            tx_expiration: 86400, // 24 hours
        }
    }
}

/// Transaction memory pool
#[derive(Debug)]
pub struct Mempool {
    /// Configuration
    config: MempoolConfig,
    /// Transactions by hash for quick lookup
    transactions: HashMap<TxHash, Transaction>,
    /// Priority queue for transaction ordering
    priority_queue: BinaryHeap<TxPriority>,
    /// Total size in bytes
    total_size: usize,
}

impl Mempool {
    /// Create new mempool with default config
    pub fn new() -> Self {
        Self::with_config(MempoolConfig::default())
    }

    /// Create new mempool with custom config
    pub fn with_config(config: MempoolConfig) -> Self {
        Mempool {
            config,
            transactions: HashMap::new(),
            priority_queue: BinaryHeap::new(),
            total_size: 0,
        }
    }

    /// Add transaction to mempool
    pub fn add_transaction(&mut self, tx: Transaction) -> Result<()> {
        // Calculate transaction hash
        let tx_hash = tx.hash();

        // Check if already in mempool
        if self.transactions.contains_key(&tx_hash) {
            return Err(Error::DuplicateTransaction);
        }

        // Validate transaction
        self.validate_transaction(&tx)?;

        // Calculate fee (output sum - input sum, but we need UTXO set for inputs)
        // For now, estimate fee based on transaction size
        let tx_size = self.estimate_tx_size(&tx);
        let estimated_fee = tx_size as u64; // 1 sat per byte minimum
        
        let fee_per_byte = if tx_size > 0 {
            estimated_fee / tx_size as u64
        } else {
            0
        };

        // Check if fee meets minimum
        if fee_per_byte < self.config.min_fee_per_byte {
            return Err(Error::FeeTooLow);
        }

        // Check mempool size limit
        if self.transactions.len() >= self.config.max_size {
            // Try to evict lowest priority transaction
            self.evict_lowest_priority()?;
        }

        // Add to priority queue
        self.priority_queue.push(TxPriority {
            fee_per_byte,
            tx_hash,
            transaction: tx.clone(),
        });

        // Add to transactions map
        self.transactions.insert(tx_hash, tx);
        self.total_size += tx_size;

        Ok(())
    }

    /// Remove transaction from mempool
    pub fn remove_transaction(&mut self, tx_hash: &TxHash) -> Option<Transaction> {
        if let Some(tx) = self.transactions.remove(tx_hash) {
            let tx_size = self.estimate_tx_size(&tx);
            self.total_size = self.total_size.saturating_sub(tx_size);
            
            // Rebuild priority queue without this transaction
            self.rebuild_priority_queue();
            
            Some(tx)
        } else {
            None
        }
    }

    /// Get transaction by hash
    pub fn get_transaction(&self, tx_hash: &TxHash) -> Option<&Transaction> {
        self.transactions.get(tx_hash)
    }

    /// Get transactions sorted by priority (highest fee first)
    pub fn get_ordered_transactions(&self, limit: usize) -> Vec<Transaction> {
        self.priority_queue
            .iter()
            .take(limit)
            .map(|p| p.transaction.clone())
            .collect()
    }

    /// Remove transactions that are included in a block
    pub fn remove_confirmed_transactions(&mut self, confirmed_txs: &[Transaction]) {
        for tx in confirmed_txs {
            let tx_hash = tx.hash();
            self.remove_transaction(&tx_hash);
        }
    }

    /// Clear all transactions from mempool
    pub fn clear(&mut self) {
        self.transactions.clear();
        self.priority_queue.clear();
        self.total_size = 0;
    }

    /// Get mempool statistics
    pub fn stats(&self) -> MempoolStats {
        MempoolStats {
            transaction_count: self.transactions.len(),
            total_size_bytes: self.total_size,
            max_size: self.config.max_size,
        }
    }
    
    /// Get number of transactions in mempool
    pub fn size(&self) -> usize {
        self.transactions.len()
    }
    
    /// Get transactions for block inclusion (up to limit)
    pub fn get_transactions(&self, limit: usize) -> Result<Vec<Transaction>> {
        Ok(self.get_ordered_transactions(limit))
    }

    /// Validate transaction
    fn validate_transaction(&self, tx: &Transaction) -> Result<()> {
        // Check transaction size
        let tx_size = self.estimate_tx_size(tx);
        if tx_size > self.config.max_tx_size {
            return Err(Error::TransactionTooLarge);
        }

        // Check if has inputs and outputs
        if tx.inputs.is_empty() && !tx.is_coinbase() {
            return Err(Error::InvalidTransaction);
        }

        if tx.outputs.is_empty() {
            return Err(Error::InvalidTransaction);
        }

        // Check output amounts are positive
        for output in &tx.outputs {
            if output.amount == 0 {
                return Err(Error::InvalidAmount);
            }
        }

        // Additional validation can be added here:
        // - Signature verification
        // - Input UTXO existence
        // - Double-spend check
        // - Script validation

        Ok(())
    }

    /// Estimate transaction size in bytes
    fn estimate_tx_size(&self, tx: &Transaction) -> usize {
        // Estimate based on UTXO transaction structure:
        // - Version: 4 bytes
        // - Input count: 1-9 bytes (varint)
        // - Inputs: ~180 bytes each (outpoint + script + sequence)
        // - Output count: 1-9 bytes (varint)
        // - Outputs: ~34 bytes each (amount + script)
        // - Locktime: 4 bytes
        
        let base_size = 4 + 1 + 1 + 4; // version + input_count + output_count + locktime
        let input_size = tx.inputs.len() * 180;
        let output_size = tx.outputs.len() * 34;
        
        base_size + input_size + output_size
    }

    /// Evict lowest priority transaction
    fn evict_lowest_priority(&mut self) -> Result<()> {
        // Get all transactions sorted by priority (lowest first)
        let mut txs: Vec<_> = self.priority_queue.iter().cloned().collect();
        txs.sort_by(|a, b| a.fee_per_byte.cmp(&b.fee_per_byte));
        
        if let Some(lowest) = txs.first() {
            self.remove_transaction(&lowest.tx_hash);
            Ok(())
        } else {
            Err(Error::MempoolFull)
        }
    }

    /// Rebuild priority queue (after removing transactions)
    fn rebuild_priority_queue(&mut self) {
        let mut new_queue = BinaryHeap::new();
        
        for (tx_hash, tx) in &self.transactions {
            let tx_size = self.estimate_tx_size(tx);
            let estimated_fee = tx_size as u64;
            let fee_per_byte = if tx_size > 0 {
                estimated_fee / tx_size as u64
            } else {
                0
            };
            
            new_queue.push(TxPriority {
                fee_per_byte,
                tx_hash: *tx_hash,
                transaction: tx.clone(),
            });
        }
        
        self.priority_queue = new_queue;
    }
}

impl Default for Mempool {
    fn default() -> Self {
        Self::new()
    }
}

/// Mempool statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MempoolStats {
    /// Number of transactions
    pub transaction_count: usize,
    /// Total size in bytes
    pub total_size_bytes: usize,
    /// Maximum size
    pub max_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::{TxInput, TxOutput};
    use crate::types::TxHash;

    fn create_test_transaction(output_amount: u64) -> Transaction {
        Transaction {
            version: 1,
            inputs: vec![TxInput {
                prev_tx_hash: TxHash::from_bytes([1u8; 32]),
                prev_tx_index: 0,
                script: vec![],
                sequence: 0xFFFFFFFF,
            }],
            outputs: vec![TxOutput {
                amount: output_amount,
                script_pubkey: vec![],
            }],
            locktime: 0,
        }
    }

    #[test]
    fn test_mempool_creation() {
        let mempool = Mempool::new();
        assert_eq!(mempool.transactions.len(), 0);
        assert_eq!(mempool.total_size, 0);
    }

    #[test]
    fn test_add_transaction() {
        let mut mempool = Mempool::new();
        let tx = create_test_transaction(1000000);
        
        let result = mempool.add_transaction(tx);
        assert!(result.is_ok());
        assert_eq!(mempool.transactions.len(), 1);
    }

    #[test]
    fn test_remove_transaction() {
        let mut mempool = Mempool::new();
        let tx = create_test_transaction(1000000);
        let tx_hash = tx.hash();
        
        let _ = mempool.add_transaction(tx);
        
        let removed = mempool.remove_transaction(&tx_hash);
        assert!(removed.is_some());
        assert_eq!(mempool.transactions.len(), 0);
    }

    #[test]
    fn test_mempool_stats() {
        let mut mempool = Mempool::new();
        let tx = create_test_transaction(1000000);
        
        let _ = mempool.add_transaction(tx);
        
        let stats = mempool.stats();
        assert_eq!(stats.transaction_count, 1);
        assert!(stats.total_size_bytes > 0);
    }
}
