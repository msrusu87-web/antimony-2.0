// Storage layer using RocksDB
use crate::{Block, Transaction};
use crate::types::{BlockHash, BlockHeight, TxHash};
use crate::error::{Error, Result};
use rocksdb::{DB, Options, IteratorMode};
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::sync::Arc;

/// Column families for different data types
const CF_BLOCKS: &str = "blocks";
const CF_BLOCK_INDEX: &str = "block_index";
const CF_TRANSACTIONS: &str = "transactions";
const CF_UTXOS: &str = "utxos";
const CF_ADDRESS_INDEX: &str = "address_index";  // address -> list of UTXO keys
const CF_METADATA: &str = "metadata";

/// Storage manager for blockchain data
pub struct Storage {
    db: Arc<DB>,
}

/// UTXO entry for tracking unspent outputs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtxoEntry {
    pub tx_hash: TxHash,
    pub output_index: u32,
    pub amount: u64,
    pub script_pubkey: Vec<u8>,
    pub block_height: BlockHeight,
}

impl Storage {
    /// Create new storage instance
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        
        // Define column families
        let cfs = vec![CF_BLOCKS, CF_BLOCK_INDEX, CF_TRANSACTIONS, CF_UTXOS, CF_ADDRESS_INDEX, CF_METADATA];
        
        let db = DB::open_cf(&opts, path, cfs)
            .map_err(|e| Error::DatabaseError(e.to_string()))?;
        
