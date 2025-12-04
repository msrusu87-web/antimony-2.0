// atmn-core/src/types.rs
// Core Data Types for Antimony Blockchain

use serde::{Deserialize, Serialize};
use std::fmt;

/// Block Hash - SHA256 double hash
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlockHash(pub [u8; 32]);

impl BlockHash {
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        BlockHash(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn zero() -> Self {
        BlockHash([0u8; 32])
    }
}

impl fmt::Display for BlockHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.0))
    }
}

/// Transaction Hash
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TxHash([u8; 32]);

impl TxHash {
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        TxHash(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Display for TxHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.0))
    }
}

/// Block Height (block number)
pub type BlockHeight = u64;

/// Amount in smallest unit (1 ATMN = 100,000,000 units)
pub type Amount = u64;

/// Difficulty target (as bits - compact form)
pub type Difficulty = u32;

/// Timestamp (seconds since epoch)
pub type Timestamp = u32;

/// Nonce for PoW
pub type Nonce = u32;

/// Account address (20 bytes for EVM compatibility)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address([u8; 20]);

impl Address {
    pub fn from_bytes(bytes: [u8; 20]) -> Self {
        Address(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 20] {
        &self.0
    }

    pub fn zero() -> Self {
        Address([0u8; 20])
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", hex::encode(&self.0))
    }
}

/// Public key (33 bytes compressed)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PublicKey(Vec<u8>);

impl PublicKey {
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        PublicKey(bytes)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// Signature (64 bytes R||S)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Signature(Vec<u8>);

impl Signature {
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Signature(bytes)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_hash_display() {
        let hash = BlockHash([0x01u8; 32]);
        let display = format!("{}", hash);
        assert!(display.contains("01"));
    }

    #[test]
    fn test_address_display() {
        let addr = Address([0x42u8; 20]);
        let display = format!("{}", addr);
        assert!(display.starts_with("0x"));
    }
}
