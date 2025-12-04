// atmn-core/src/chain_params.rs
// ANTIMONY COIN 2.0 - Chain Parameters and Constants

use serde::{Deserialize, Serialize};
use crate::types::{Amount, Timestamp, BlockHeight};

/// ATMN Chain Parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainParams {
    /// Network ID
    pub network_id: u32,
    
    /// Magic bytes for network identification
    pub magic_bytes: [u8; 4],
    
    /// P2P Port
    pub p2p_port: u16,
    
    /// RPC Port
    pub rpc_port: u16,
    
    /// Block time in seconds
    pub block_time: u32,
    
    /// Target timespan in seconds for difficulty adjustment
    pub target_timespan: u32,
    
    /// Maximum block size in bytes
    pub max_block_size: u32,
    
    /// Maximum transaction size in bytes
    pub max_tx_size: u32,
    
    /// Total coin supply in satoshis (1 ATMN = 100M satoshis)
    pub total_supply: u64,
    
    /// Genesis block subsidy in satoshis
    pub genesis_subsidy: u64,
    
    /// Block maturity in confirmations
    pub block_maturity: u32,
    
    /// Proof of work limit
    pub pow_limit: [u8; 32],
    
    /// Genesis timestamp
    pub genesis_timestamp: Timestamp,
    
    /// Genesis block difficulty bits
    pub genesis_bits: u32,
    
    /// Genesis block nonce
    pub genesis_nonce: u32,
    
    /// Maximum reorganization depth
    pub max_reorg_depth: u32,
}

impl ChainParams {
    /// Mainnet parameters
    pub fn mainnet() -> Self {
        ChainParams {
            network_id: 7676,
            magic_bytes: [0xa7, 0xc2, 0xd2, 0xf9],
            p2p_port: 7676,
            rpc_port: 7674,
            block_time: 12,  // 12 seconds
            target_timespan: 20 * 60,  // 20 minutes
            max_block_size: 8 * 1024 * 1024,  // 8 MB
            max_tx_size: 1024 * 1024,  // 1 MB
            total_supply: 500_000_000 * SATOSHI_PER_ATMN,  // 500M ATMN
            genesis_subsidy: 50_000_000 * SATOSHI_PER_ATMN,  // 50M ATMN
            block_maturity: 100,
            pow_limit: POW_LIMIT_MAINNET,
            genesis_timestamp: 1_704_067_200,  // Jan 1, 2024
            genesis_bits: 0x1d00ffff,
            genesis_nonce: 0,
            max_reorg_depth: 100,
        }
    }
    
    /// Testnet parameters
    pub fn testnet() -> Self {
        ChainParams {
            network_id: 17676,
            magic_bytes: [0x09, 0x11, 0x05, 0x88],
            p2p_port: 17676,
            rpc_port: 17674,
            block_time: 12,
            target_timespan: 20 * 60,
            max_block_size: 8 * 1024 * 1024,
            max_tx_size: 1024 * 1024,
            total_supply: 500_000_000 * SATOSHI_PER_ATMN,
            genesis_subsidy: 50_000_000 * SATOSHI_PER_ATMN,
            block_maturity: 100,
            pow_limit: POW_LIMIT_TESTNET,
            genesis_timestamp: 1_704_067_200,
            genesis_bits: 0x1d00ffff,
            genesis_nonce: 0,
            max_reorg_depth: 100,
        }
    }
    
    /// Regtest parameters (for testing)
    pub fn regtest() -> Self {
        ChainParams {
            network_id: 18332,
            magic_bytes: [0xfa, 0xbf, 0xb5, 0xda],
            p2p_port: 18444,
            rpc_port: 18332,
            block_time: 1,  // 1 second for testing
            target_timespan: 10 * 60,
            max_block_size: 8 * 1024 * 1024,
            max_tx_size: 1024 * 1024,
            total_supply: 500_000_000 * SATOSHI_PER_ATMN,
            genesis_subsidy: 50_000_000 * SATOSHI_PER_ATMN,
            block_maturity: 1,
            pow_limit: POW_LIMIT_REGTEST,
            genesis_timestamp: 1_704_067_200,
            genesis_bits: 0x207fffff,
            genesis_nonce: 0,
            max_reorg_depth: 100,
        }
    }
    
    /// Get block reward for given height (Pure PoW model)
    pub fn get_block_reward(&self, height: BlockHeight) -> Amount {
        match height {
            0 => self.genesis_subsidy,
            1..=525_600 => 50 * SATOSHI_PER_ATMN,  // Year 1: 50 ATMN
            525_601..=1_051_200 => 25 * SATOSHI_PER_ATMN,  // Year 2: 25 ATMN
            1_051_201..=2_628_000 => 12_500_000_000,  // Year 3: 12.5 ATMN (1250M satoshi)
            _ => 6_250_000_000,  // Year 4+: 6.25 ATMN (indefinite)
        }
    }
    
    /// Check if height is in Proof-of-Work phase (all blocks use PoW)
    pub fn is_pow_phase(&self, height: BlockHeight) -> bool {
        true  // Pure PoW indefinitely
    }

}

// Constants
pub const SATOSHI_PER_ATMN: u64 = 100_000_000;  // 1 ATMN = 100M satoshis

/// PoW limit for mainnet
pub const POW_LIMIT_MAINNET: [u8; 32] = [
    0x00, 0x00, 0x0f, 0xff, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

/// PoW limit for testnet
pub const POW_LIMIT_TESTNET: [u8; 32] = [
    0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
];

/// PoW limit for regtest
pub const POW_LIMIT_REGTEST: [u8; 32] = [
    0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
];

/// Block reward distribution (Pure PoW only)
pub const REWARD_POW_PERCENTAGE: u32 = 100;  // 100% to PoW miners

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mainnet_params() {
        let params = ChainParams::mainnet();
        assert_eq!(params.network_id, 7676);
        assert_eq!(params.block_time, 12);
        assert_eq!(params.get_block_reward(0), 50_000_000 * SATOSHI_PER_ATMN);
    }

    #[test]
    fn test_block_rewards_schedule() {
        let params = ChainParams::mainnet();
        
        // Year 1
        assert_eq!(params.get_block_reward(1), 50 * SATOSHI_PER_ATMN);
        assert_eq!(params.get_block_reward(525_600), 50 * SATOSHI_PER_ATMN);
        
        // Year 2
        assert_eq!(params.get_block_reward(525_601), 25 * SATOSHI_PER_ATMN);
        assert_eq!(params.get_block_reward(1_051_200), 25 * SATOSHI_PER_ATMN);
        
        // Year 3+
        assert_eq!(params.get_block_reward(1_051_201), 12_500_000_000);
    }

    #[test]
    fn test_pow_phase() {
        let params = ChainParams::mainnet();
        
        // Pure PoW: all heights are in PoW phase
        assert!(params.is_pow_phase(0));
        assert!(params.is_pow_phase(5_256_000));
        assert!(params.is_pow_phase(10_000_000));
    }
}
