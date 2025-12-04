// atmn-core/src/storage.rs
// Database abstraction for blockchain state

use crate::error::Result;

/// Database abstraction
pub struct Database {
    // TODO: Add RocksDB connection
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        // TODO: Initialize RocksDB
        Ok(Database {})
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        // TODO: Implement get
        Ok(None)
    }

    pub fn put(&self, key: Vec<u8>, value: Vec<u8>) -> Result<()> {
        // TODO: Implement put
        Ok(())
    }

    pub fn delete(&self, key: &[u8]) -> Result<()> {
        // TODO: Implement delete
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_creation() {
        // Note: In real tests, use temporary directories
        let _db = Database::new("/tmp/atmn_test").unwrap();
    }
}
