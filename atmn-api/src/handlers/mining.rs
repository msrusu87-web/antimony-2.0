use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;
use serde::{Deserialize, Serialize};
use crate::models::ErrorResponse;
use crate::mining_manager::MiningManager;
use crate::db;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct StartMiningRequest {
    pub miner_address: String,
    pub threads: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct SubmitBlockRequest {
    pub block_data: String,
    pub nonce: u32,
    pub hash: String,
}

#[derive(Debug, Serialize)]
pub struct MiningStatusResponse {
    pub is_mining: bool,
    pub miner_address: Option<String>,
    pub blocks_found: u64,
    pub hash_rate: f64,
    pub uptime_seconds: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct BlockTemplateResponse {
    pub version: u32,
    pub prev_block_hash: String,
    pub merkle_root: String,
    pub timestamp: u32,
    pub bits: u32,
    pub height: u64,
    pub transactions: Vec<String>,
}

/// Start mining
pub async fn start_mining(
    pool: web::Data<SqlitePool>,
    mining_manager: web::Data<Arc<MiningManager>>,
    req: web::Json<StartMiningRequest>,
) -> HttpResponse {
    // Validate miner address
    if !req.miner_address.starts_with("ATMN_") {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "INVALID_ADDRESS".to_string(),
            message: "Miner address must start with ATMN_".to_string(),
        });
    }

    // Get latest block
    let result = sqlx::query!(
        "SELECT height, hash FROM blocks ORDER BY height DESC LIMIT 1"
    )
    .fetch_optional(pool.get_ref())
    .await;

    let (height, prev_hash_str) = match result {
        Ok(Some(block)) => (block.height as u64, block.hash),
        Ok(None) => (0, "0000000000000000000000000000000000000000000000000000000000000000".to_string()),
        Err(e) => {
            log::error!("Failed to get latest block: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "DATABASE_ERROR".to_string(),
                message: "Failed to get latest block".to_string(),
            });
        }
    };

    // Parse prev block hash
    let prev_hash_bytes = hex::decode(&prev_hash_str).unwrap_or_else(|_| vec![0u8; 32]);
    let mut prev_hash = [0u8; 32];
    prev_hash.copy_from_slice(&prev_hash_bytes[..32]);

    // Start mining
    let threads = req.threads.unwrap_or(1);
    let difficulty_bits = 0x1d00ffff; // Default difficulty

    match mining_manager.start_mining(
        req.miner_address.clone(),
        threads,
        difficulty_bits,
        prev_hash,
        height + 1,
    ) {
        Ok(_) => {
            log::info!("Mining started for address: {}", req.miner_address);
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "message": "Mining started",
                "miner_address": req.miner_address,
                "threads": threads,
                "height": height + 1
            }))
        }
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse {
            error: "START_FAILED".to_string(),
            message: e,
        }),
    }
}

/// Stop mining
pub async fn stop_mining(
    mining_manager: web::Data<Arc<MiningManager>>,
) -> HttpResponse {
    match mining_manager.stop_mining() {
        Ok((blocks_found, miner_address)) => {
            log::info!("Mining stopped. Blocks found: {}", blocks_found);
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "message": "Mining stopped",
                "blocks_found": blocks_found,
                "miner_address": miner_address
            }))
        }
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse {
            error: "NOT_MINING".to_string(),
            message: e,
        }),
    }
}

/// Get mining status
pub async fn get_mining_status(
    mining_manager: web::Data<Arc<MiningManager>>,
) -> HttpResponse {
    let state = mining_manager.get_state();

    let uptime_seconds = if let Some(start_time) = state.start_time {
        Some(chrono::Utc::now().timestamp() - start_time)
    } else {
        None
    };

    let response = MiningStatusResponse {
        is_mining: state.is_mining,
        miner_address: state.miner_address,
        blocks_found: state.blocks_found,
        hash_rate: state.hash_rate,
        uptime_seconds,
    };

    HttpResponse::Ok().json(response)
}

