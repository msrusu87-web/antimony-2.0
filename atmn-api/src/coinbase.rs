// Coinbase transaction validation module
// Validates block rewards according to the ATMN halving schedule

use anyhow::{Result, anyhow};

/// Block reward schedule for ATMN
/// - Blocks 1-525,600 (Year 1): 50 ATMN per block
/// - Blocks 525,601-1,051,200 (Year 2): 25 ATMN per block
/// - Blocks 1,051,201-2,628,000 (Year 2-3): 12.5 ATMN per block
/// - Blocks 2,628,001+ (Year 4+): 6.25 ATMN per block

const INITIAL_REWARD: f64 = 50.0;
const HALVING_1_HEIGHT: u64 = 525_600;   // After Year 1
const HALVING_2_HEIGHT: u64 = 1_051_200; // After Year 2
const HALVING_3_HEIGHT: u64 = 2_628_000; // After Year 3

/// Calculate the correct block reward for a given block height
pub fn calculate_block_reward(height: u64) -> f64 {
    if height == 0 {
        // Genesis block has no reward
        return 0.0;
    } else if height <= HALVING_1_HEIGHT {
        // Year 1: 50 ATMN
        INITIAL_REWARD
    } else if height <= HALVING_2_HEIGHT {
        // Year 2: 25 ATMN (first halving)
        INITIAL_REWARD / 2.0
    } else if height <= HALVING_3_HEIGHT {
        // Year 2-3: 12.5 ATMN (second halving)
        INITIAL_REWARD / 4.0
    } else {
        // Year 4+: 6.25 ATMN (third halving, permanent)
        INITIAL_REWARD / 8.0
    }
}

/// Validate that a coinbase transaction has the correct reward amount
/// 
/// # Arguments
/// * `height` - Block height being validated
/// * `coinbase_amount` - Amount claimed in the coinbase transaction
/// * `total_fees` - Sum of all transaction fees in the block
/// 
/// # Returns
/// * `Ok(())` if validation passes
/// * `Err` with description if validation fails
pub fn validate_coinbase_reward(
    height: u64,
    coinbase_amount: f64,
    total_fees: f64,
) -> Result<()> {
    let expected_reward = calculate_block_reward(height);
    let max_allowed = expected_reward + total_fees;
    
    // Allow small floating point tolerance (0.00001 ATMN)
    let tolerance = 0.00001;
    
    if coinbase_amount > max_allowed + tolerance {
        return Err(anyhow!(
            "Coinbase amount {} exceeds maximum allowed {} (reward: {}, fees: {})",
            coinbase_amount,
            max_allowed,
            expected_reward,
            total_fees
        ));
    }
    
    // Miners are allowed to take less than the full reward (burning coins)
    // Only validate they don't take MORE than allowed
    
    Ok(())
}

/// Get the total emission for a block range
pub fn calculate_total_emission(start_height: u64, end_height: u64) -> f64 {
    let mut total = 0.0;
    for height in start_height..=end_height {
        total += calculate_block_reward(height);
    }
    total
}

/// Get reward era information for a given height
pub fn get_reward_era(height: u64) -> (u64, &'static str, f64) {
    if height <= HALVING_1_HEIGHT {
        (1, "Year 1", 50.0)
    } else if height <= HALVING_2_HEIGHT {
        (2, "Year 2", 25.0)
    } else if height <= HALVING_3_HEIGHT {
        (3, "Year 2-3", 12.5)
    } else {
        (4, "Year 4+", 6.25)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_rewards() {
        // Genesis block
        assert_eq!(calculate_block_reward(0), 0.0);
        
        // Year 1
        assert_eq!(calculate_block_reward(1), 50.0);
        assert_eq!(calculate_block_reward(525_600), 50.0);
        
        // Year 2 (first halving)
        assert_eq!(calculate_block_reward(525_601), 25.0);
        assert_eq!(calculate_block_reward(1_051_200), 25.0);
        
        // Year 2-3 (second halving)
        assert_eq!(calculate_block_reward(1_051_201), 12.5);
        assert_eq!(calculate_block_reward(2_628_000), 12.5);
        
        // Year 4+ (third halving)
        assert_eq!(calculate_block_reward(2_628_001), 6.25);
        assert_eq!(calculate_block_reward(10_000_000), 6.25);
    }

    #[test]
    fn test_coinbase_validation() {
        // Valid coinbase with no fees
        assert!(validate_coinbase_reward(1, 50.0, 0.0).is_ok());
        
        // Valid coinbase with fees
        assert!(validate_coinbase_reward(1, 50.5, 0.5).is_ok());
        
        // Too much reward claimed
        assert!(validate_coinbase_reward(1, 51.0, 0.0).is_err());
        
        // Miners are allowed to take less (coin burning)
        assert!(validate_coinbase_reward(1, 45.0, 0.0).is_ok());
        
        // Correct reward for halving periods
        assert!(validate_coinbase_reward(525_601, 25.0, 0.0).is_ok());
        assert!(validate_coinbase_reward(1_051_201, 12.5, 0.0).is_ok());
        assert!(validate_coinbase_reward(2_628_001, 6.25, 0.0).is_ok());
    }

    #[test]
    fn test_total_emission() {
        // First 100 blocks of year 1
        let emission = calculate_total_emission(1, 100);
        assert_eq!(emission, 5000.0); // 100 blocks * 50 ATMN
        
        // Across halving boundary
        let emission = calculate_total_emission(525_600, 525_601);
        assert_eq!(emission, 75.0); // 50 + 25
    }

    #[test]
    fn test_reward_era() {
        assert_eq!(get_reward_era(1).0, 1);
        assert_eq!(get_reward_era(525_601).0, 2);
        assert_eq!(get_reward_era(1_051_201).0, 3);
        assert_eq!(get_reward_era(2_628_001).0, 4);
    }
}
