// Rosetta API endpoint handlers
use axum::{Json, extract::State};
use crate::types::*;
use crate::error::{ApiError, ApiResult};
use crate::converters::*;
use crate::AppState;

/// Health check endpoint
pub async fn health() -> &'static str {
    "OK"
}

/// /network/list - List available networks
pub async fn network_list(
    Json(_req): Json<NetworkListRequest>,
) -> ApiResult<Json<NetworkListResponse>> {
    Ok(Json(NetworkListResponse {
        network_identifiers: vec![get_mainnet_identifier()],
    }))
}

/// /network/options - Get supported operations
pub async fn network_options(
    Json(req): Json<NetworkOptionsRequest>,
) -> ApiResult<Json<NetworkOptionsResponse>> {
    if !is_mainnet(&req.network_identifier) {
        return Err(ApiError::NetworkNotFound(req.network_identifier.network));
    }

    Ok(Json(NetworkOptionsResponse {
        version: Version {
            rosetta_version: "1.4.13".to_string(),
            node_version: "0.1.0".to_string(),
            middleware_version: None,
            metadata: Some(serde_json::json!({
                "implementation": "atmn-rosetta",
                "consensus": "SHA-256d Proof of Work"
            })),
        },
        allow: Allow {
            operation_statuses: vec![
                OperationStatus {
                    status: "SUCCESS".to_string(),
                    successful: true,
                },
                OperationStatus {
                    status: "FAILED".to_string(),
                    successful: false,
                },
            ],
            operation_types: vec![
                "TRANSFER".to_string(),
                "MINT".to_string(),
                "FEE".to_string(),
            ],
            errors: vec![
                Error {
                    code: 1,
                    message: "Network not found".to_string(),
                    retriable: false,
                    details: None,
                },
                Error {
                    code: 2,
                    message: "Block not found".to_string(),
                    retriable: true,
                    details: None,
                },
                Error {
                    code: 3,
                    message: "Transaction not found".to_string(),
                    retriable: true,
                    details: None,
                },
            ],
            historical_balance_lookup: true,
            timestamp_start_index: Some(0),
            call_methods: None,
            balance_exemptions: vec![],
            mempool_coins: false,
        },
    }))
}

/// /network/status - Get current network status
pub async fn network_status(
    State(state): State<AppState>,
    Json(req): Json<NetworkStatusRequest>,
) -> ApiResult<Json<NetworkStatusResponse>> {
    if !is_mainnet(&req.network_identifier) {
        return Err(ApiError::NetworkNotFound(req.network_identifier.network));
    }

    // Get actual blockchain state from storage
    let best_height = state.storage.get_best_height()
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .unwrap_or(0);
    
    // Get current block
    let current_block = state.storage.get_block(best_height)
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    
    let (current_height, current_hash, current_timestamp) = if let Some(block) = current_block {
        let hash = block.hash();
        let timestamp = block.header.timestamp as i64 * 1000; // Convert to milliseconds
        (best_height, hex::encode(hash.as_bytes()), timestamp)
    } else {
        // No blocks yet, return genesis placeholder
        (0, format!("{:064x}", 0), 1701657600000)
    };

    Ok(Json(NetworkStatusResponse {
        current_block_identifier: BlockIdentifier::new(current_height, current_hash.clone()),
        current_block_timestamp: current_timestamp,
        genesis_block_identifier: BlockIdentifier::new(0, current_hash.clone()),
        oldest_block_identifier: Some(BlockIdentifier::new(0, current_hash.clone())),
        sync_status: Some(SyncStatus {
            current_index: Some(current_height as i64),
            target_index: Some(current_height as i64),
            stage: Some("synced".to_string()),
            synced: Some(true),
        }),
        peers: vec![],
    }))
}

