// atmn-core/src/error.rs
// Error handling for Antimony blockchain

use std::fmt;

#[derive(Debug)]
pub enum Error {
    // Blockchain errors
    InvalidBlockHeight(u64),
    InvalidBlockHash,
    InvalidTransaction,
    InvalidBlockReward,
    InvalidDifficulty,
    InvalidProofOfWork,
    
    // Chain state errors
    ChainNotInitialized,
    OrphanBlock,
    DuplicateBlock,
    
    // Database errors
    DatabaseError(String),
    
    // Validation errors
    InvalidSignature,
    InvalidAddress,
    InsufficientBalance,
    
    // Network errors
    NetworkError(String),
    
    // Generic errors
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidBlockHeight(h) => write!(f, "Invalid block height: {}", h),
            Error::InvalidBlockHash => write!(f, "Invalid block hash"),
            Error::InvalidTransaction => write!(f, "Invalid transaction"),
            Error::InvalidBlockReward => write!(f, "Invalid block reward"),
            Error::InvalidDifficulty => write!(f, "Invalid difficulty"),
            Error::InvalidProofOfWork => write!(f, "Invalid proof of work"),
            Error::ChainNotInitialized => write!(f, "Blockchain not initialized"),
            Error::OrphanBlock => write!(f, "Orphan block received"),
            Error::DuplicateBlock => write!(f, "Duplicate block"),
            Error::DatabaseError(e) => write!(f, "Database error: {}", e),
            Error::InvalidSignature => write!(f, "Invalid signature"),
            Error::InvalidAddress => write!(f, "Invalid address"),
            Error::InsufficientBalance => write!(f, "Insufficient balance"),
            Error::NetworkError(e) => write!(f, "Network error: {}", e),
            Error::Other(e) => write!(f, "Error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::InvalidBlockHeight(12345);
        assert_eq!(format!("{}", err), "Invalid block height: 12345");
    }
}