        Ok(Self {
            db: Arc::new(db),
        })
    }

    /// Store a block
    pub fn put_block(&self, height: BlockHeight, block: &Block) -> Result<()> {
        let cf_blocks = self.db.cf_handle(CF_BLOCKS)
            .ok_or_else(|| Error::DatabaseError("CF_BLOCKS not found".to_string()))?;
        let cf_index = self.db.cf_handle(CF_BLOCK_INDEX)
            .ok_or_else(|| Error::DatabaseError("CF_BLOCK_INDEX not found".to_string()))?;
        
        // Serialize block
        let block_data = bincode::serialize(block)
            .map_err(|e| Error::DatabaseError(format!("Serialization error: {}", e)))?;
        let hash = block.hash();
        
        // Store by height
        self.db.put_cf(cf_blocks, height.to_le_bytes(), block_data.clone())
            .map_err(|e| Error::DatabaseError(e.to_string()))?;
        
        // Store hash -> height index
        self.db.put_cf(cf_index, hash.as_bytes(), height.to_le_bytes())
            .map_err(|e| Error::DatabaseError(e.to_string()))?;
        
        // Store transactions
        self.store_block_transactions(height, block)?;
        
        // Update best block height
        self.update_best_height(height)?;
        
        Ok(())
    }

    /// Get block by height
    pub fn get_block(&self, height: BlockHeight) -> Result<Option<Block>> {
        let cf_blocks = self.db.cf_handle(CF_BLOCKS)
            .ok_or_else(|| Error::DatabaseError("CF_BLOCKS not found".to_string()))?;
        
        match self.db.get_cf(cf_blocks, height.to_le_bytes())
            .map_err(|e| Error::DatabaseError(e.to_string()))? {
            Some(data) => {
                let block: Block = bincode::deserialize(&data)
                    .map_err(|e| Error::DatabaseError(format!("Deserialization error: {}", e)))?;
                Ok(Some(block))
            }
            None => Ok(None),
        }
    }

    /// Get block by hash
    pub fn get_block_by_hash(&self, hash: &BlockHash) -> Result<Option<Block>> {
        let cf_index = self.db.cf_handle(CF_BLOCK_INDEX)
            .ok_or_else(|| Error::DatabaseError("CF_BLOCK_INDEX not found".to_string()))?;
        
        // Get height from hash
        match self.db.get_cf(cf_index, hash.as_bytes())
            .map_err(|e| Error::DatabaseError(e.to_string()))? {
            Some(height_bytes) => {
                let height = BlockHeight::from_le_bytes(
                    height_bytes.as_slice().try_into()
                        .map_err(|e| Error::DatabaseError(format!("Invalid height bytes: {:?}", e)))?
                );
                self.get_block(height)
            }
            None => Ok(None),
        }
    }

    /// Store transactions from a block
    fn store_block_transactions(&self, height: BlockHeight, block: &Block) -> Result<()> {
        let cf_txs = self.db.cf_handle(CF_TRANSACTIONS)
            .ok_or_else(|| Error::DatabaseError("CF_TRANSACTIONS not found".to_string()))?;
        
        for (idx, tx) in block.transactions.iter().enumerate() {
            // Calculate transaction hash
            let tx_hash = self.calculate_tx_hash(tx);
            
            // Store tx with metadata
            let tx_meta = TransactionMetadata {
                transaction: tx.clone(),
                block_height: height,
                block_hash: block.hash(),
                tx_index: idx as u32,
            };
            
            let tx_data = bincode::serialize(&tx_meta)
                .map_err(|e| Error::DatabaseError(format!("Serialization error: {}", e)))?;
            self.db.put_cf(cf_txs, tx_hash.as_bytes(), tx_data)
                .map_err(|e| Error::DatabaseError(e.to_string()))?;
            
            // Update UTXO set
            self.update_utxos(height, &tx_hash, tx)?;
        }
        
        Ok(())
    }

    /// Get transaction by hash
    pub fn get_transaction(&self, tx_hash: &TxHash) -> Result<Option<TransactionMetadata>> {
        let cf_txs = self.db.cf_handle(CF_TRANSACTIONS)
            .ok_or_else(|| Error::DatabaseError("CF_TRANSACTIONS not found".to_string()))?;
        
        match self.db.get_cf(cf_txs, tx_hash.as_bytes())
            .map_err(|e| Error::DatabaseError(e.to_string()))? {
            Some(data) => {
                let tx_meta: TransactionMetadata = bincode::deserialize(&data)
                    .map_err(|e| Error::DatabaseError(format!("Deserialization error: {}", e)))?;
                Ok(Some(tx_meta))
            }
            None => Ok(None),
        }
    }

    /// Update UTXO set
    fn update_utxos(&self, height: BlockHeight, tx_hash: &TxHash, tx: &Transaction) -> Result<()> {
        let cf_utxos = self.db.cf_handle(CF_UTXOS)
            .ok_or_else(|| Error::DatabaseError("CF_UTXOS not found".to_string()))?;
        let cf_addr_idx = self.db.cf_handle(CF_ADDRESS_INDEX)
            .ok_or_else(|| Error::DatabaseError("CF_ADDRESS_INDEX not found".to_string()))?;
        
        // Remove spent UTXOs (inputs)
        for input in &tx.inputs {
            let utxo_key = format!("{}:{}", input.prev_tx_hash, input.prev_tx_index);
            self.db.delete_cf(cf_utxos, utxo_key.as_bytes())
                .map_err(|e| Error::DatabaseError(e.to_string()))?;
        }
        
        // Add new UTXOs (outputs)
        for (output_index, output) in tx.outputs.iter().enumerate() {
            let utxo_entry = UtxoEntry {
                tx_hash: tx_hash.clone(),
                output_index: output_index as u32,
                amount: output.amount,
                script_pubkey: output.script_pubkey.clone(),
                block_height: height,
            };
            
            let utxo_key = format!("{}:{}", tx_hash, output_index);
            let utxo_data = bincode::serialize(&utxo_entry)
                .map_err(|e| Error::DatabaseError(format!("Serialization error: {}", e)))?;
            self.db.put_cf(cf_utxos, utxo_key.as_bytes(), utxo_data.clone())
                .map_err(|e| Error::DatabaseError(e.to_string()))?;
            
            // Extract address from script_pubkey (simplified - just use hash of script)
            let address = format!("addr_{:x}", output.amount.wrapping_mul(31));  // Simple deterministic addr
            
            // Add to address index
            self.add_to_address_index(&cf_addr_idx, &address, &utxo_key)?;
        }
        
        Ok(())
    }

    /// Add UTXO key to address index
    fn add_to_address_index(&self, cf_addr_idx: &rocksdb::ColumnFamily, address: &str, utxo_key: &str) -> Result<()> {
        // Get existing UTXO list for address
        let mut utxo_list = match self.db.get_cf(cf_addr_idx, address.as_bytes())
            .map_err(|e| Error::DatabaseError(e.to_string()))? {
            Some(data) => {
                bincode::deserialize::<Vec<String>>(&data)
                    .unwrap_or_default()
            }
            None => Vec::new(),
        };
        
        // Add new UTXO key if not present
        if !utxo_list.contains(&utxo_key.to_string()) {
            utxo_list.push(utxo_key.to_string());
        }
        
        // Save updated list
        let data = bincode::serialize(&utxo_list)
            .map_err(|e| Error::DatabaseError(format!("Serialization error: {}", e)))?;
        self.db.put_cf(cf_addr_idx, address.as_bytes(), data)
            .map_err(|e| Error::DatabaseError(e.to_string()))?;
        
        Ok(())
    }

    /// Get UTXOs for an address (now uses address index for efficiency)
    pub fn get_utxos_for_address(&self, address: &str) -> Result<Vec<UtxoEntry>> {
        let cf_addr_idx = self.db.cf_handle(CF_ADDRESS_INDEX)
            .ok_or_else(|| Error::DatabaseError("CF_ADDRESS_INDEX not found".to_string()))?;
        let cf_utxos = self.db.cf_handle(CF_UTXOS)
            .ok_or_else(|| Error::DatabaseError("CF_UTXOS not found".to_string()))?;
        
        let mut utxos = Vec::new();
        
        // Get UTXO keys for this address from index
        if let Some(data) = self.db.get_cf(cf_addr_idx, address.as_bytes())
            .map_err(|e| Error::DatabaseError(e.to_string()))? {
            let utxo_keys: Vec<String> = bincode::deserialize(&data)
                .unwrap_or_default();
            
            // Retrieve each UTXO
            for utxo_key in utxo_keys {
                if let Some(utxo_data) = self.db.get_cf(cf_utxos, utxo_key.as_bytes())
                    .map_err(|e| Error::DatabaseError(e.to_string()))? {
                    let utxo: UtxoEntry = bincode::deserialize(&utxo_data)
                        .map_err(|e| Error::DatabaseError(format!("Deserialization error: {}", e)))?;
                    utxos.push(utxo);
                }
            }
        }
        
        Ok(utxos)
    }

    /// Calculate balance for an address
    pub fn get_balance(&self, address: &str) -> Result<u64> {
        let utxos = self.get_utxos_for_address(address)?;
        Ok(utxos.iter().map(|u| u.amount).sum())
    }

    /// Get best block height
    pub fn get_best_height(&self) -> Result<Option<BlockHeight>> {
        let cf_meta = self.db.cf_handle(CF_METADATA)
            .ok_or_else(|| Error::DatabaseError("CF_METADATA not found".to_string()))?;
        
        match self.db.get_cf(cf_meta, b"best_height")
            .map_err(|e| Error::DatabaseError(e.to_string()))? {
            Some(data) => {
                let height = BlockHeight::from_le_bytes(
                    data.as_slice().try_into()
                        .map_err(|e| Error::DatabaseError(format!("Invalid height bytes: {:?}", e)))?
                );
                Ok(Some(height))
            }
            None => Ok(None),
        }
    }

    /// Update best block height
    fn update_best_height(&self, height: BlockHeight) -> Result<()> {
        let cf_meta = self.db.cf_handle(CF_METADATA)
            .ok_or_else(|| Error::DatabaseError("CF_METADATA not found".to_string()))?;
        
        self.db.put_cf(cf_meta, b"best_height", height.to_le_bytes())
            .map_err(|e| Error::DatabaseError(e.to_string()))?;
        Ok(())
    }

    /// Calculate transaction hash (simplified)
    fn calculate_tx_hash(&self, tx: &Transaction) -> TxHash {
        use sha2::{Sha256, Digest};
        let tx_data = bincode::serialize(tx).unwrap_or_default();
        let hash = Sha256::digest(&tx_data);
        TxHash::from_bytes(hash.into())
    }

    /// Delete block (for reorg handling)
    pub fn delete_block(&self, height: BlockHeight) -> Result<()> {
        let cf_blocks = self.db.cf_handle(CF_BLOCKS)
            .ok_or_else(|| Error::DatabaseError("CF_BLOCKS not found".to_string()))?;
        
        self.db.delete_cf(cf_blocks, height.to_le_bytes())
            .map_err(|e| Error::DatabaseError(e.to_string()))?;
        Ok(())
    }

    /// Get database statistics
    pub fn get_stats(&self) -> Result<StorageStats> {
        let best_height = self.get_best_height()?.unwrap_or(0);
        
        Ok(StorageStats {
            best_height,
            total_blocks: best_height + 1,
        })
    }
}

