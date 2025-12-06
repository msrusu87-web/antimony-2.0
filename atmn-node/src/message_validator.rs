/// P2P Message validation
use serde::{Deserialize, Serialize};

pub const MAX_MESSAGE_SIZE: usize = 32 * 1024 * 1024; // 32 MB
pub const MAX_BLOCK_SIZE: usize = 4 * 1024 * 1024;    // 4 MB
pub const MAX_TX_SIZE: usize = 1 * 1024 * 1024;       // 1 MB

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Handshake,
    Block,
    Transaction,
    GetBlocks,
    GetData,
    Ping,
    Pong,
}

pub struct MessageValidator;

impl MessageValidator {
    /// Validate message size
    pub fn validate_size(data: &[u8], msg_type: &MessageType) -> bool {
        let max_size = match msg_type {
            MessageType::Block => MAX_BLOCK_SIZE,
            MessageType::Transaction => MAX_TX_SIZE,
            _ => MAX_MESSAGE_SIZE,
        };
        
        data.len() <= max_size
    }

    /// Validate message structure
    pub fn validate_structure(data: &[u8]) -> bool {
        // Check minimum size
        if data.len() < 4 {
            return false;
        }
        
        // Check magic bytes (first 4 bytes)
        let magic = &data[0..4];
        magic == &[0x0B, 0x11, 0x09, 0x07] || // Testnet
        magic == &[0xF9, 0xBE, 0xB4, 0xD9]    // Mainnet
    }

    /// Check for malformed data
    pub fn detect_malformed(data: &[u8]) -> bool {
        // Check for NULL bytes in unexpected places
        if data.is_empty() {
            return true;
        }
        
        // Check for excessive repetition (possible attack)
        if data.len() > 100 {
            let first_byte = data[0];
            let same_count = data.iter().filter(|&&b| b == first_byte).count();
            if same_count > data.len() * 95 / 100 {
                return true; // > 95% same byte
            }
        }
        
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_validation() {
        let data = vec![0u8; 1000];
        assert!(MessageValidator::validate_size(&data, &MessageType::Transaction));
        
        let large_data = vec![0u8; 10 * 1024 * 1024];
        assert!(!MessageValidator::validate_size(&large_data, &MessageType::Transaction));
    }

    #[test]
    fn test_malformed_detection() {
        let normal_data = vec![1, 2, 3, 4, 5];
        assert!(!MessageValidator::detect_malformed(&normal_data));
        
        let malformed = vec![0xFF; 1000];
        assert!(MessageValidator::detect_malformed(&malformed));
    }
}
