// Genesis block creation for Antimony blockchain
use crate::{Block, Transaction};
use crate::block::BlockHeader;
use crate::types::BlockHash;
use crate::error::Result;
use crate::Storage;

/// Create the genesis block
pub fn create_genesis_block() -> Block {
    Block {
        header: BlockHeader {
            version: 1,
            prev_block_hash: BlockHash::zero(),
            merkle_root: BlockHash::zero(),
            timestamp: 1701657600, // December 4, 2023 00:00:00 UTC
            bits: 0x1d00ffff, // Initial difficulty
            nonce: 0,
        },
        transactions: vec![],
        height: 0,
    }
}

/// Initialize storage with genesis block
pub fn initialize_genesis(storage: &Storage) -> Result<()> {
    // Check if genesis already exists
    if storage.get_block(0)?.is_some() {
        return Ok(());
    }
    
    let genesis = create_genesis_block();
    storage.put_block(0, &genesis)?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_genesis_creation() {
        let genesis = create_genesis_block();
        assert_eq!(genesis.height, 0);
        assert_eq!(genesis.header.version, 1);
        assert_eq!(genesis.transactions.len(), 0);
    }

    #[test]
    fn test_genesis_initialization() {
        let temp_dir = TempDir::new().unwrap();
        let storage = Storage::new(temp_dir.path()).unwrap();
        
        initialize_genesis(&storage).unwrap();
        
        let genesis = storage.get_block(0).unwrap();
        assert!(genesis.is_some());
        assert_eq!(genesis.unwrap().height, 0);
    }

    #[test]
    fn test_genesis_idempotent() {
        let temp_dir = TempDir::new().unwrap();
        let storage = Storage::new(temp_dir.path()).unwrap();
        
        // Initialize twice
        initialize_genesis(&storage).unwrap();
        initialize_genesis(&storage).unwrap();
        
        // Should still have only one genesis block
        let genesis = storage.get_block(0).unwrap();
        assert!(genesis.is_some());
    }
}
