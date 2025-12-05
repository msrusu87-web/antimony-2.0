// atmn-core/src/error.rs
// Error handling for Antimony blockchain

use std::fmt;

#[derive(Debug)]
pub enum Error {
    // Blockchain errors
    InvalidBlockHeight(u64),
    InvalidBlockHash,
    InvalidBlock(String),
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
    InvalidAmount,
    
    // Transaction errors
    DuplicateTransaction,
    TransactionTooLarge,
    FeeTooLow,
    
    // Mempool errors
    MempoolFull,
    
    // Serialization errors
    SerializationError,
    
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
            Error::InvalidBlock(msg) => write!(f, "Invalid block: {}", msg),
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
            Error::InvalidAmount => write!(f, "Invalid amount"),
            Error::DuplicateTransaction => write!(f, "Duplicate transaction in mempool"),
            Error::TransactionTooLarge => write!(f, "Transaction size exceeds maximum"),
            Error::FeeTooLow => write!(f, "Transaction fee too low"),
            Error::MempoolFull => write!(f, "Mempool is full"),
            Error::SerializationError => write!(f, "Serialization error"),
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
    
    #[test]
    fn test_mempool_errors() {
        let err = Error::MempoolFull;
        assert_eq!(format!("{}", err), "Mempool is full");
        
        let err = Error::FeeTooLow;
        assert_eq!(format!("{}", err), "Transaction fee too low");
    }
}