/// /block - Get block by height or hash
pub async fn block(
    State(state): State<AppState>,
    Json(req): Json<BlockRequest>,
) -> ApiResult<Json<BlockResponse>> {
    if !is_mainnet(&req.network_identifier) {
        return Err(ApiError::NetworkNotFound(req.network_identifier.network));
    }

    // Get block from storage by height or hash
    let block_opt = if let Some(height) = req.block_identifier.index {
        state.storage.get_block(height as u64)
            .map_err(|e| ApiError::Internal(e.to_string()))?
    } else if let Some(hash_str) = &req.block_identifier.hash {
        // Decode hex hash
        let hash_bytes = hex::decode(hash_str)
            .map_err(|_| ApiError::InvalidBlockIdentifier)?;
        if hash_bytes.len() != 32 {
            return Err(ApiError::InvalidBlockIdentifier);
        }
        let mut hash_array = [0u8; 32];
        hash_array.copy_from_slice(&hash_bytes);
        let block_hash = atmn_core::types::BlockHash::from_bytes(hash_array);
        
        state.storage.get_block_by_hash(&block_hash)
            .map_err(|e| ApiError::Internal(e.to_string()))?
    } else {
        return Err(ApiError::InvalidBlockIdentifier);
    };

    let block = block_opt.ok_or_else(|| 
        ApiError::BlockNotFound(format!("{:?}", req.block_identifier))
    )?;
    let height = block.height;
    let rosetta_block = block_to_rosetta(&block, height);
    
    Ok(Json(BlockResponse {
        block: Some(rosetta_block),
        other_transactions: None,
    }))
}

/// /block/transaction - Get specific transaction in block
pub async fn block_transaction(
    Json(req): Json<BlockTransactionRequest>,
) -> ApiResult<Json<BlockTransactionResponse>> {
    if !is_mainnet(&req.network_identifier) {
        return Err(ApiError::NetworkNotFound(req.network_identifier.network));
    }

    // TODO: Query atmn-core for transaction data
    Err(ApiError::NotImplemented("block/transaction".to_string()))
}

/// /mempool - Get pending transactions
pub async fn mempool(
    Json(req): Json<NetworkStatusRequest>,
) -> ApiResult<Json<Vec<TransactionIdentifier>>> {
    if !is_mainnet(&req.network_identifier) {
        return Err(ApiError::NetworkNotFound(req.network_identifier.network));
    }

    // TODO: Query mempool from atmn-core
    Ok(Json(vec![]))
}

/// /mempool/transaction - Get pending transaction details
pub async fn mempool_transaction(
    Json(req): Json<BlockTransactionRequest>,
) -> ApiResult<Json<BlockTransactionResponse>> {
    if !is_mainnet(&req.network_identifier) {
        return Err(ApiError::NetworkNotFound(req.network_identifier.network));
    }

    // TODO: Query mempool for transaction
    Err(ApiError::NotImplemented("mempool/transaction".to_string()))
}

/// /account/balance - Get account balance
pub async fn account_balance(
    State(state): State<AppState>,
    Json(req): Json<AccountBalanceRequest>,
) -> ApiResult<Json<AccountBalanceResponse>> {
    if !is_mainnet(&req.network_identifier) {
        return Err(ApiError::NetworkNotFound(req.network_identifier.network));
    }

    // Get address from account identifier
    let address = &req.account_identifier.address;

    // Get best block for response
    let best_height = state.storage.get_best_height()
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .unwrap_or(0);
    
    let current_block = state.storage.get_block(best_height)
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    
    let (current_height, current_hash) = if let Some(block) = current_block {
        let hash = block.hash();
        (best_height, hex::encode(hash.as_bytes()))
    } else {
        (0, format!("{:064x}", 0))
    };

    // Query UTXO set for this address
    let utxos = state.storage.get_utxos_for_address(address)
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    // Calculate total balance in satoshis
    let total_satoshis: i64 = utxos.iter().map(|u| u.amount as i64).sum();

    // Create balance response
    let balances = vec![Amount {
        value: total_satoshis.to_string(),
        currency: Currency::atmn(),
        metadata: None,
    }];

    // Optionally include coins
    let coins = if !utxos.is_empty() {
        Some(utxos.iter().enumerate().map(|(idx, utxo)| {
            Coin {
                coin_identifier: CoinIdentifier {
                    identifier: format!("{}:{}", hex::encode(utxo.tx_hash.as_bytes()), utxo.output_index),
                },
                amount: Amount {
                    value: (utxo.amount as i64).to_string(),
                    currency: Currency::atmn(),
                    metadata: None,
                },
            }
        }).collect())
    } else {
        None
    };

    Ok(Json(AccountBalanceResponse {
        block_identifier: BlockIdentifier::new(current_height, current_hash),
        balances,
        coins,
        metadata: None,
    }))
}

