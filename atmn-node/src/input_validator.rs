/// Input validation for all user-provided data
use std::net::IpAddr;

pub struct InputValidator;

impl InputValidator {
    /// Validate ATMN address format
    pub fn validate_address(addr: &str) -> bool {
        // ATMN addresses start with "atmn1" and are 42-62 chars
        if !addr.starts_with("atmn1") {
            return false;
        }
        
        let len = addr.len();
        if len < 42 || len > 62 {
            return false;
        }
        
        // Check for valid characters (bech32)
        addr.chars().all(|c| {
            c.is_ascii_lowercase() || c.is_ascii_digit()
        })
    }

    /// Validate transaction amount
    pub fn validate_amount(amount: i64) -> bool {
        amount > 0 && amount <= 500_000_000_00_000_000 // Max supply
    }

    /// Sanitize string input
    pub fn sanitize_string(input: &str, max_len: usize) -> String {
        input
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == '-' || *c == '_')
            .take(max_len)
            .collect()
    }

    /// Validate IP address
    pub fn validate_ip(ip: &str) -> bool {
        ip.parse::<IpAddr>().is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_validation() {
        assert!(InputValidator::validate_address("atmn1qyqszqgpqyqszqgpqyqszqgpqyqszqgpq5n3xd2"));
        assert!(!InputValidator::validate_address("invalid"));
        assert!(!InputValidator::validate_address("btc1qyqszqgpq"));
    }

    #[test]
    fn test_amount_validation() {
        assert!(InputValidator::validate_amount(1000000));
        assert!(!InputValidator::validate_amount(-100));
        assert!(!InputValidator::validate_amount(0));
    }
}
