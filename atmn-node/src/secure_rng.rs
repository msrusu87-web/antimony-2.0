/// Secure Random Number Generator
use rand::rngs::OsRng;
use rand::RngCore;

pub struct SecureRng;

impl SecureRng {
    /// Generate secure random bytes
    pub fn generate_bytes(len: usize) -> Vec<u8> {
        let mut rng = OsRng;
        let mut bytes = vec![0u8; len];
        rng.fill_bytes(&mut bytes);
        bytes
    }

    /// Generate secure random u64
    pub fn generate_u64() -> u64 {
        let mut rng = OsRng;
        rng.next_u64()
    }

    /// Generate secure random nonce
    pub fn generate_nonce() -> [u8; 32] {
        let mut rng = OsRng;
        let mut nonce = [0u8; 32];
        rng.fill_bytes(&mut nonce);
        nonce
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_bytes() {
        let bytes1 = SecureRng::generate_bytes(32);
        let bytes2 = SecureRng::generate_bytes(32);
        
        assert_eq!(bytes1.len(), 32);
        assert_eq!(bytes2.len(), 32);
        assert_ne!(bytes1, bytes2); // Should be different
    }

    #[test]
    fn test_generate_u64() {
        let n1 = SecureRng::generate_u64();
        let n2 = SecureRng::generate_u64();
        
        assert_ne!(n1, n2); // Should be different
    }
}