/// /account/coins - Get spendable coins (UTXOs)
pub async fn account_coins(
    State(state): State<AppState>,
    Json(req): Json<AccountCoinsRequest>,
) -> ApiResult<Json<AccountCoinsResponse>> {
    if !is_mainnet(&req.network_identifier) {
        return Err(ApiError::NetworkNotFound(req.network_identifier.network));
    }

    // Get address from account identifier
    let address = &req.account_identifier.address;

    // Get best block for response
    let best_height = state.storage.get_best_height()
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .unwrap_or(0);
    
    let current_block = state.storage.get_block(best_height)
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    
    let (current_height, current_hash) = if let Some(block) = current_block {
        let hash = block.hash();
        (best_height, hex::encode(hash.as_bytes()))
    } else {
        (0, format!("{:064x}", 0))
    };

    // Query UTXO set for this address
    let utxos = state.storage.get_utxos_for_address(address)
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    // Convert UTXOs to Rosetta Coins
    let coins = utxos.iter().map(|utxo| {
        Coin {
            coin_identifier: CoinIdentifier {
                identifier: format!("{}:{}", hex::encode(utxo.tx_hash.as_bytes()), utxo.output_index),
            },
            amount: Amount {
                value: (utxo.amount as i64).to_string(),
                currency: Currency::atmn(),
                metadata: None,
            },
        }
    }).collect();

    Ok(Json(AccountCoinsResponse {
        block_identifier: BlockIdentifier::new(current_height, current_hash),
        coins,
        metadata: None,
    }))
}

/// /construction/preprocess - Initial validation
pub async fn construction_preprocess(
    Json(_req): Json<serde_json::Value>,
) -> ApiResult<Json<serde_json::Value>> {
    Err(ApiError::NotImplemented("construction/preprocess".to_string()))
}

/// /construction/metadata - Get dynamic metadata
pub async fn construction_metadata(
    Json(_req): Json<serde_json::Value>,
) -> ApiResult<Json<serde_json::Value>> {
    Err(ApiError::NotImplemented("construction/metadata".to_string()))
}

/// /construction/payloads - Create payloads to sign
pub async fn construction_payloads(
    Json(_req): Json<serde_json::Value>,
) -> ApiResult<Json<serde_json::Value>> {
    Err(ApiError::NotImplemented("construction/payloads".to_string()))
}

/// /construction/parse - Parse transaction
pub async fn construction_parse(
    Json(_req): Json<serde_json::Value>,
) -> ApiResult<Json<serde_json::Value>> {
    Err(ApiError::NotImplemented("construction/parse".to_string()))
}

/// /construction/combine - Combine unsigned + signatures
pub async fn construction_combine(
    Json(_req): Json<serde_json::Value>,
) -> ApiResult<Json<serde_json::Value>> {
    Err(ApiError::NotImplemented("construction/combine".to_string()))
}

/// /construction/hash - Get transaction hash
pub async fn construction_hash(
    Json(_req): Json<serde_json::Value>,
) -> ApiResult<Json<serde_json::Value>> {
    Err(ApiError::NotImplemented("construction/hash".to_string()))
}

/// /construction/submit - Broadcast transaction
pub async fn construction_submit(
    Json(_req): Json<serde_json::Value>,
) -> ApiResult<Json<serde_json::Value>> {
    Err(ApiError::NotImplemented("construction/submit".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_network_list() {
        let req = NetworkListRequest { metadata: None };
        let result = network_list(Json(req)).await.unwrap();
        assert_eq!(result.0.network_identifiers.len(), 1);
        assert_eq!(result.0.network_identifiers[0].blockchain, "Antimony");
    }

    #[tokio::test]
    async fn test_network_options() {
        let req = NetworkOptionsRequest {
            network_identifier: get_mainnet_identifier(),
            metadata: None,
        };
        let result = network_options(Json(req)).await.unwrap();
        assert_eq!(result.0.version.rosetta_version, "1.4.13");
        assert!(result.0.allow.operation_types.contains(&"TRANSFER".to_string()));
    }

    #[tokio::test]
    async fn test_health() {
        let result = health().await;
        assert_eq!(result, "OK");
    }
}
