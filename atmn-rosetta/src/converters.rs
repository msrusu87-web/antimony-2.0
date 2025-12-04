// Converters between atmn-core types and Rosetta types
use atmn_core::Block;
use atmn_core::Transaction as CoreTransaction;
use atmn_core::types::BlockHash;
use crate::types::*;

/// Convert atmn-core Block to Rosetta Block
pub fn block_to_rosetta(block: &Block, height: u64) -> crate::types::Block {
    // Convert transactions
    let transactions = block
        .transactions
        .iter()
        .enumerate()
        .map(|(idx, tx)| transaction_to_rosetta(tx, idx, height))
        .collect();

    // Get block hash (using Display trait)
    let block_hash = block.hash().to_string();
    
    // Parent hash (for genesis, parent = self)
    let parent_hash = if height == 0 {
        block_hash.clone()
    } else {
        block.header.prev_block_hash.to_string()
    };

    crate::types::Block {
        block_identifier: BlockIdentifier::new(height, block_hash),
        parent_block_identifier: BlockIdentifier::new(
            height.saturating_sub(1),
            parent_hash,
        ),
        timestamp: (block.header.timestamp as i64) * 1000, // Convert to milliseconds
        transactions,
        metadata: Some(serde_json::json!({
            "difficulty": block.header.bits,
            "nonce": block.header.nonce,
            "version": block.header.version,
            "merkle_root": block.header.merkle_root.to_string(),
        })),
    }
}

/// Convert atmn-core Transaction to Rosetta Transaction
pub fn transaction_to_rosetta(
    tx: &CoreTransaction,
    tx_index: usize,
    block_height: u64,
) -> Transaction {
    let tx_hash = format!("{:064x}", tx_index); // Placeholder until we have real tx hashing
    
    let mut operations = Vec::new();
    let mut op_index = 0;

    // Add inputs as negative operations
    for (vin_idx, input) in tx.inputs.iter().enumerate() {
        operations.push(Operation {
            operation_identifier: OperationIdentifier {
                index: op_index,
                network_index: Some(vin_idx as i64),
            },
            related_operations: None,
            r#type: "TRANSFER".to_string(),
            status: Some("SUCCESS".to_string()),
            account: Some(AccountIdentifier {
                address: format!("ATMN_{}", &input.prev_tx_hash.to_string()[..16]), // Placeholder
                sub_account: None,
                metadata: None,
            }),
            amount: Some(Amount::new(-1000000, Currency::atmn())), // Placeholder amount
            coin_change: Some(CoinChange {
                coin_identifier: CoinIdentifier {
                    identifier: format!("{}:{}", input.prev_tx_hash.to_string(), input.prev_tx_index),
                },
                coin_action: "coin_spent".to_string(),
            }),
            metadata: None,
        });
        op_index += 1;
    }

    // Add outputs as positive operations
    for (vout_idx, output) in tx.outputs.iter().enumerate() {
        let address_bytes = if output.script_pubkey.len() >= 8 {
            hex::encode(&output.script_pubkey[..8])
        } else {
            hex::encode(&output.script_pubkey)
        };
        
        operations.push(Operation {
            operation_identifier: OperationIdentifier {
                index: op_index,
                network_index: Some(vout_idx as i64),
            },
            related_operations: None,
            r#type: "TRANSFER".to_string(),
            status: Some("SUCCESS".to_string()),
            account: Some(AccountIdentifier {
                address: format!("ATMN_{}", address_bytes), // Placeholder
                sub_account: None,
                metadata: None,
            }),
            amount: Some(Amount::new(output.amount as i64, Currency::atmn())),
            coin_change: Some(CoinChange {
                coin_identifier: CoinIdentifier {
                    identifier: format!("{}:{}", tx_hash, vout_idx),
                },
                coin_action: "coin_created".to_string(),
            }),
            metadata: None,
        });
        op_index += 1;
    }

    Transaction {
        transaction_identifier: TransactionIdentifier { hash: tx_hash },
        operations,
        related_transactions: None,
        metadata: Some(serde_json::json!({
            "version": tx.version,
            "locktime": tx.locktime,
        })),
    }
}

/// Get network identifier for Antimony mainnet
pub fn get_mainnet_identifier() -> NetworkIdentifier {
    NetworkIdentifier {
        blockchain: "Antimony".to_string(),
        network: "mainnet".to_string(),
        sub_network_identifier: None,
    }
}

/// Check if a network identifier matches mainnet
pub fn is_mainnet(network: &NetworkIdentifier) -> bool {
    network.blockchain == "Antimony" && network.network == "mainnet"
}

#[cfg(test)]
mod tests {
    use super::*;
    use atmn_core::block::BlockHeader;

    #[test]
    fn test_network_identifier() {
        let mainnet = get_mainnet_identifier();
        assert_eq!(mainnet.blockchain, "Antimony");
        assert_eq!(mainnet.network, "mainnet");
        assert!(is_mainnet(&mainnet));
    }

    #[test]
    fn test_block_conversion() {
        let block = Block {
            header: BlockHeader {
                version: 1,
                prev_block_hash: BlockHash::from_bytes([0; 32]),
                merkle_root: BlockHash::from_bytes([1; 32]),
                timestamp: 1701657600,
                bits: 0x1d00ffff,
                nonce: 42,
            },
            transactions: vec![],
            height: 123,
        };

        let rosetta_block = block_to_rosetta(&block, 123);
        assert_eq!(rosetta_block.block_identifier.index, 123);
        assert_eq!(rosetta_block.timestamp, 1701657600000); // Milliseconds
        assert_eq!(rosetta_block.transactions.len(), 0);
    }

    #[test]
    fn test_currency() {
        let currency = Currency::atmn();
        assert_eq!(currency.symbol, "ATMN");
        assert_eq!(currency.decimals, 8);
    }
}