/// Get block template for mining
pub async fn get_block_template(
    pool: web::Data<SqlitePool>,
    mining_manager: web::Data<Arc<MiningManager>>,
) -> HttpResponse {
    // Get latest block
    let result = sqlx::query!(
        "SELECT height, hash FROM blocks ORDER BY height DESC LIMIT 1"
    )
    .fetch_optional(pool.get_ref())
    .await;

    let (prev_hash, height) = match result {
        Ok(Some(block)) => (block.hash, block.height as u64),
        Ok(None) => ("0000000000000000000000000000000000000000000000000000000000000000".to_string(), 0),
        Err(e) => {
            log::error!("Failed to get latest block: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "DATABASE_ERROR".to_string(),
                message: "Failed to get latest block".to_string(),
            });
        }
    };

    // Get pending transactions
    let transactions = mining_manager.get_pending_transactions(100);
    let tx_ids: Vec<String> = transactions
        .iter()
        .map(|tx| {
            // Generate transaction ID (hash of serialized tx)
            use sha2::{Sha256, Digest};
            let serialized = bincode::serialize(tx).unwrap_or_default();
            let hash = Sha256::digest(&serialized);
            hex::encode(hash)
        })
        .collect();

    // Calculate merkle root
    let merkle_root = if transactions.is_empty() {
        "0000000000000000000000000000000000000000000000000000000000000000".to_string()
    } else {
        match atmn_core::miner::calculate_merkle_root(&transactions) {
            Ok(root) => hex::encode(root.0),
            Err(_) => "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
        }
    };

    let template = BlockTemplateResponse {
        version: 1,
        prev_block_hash: prev_hash,
        merkle_root,
        timestamp: chrono::Utc::now().timestamp() as u32,
        bits: 0x1d00ffff,
        height: height + 1,
        transactions: tx_ids,
    };

    HttpResponse::Ok().json(template)
}

/// Submit mined block
pub async fn submit_block(
    pool: web::Data<SqlitePool>,
    mining_manager: web::Data<Arc<MiningManager>>,
    req: web::Json<SubmitBlockRequest>,
) -> HttpResponse {
    log::info!("Block submission received. Hash: {}", req.hash);

    // Validate hash format
    if req.hash.len() != 64 {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "INVALID_HASH".to_string(),
            message: "Block hash must be 64 hex characters".to_string(),
        });
    }

    // TODO: Parse block_data to extract transactions
    // For now, we'll implement the validation framework
    
    // Example: Extract transaction hashes from block_data
    // In a real implementation, you would parse the block structure
    let tx_hashes: Vec<String> = vec![]; // Parse from req.block_data
    
    // Validate all transactions before accepting the block
    for tx_hash in &tx_hashes {
        // Check if transaction already exists (replay attack prevention)
        match db::check_transaction_exists(pool.get_ref(), tx_hash).await {
            Ok(exists) => {
                if exists {
                    log::warn!("Block contains duplicate transaction: {}", tx_hash);
                    return HttpResponse::BadRequest().json(ErrorResponse {
                        error: "DUPLICATE_TRANSACTION".to_string(),
                        message: format!("Transaction {} already exists in blockchain", tx_hash),
                    });
                }
            }
            Err(e) => {
                log::error!("Error checking transaction existence: {}", e);
                return HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "DATABASE_ERROR".to_string(),
                    message: "Failed to verify transaction uniqueness".to_string(),
                });
            }
        }
        
        // Verify all inputs reference valid unspent UTXOs (double-spend prevention)
        match db::verify_transaction_inputs(pool.get_ref(), tx_hash).await {
            Ok(valid) => {
                if !valid {
                    log::warn!("Block contains transaction with invalid inputs: {}", tx_hash);
                    return HttpResponse::BadRequest().json(ErrorResponse {
                        error: "INVALID_INPUTS".to_string(),
                        message: format!("Transaction {} references spent or non-existent UTXOs", tx_hash),
                    });
                }
            }
            Err(e) => {
                log::error!("Error verifying transaction inputs: {}", e);
                return HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "VALIDATION_ERROR".to_string(),
                    message: "Failed to verify transaction inputs".to_string(),
                });
            }
        }
    }
    
    // If we get here, all validation passed
    // Process the block: mark UTXOs as spent and create new ones
    let block_height = 0; // TODO: Get actual block height from database
    
    match db::process_block_transactions(pool.get_ref(), block_height, &tx_hashes).await {
        Ok(_) => {
            log::info!("Block accepted and processed! Hash: {}", req.hash);
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "message": "Block accepted",
                "hash": req.hash,
                "height": block_height,
                "reward": 50.0
            }))
        }
        Err(e) => {
            log::error!("Error processing block transactions: {}", e);
            HttpResponse::BadRequest().json(ErrorResponse {
                error: "PROCESSING_ERROR".to_string(),
                message: format!("Failed to process block: {}", e),
            })
        }
    }
}



