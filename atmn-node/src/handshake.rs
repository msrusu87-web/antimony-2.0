/// Handshake Validation for P2P Network
/// Ensures secure peer authentication and protocol compatibility
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

/// Protocol version
pub const PROTOCOL_VERSION: u32 = 1;

/// Network magic bytes (mainnet vs testnet)
pub const MAINNET_MAGIC: [u8; 4] = [0xF9, 0xBE, 0xB4, 0xD9];
pub const TESTNET_MAGIC: [u8; 4] = [0x0B, 0x11, 0x09, 0x07];

/// Handshake message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeMessage {
    /// Protocol version
    pub version: u32,
    /// Network magic
    pub magic: [u8; 4],
    /// Node ID (random 64-bit)
    pub node_id: u64,
    /// Timestamp
    pub timestamp: u64,
    /// Best block height
    pub best_height: u64,
    /// Challenge nonce for authentication
    pub nonce: [u8; 32],
    /// User agent
    pub user_agent: String,
}

impl HandshakeMessage {
    pub fn new(node_id: u64, best_height: u64, is_testnet: bool) -> Self {
        let magic = if is_testnet {
            TESTNET_MAGIC
        } else {
            MAINNET_MAGIC
        };

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Generate random nonce
        let mut nonce = [0u8; 32];
        for i in 0..32 {
            nonce[i] = (timestamp as u8).wrapping_add(i as u8);
        }

        Self {
            version: PROTOCOL_VERSION,
            magic,
            node_id,
            timestamp,
            best_height,
            nonce,
            user_agent: "ATMN/2.0".to_string(),
        }
    }

    /// Compute message hash for verification
    pub fn compute_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(&self.version.to_le_bytes());
        hasher.update(&self.magic);
        hasher.update(&self.node_id.to_le_bytes());
        hasher.update(&self.timestamp.to_le_bytes());
        hasher.update(&self.best_height.to_le_bytes());
        hasher.update(&self.nonce);
        hasher.update(self.user_agent.as_bytes());
        
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
    }
}

/// Handshake response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeResponse {
    /// Response to challenge nonce
    pub response: [u8; 32],
    /// Peer's own handshake
    pub handshake: HandshakeMessage,
}

/// Handshake validator
pub struct HandshakeValidator {
    testnet: bool,
    min_version: u32,
    max_timestamp_drift: u64, // seconds
}

impl HandshakeValidator {
    pub fn new(testnet: bool) -> Self {
        Self {
            testnet,
            min_version: 1,
            max_timestamp_drift: 300, // 5 minutes
        }
    }

    /// Validate incoming handshake
    pub fn validate(&self, handshake: &HandshakeMessage) -> Result<(), HandshakeError> {
        // Check protocol version
        if handshake.version < self.min_version {
            return Err(HandshakeError::UnsupportedVersion(handshake.version));
        }

        // Check network magic
        let expected_magic = if self.testnet {
            TESTNET_MAGIC
        } else {
            MAINNET_MAGIC
        };

        if handshake.magic != expected_magic {
            return Err(HandshakeError::WrongNetwork);
        }

        // Check timestamp (prevent replay attacks)
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let time_diff = if now > handshake.timestamp {
            now - handshake.timestamp
        } else {
            handshake.timestamp - now
        };

        if time_diff > self.max_timestamp_drift {
            return Err(HandshakeError::InvalidTimestamp(time_diff));
        }

        // Check user agent
        if handshake.user_agent.is_empty() || handshake.user_agent.len() > 100 {
            return Err(HandshakeError::InvalidUserAgent);
        }

        Ok(())
    }

    /// Create challenge response
    pub fn create_response(&self, challenge: &HandshakeMessage) -> [u8; 32] {
        // Hash the challenge nonce to create response
        let mut hasher = Sha256::new();
        hasher.update(&challenge.nonce);
        hasher.update(&challenge.node_id.to_le_bytes());
        
        let result = hasher.finalize();
        let mut response = [0u8; 32];
        response.copy_from_slice(&result);
        response
    }

    /// Verify challenge response
    pub fn verify_response(
        &self,
        original_challenge: &HandshakeMessage,
        response: &[u8; 32],
    ) -> bool {
        let expected = self.create_response(original_challenge);
        expected == *response
    }
}

/// Handshake errors
#[derive(Debug, Clone)]
pub enum HandshakeError {
    UnsupportedVersion(u32),
    WrongNetwork,
    InvalidTimestamp(u64),
    InvalidUserAgent,
    InvalidResponse,
    Timeout,
}

impl std::fmt::Display for HandshakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandshakeError::UnsupportedVersion(v) => {
                write!(f, "Unsupported protocol version: {}", v)
            }
            HandshakeError::WrongNetwork => write!(f, "Wrong network (mainnet/testnet)"),
            HandshakeError::InvalidTimestamp(drift) => {
                write!(f, "Invalid timestamp (drift: {}s)", drift)
            }
            HandshakeError::InvalidUserAgent => write!(f, "Invalid user agent"),
            HandshakeError::InvalidResponse => write!(f, "Invalid challenge response"),
            HandshakeError::Timeout => write!(f, "Handshake timeout"),
        }
    }
}

impl std::error::Error for HandshakeError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handshake_creation() {
        let handshake = HandshakeMessage::new(12345, 4032, true);
        
        assert_eq!(handshake.version, PROTOCOL_VERSION);
        assert_eq!(handshake.magic, TESTNET_MAGIC);
        assert_eq!(handshake.node_id, 12345);
        assert_eq!(handshake.best_height, 4032);
    }

    #[test]
    fn test_handshake_validation() {
        let validator = HandshakeValidator::new(true);
        let handshake = HandshakeMessage::new(12345, 4032, true);
        
        assert!(validator.validate(&handshake).is_ok());
    }

    #[test]
    fn test_wrong_network() {
        let validator = HandshakeValidator::new(false); // Mainnet validator
        let handshake = HandshakeMessage::new(12345, 4032, true); // Testnet handshake
        
        assert!(matches!(
            validator.validate(&handshake),
            Err(HandshakeError::WrongNetwork)
        ));
    }

    #[test]
    fn test_challenge_response() {
        let validator = HandshakeValidator::new(true);
        let challenge = HandshakeMessage::new(12345, 4032, true);
        
        let response = validator.create_response(&challenge);
        assert!(validator.verify_response(&challenge, &response));
    }

    #[test]
    fn test_invalid_response() {
        let validator = HandshakeValidator::new(true);
        let challenge = HandshakeMessage::new(12345, 4032, true);
        
        let wrong_response = [0u8; 32];
        assert!(!validator.verify_response(&challenge, &wrong_response));
    }
}