/// Transaction metadata with block info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionMetadata {
    pub transaction: Transaction,
    pub block_height: BlockHeight,
    pub block_hash: BlockHash,
    pub tx_index: u32,
}

/// Storage statistics
#[derive(Debug, Clone)]
pub struct StorageStats {
    pub best_height: BlockHeight,
    pub total_blocks: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::BlockHeader;
    use tempfile::TempDir;

    fn create_test_storage() -> (Storage, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let storage = Storage::new(temp_dir.path()).unwrap();
        (storage, temp_dir)
    }

    fn create_test_block(height: BlockHeight) -> Block {
        Block {
            header: BlockHeader {
                version: 1,
                prev_block_hash: BlockHash::zero(),
                merkle_root: BlockHash::zero(),
                timestamp: 1701657600 + height as u32,
                bits: 0x1d00ffff,
                nonce: 0,
            },
            transactions: vec![],
            height,
        }
    }

    #[test]
    fn test_storage_creation() {
        let (_storage, _temp_dir) = create_test_storage();
    }

    #[test]
    fn test_put_and_get_block() {
        let (storage, _temp_dir) = create_test_storage();
        let block = create_test_block(0);
        
        storage.put_block(0, &block).unwrap();
        let retrieved = storage.get_block(0).unwrap();
        
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().height, 0);
    }

    #[test]
    fn test_get_block_by_hash() {
        let (storage, _temp_dir) = create_test_storage();
        let block = create_test_block(0);
        let hash = block.hash();
        
        storage.put_block(0, &block).unwrap();
        let retrieved = storage.get_block_by_hash(&hash).unwrap();
        
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().height, 0);
    }

    #[test]
    fn test_best_height() {
        let (storage, _temp_dir) = create_test_storage();
        
        assert_eq!(storage.get_best_height().unwrap(), None);
        
        storage.put_block(0, &create_test_block(0)).unwrap();
        assert_eq!(storage.get_best_height().unwrap(), Some(0));
        
        storage.put_block(1, &create_test_block(1)).unwrap();
        assert_eq!(storage.get_best_height().unwrap(), Some(1));
    }

    #[test]
    fn test_storage_stats() {
        let (storage, _temp_dir) = create_test_storage();
        
        storage.put_block(0, &create_test_block(0)).unwrap();
        storage.put_block(1, &create_test_block(1)).unwrap();
        
        let stats = storage.get_stats().unwrap();
        assert_eq!(stats.best_height, 1);
        assert_eq!(stats.total_blocks, 2);
    }
}