/// Get mempool statistics
pub async fn get_mempool_stats(
    mining_manager: web::Data<Arc<MiningManager>>,
) -> HttpResponse {
    let (tx_count, total_size, max_size, min_fee) = mining_manager.get_mempool_stats();

    HttpResponse::Ok().json(serde_json::json!({
        "transaction_count": tx_count,
        "total_size_bytes": total_size,
        "max_size": max_size,
        "min_fee_per_byte": min_fee
    }))
}

// Legacy pool endpoints (keep for backward compatibility)

pub async fn register_worker(
    pool: web::Data<SqlitePool>,
    req: web::Json<serde_json::Value>,
) -> HttpResponse {
    if let (Some(worker_id), Some(miner_address)) = (
        req.get("worker_id").and_then(|v| v.as_str()),
        req.get("miner_address").and_then(|v| v.as_str()),
    ) {
        match db::register_mining_worker(pool.get_ref(), worker_id, miner_address).await {
            Ok(worker) => HttpResponse::Ok().json(worker),
            Err(e) => {
                log::error!("Register worker error: {}", e);
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "REGISTRATION_FAILED".to_string(),
                    message: format!("Failed to register worker: {}", e),
                })
            }
        }
    } else {
        HttpResponse::BadRequest().json(ErrorResponse {
            error: "MISSING_FIELDS".to_string(),
            message: "worker_id and miner_address required".to_string(),
        })
    }
}

pub async fn get_workers(
    pool: web::Data<SqlitePool>,
) -> HttpResponse {
    match db::get_mining_workers(pool.get_ref()).await {
        Ok(workers) => HttpResponse::Ok().json(workers),
        Err(e) => {
            log::error!("Get workers error: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "INTERNAL_ERROR".to_string(),
                message: format!("Failed to get workers: {}", e),
            })
        }
    }
}

pub async fn get_pool_stats(
    pool: web::Data<SqlitePool>,
) -> HttpResponse {
    match db::get_pool_statistics(pool.get_ref()).await {
        Ok(stats) => HttpResponse::Ok().json(stats),
        Err(e) => {
            log::error!("Get pool stats error: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "INTERNAL_ERROR".to_string(),
                message: format!("Failed to get pool stats: {}", e),
            })
        }
    }
}

pub async fn get_payouts(
    pool: web::Data<SqlitePool>,
    address: web::Path<String>,
) -> HttpResponse {
    match sqlx::query_as::<_, crate::models::MiningPayout>(
        "SELECT id, miner_address, amount, created_at, confirmed FROM mining_payouts WHERE miner_address = ? ORDER BY created_at DESC"
    )
    .bind(address.as_str())
    .fetch_all(pool.get_ref())
    .await {
        Ok(payouts) => HttpResponse::Ok().json(payouts),
        Err(e) => {
            log::error!("Get payouts error: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "INTERNAL_ERROR".to_string(),
                message: format!("Failed to get payouts: {}", e),
            })
        }
    }
}
