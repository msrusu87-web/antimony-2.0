// Blockchain query handlers
// REST API endpoints for querying blockchain data

use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;
use serde::{Deserialize, Serialize};
use crate::models::ErrorResponse;
use crate::db;

#[derive(Debug, Deserialize)]
pub struct BlockRangeQuery {
    pub start: Option<u64>,
    pub end: Option<u64>,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct BlockInfo {
    pub hash: String,
    pub height: u64,
    pub timestamp: i64,
    pub difficulty: i64,
    pub nonce: i64,
    pub prev_hash: Option<String>,
    pub merkle_root: Option<String>,
    pub transaction_count: usize,
}

#[derive(Debug, Serialize)]
pub struct AddressBalance {
    pub address: String,
    pub balance: f64,
    pub transaction_count: usize,
    pub last_activity: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct TransactionInfo {
    pub tx_hash: String,
    pub block_height: Option<u64>,
    pub timestamp: i64,
    pub from_address: Option<String>,
    pub to_address: String,
    pub amount: f64,
    pub fee: f64,
    pub status: String,
}

/// GET /api/blocks/latest
/// Get the latest blocks
pub async fn get_latest_blocks(
    pool: web::Data<SqlitePool>,
    query: web::Query<BlockRangeQuery>,
) -> HttpResponse {
    let limit = query.limit.unwrap_or(10).min(100); // Max 100 blocks
    
    match sqlx::query_as::<_, (String, i64, i64, i64, i64, Option<String>, Option<String>)>(
        "SELECT hash, height, timestamp, difficulty, nonce, prev_hash, merkle_root 
         FROM blocks 
         ORDER BY height DESC 
         LIMIT ?"
    )
    .bind(limit as i64)
    .fetch_all(pool.get_ref())
    .await {
        Ok(blocks) => {
            let block_infos: Vec<BlockInfo> = blocks.iter().map(|b| {
                BlockInfo {
                    hash: b.0.clone(),
                    height: b.1 as u64,
                    timestamp: b.2,
                    difficulty: b.3,
                    nonce: b.4,
                    prev_hash: b.5.clone(),
                    merkle_root: b.6.clone(),
                    transaction_count: 0, // TODO: Count transactions
                }
            }).collect();
            
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "count": block_infos.len(),
                "blocks": block_infos
            }))
        }
        Err(e) => {
            log::error!("Failed to query latest blocks: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "DATABASE_ERROR".to_string(),
                message: "Failed to retrieve latest blocks".to_string(),
            })
        }
    }
}

/// GET /api/blocks/range?start=0&end=100
/// Get blocks in a specific range
pub async fn get_block_range(
    pool: web::Data<SqlitePool>,
    query: web::Query<BlockRangeQuery>,
) -> HttpResponse {
    let start = query.start.unwrap_or(0);
    let end = query.end.unwrap_or(start + 100);
    let limit = (end - start).min(100); // Max 100 blocks per request
    
    match sqlx::query_as::<_, (String, i64, i64, i64, i64, Option<String>, Option<String>)>(
        "SELECT hash, height, timestamp, difficulty, nonce, prev_hash, merkle_root 
         FROM blocks 
         WHERE height >= ? AND height <= ?
         ORDER BY height ASC 
         LIMIT ?"
    )
    .bind(start as i64)
    .bind(end as i64)
    .bind(limit as i64)
    .fetch_all(pool.get_ref())
    .await {
        Ok(blocks) => {
            let block_infos: Vec<BlockInfo> = blocks.iter().map(|b| {
                BlockInfo {
                    hash: b.0.clone(),
                    height: b.1 as u64,
                    timestamp: b.2,
                    difficulty: b.3,
                    nonce: b.4,
                    prev_hash: b.5.clone(),
                    merkle_root: b.6.clone(),
                    transaction_count: 0, // TODO: Count transactions
                }
            }).collect();
            
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "start": start,
                "end": end,
                "count": block_infos.len(),
                "blocks": block_infos
            }))
        }
        Err(e) => {
            log::error!("Failed to query block range: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "DATABASE_ERROR".to_string(),
                message: "Failed to retrieve block range".to_string(),
            })
        }
    }
}

/// GET /api/blocks/{height}
/// Get a specific block by height
pub async fn get_block_by_height(
    pool: web::Data<SqlitePool>,
    height: web::Path<u64>,
) -> HttpResponse {
    match db::get_block_by_height(pool.get_ref(), *height).await {
        Ok(block) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "block": block
        })),
        Err(e) => {
            log::warn!("Block not found at height {}: {}", height, e);
            HttpResponse::NotFound().json(ErrorResponse {
                error: "BLOCK_NOT_FOUND".to_string(),
                message: format!("Block at height {} not found", height),
            })
        }
    }
}

/// GET /api/address/{address}/balance
/// Get address balance
pub async fn get_address_balance(
    pool: web::Data<SqlitePool>,
    address: web::Path<String>,
) -> HttpResponse {
    // Query wallet balance
    let balance: Option<f64> = sqlx::query_scalar(
        "SELECT balance FROM wallets WHERE wallet_address = ?"
    )
    .bind(address.as_str())
    .fetch_optional(pool.get_ref())
    .await
    .unwrap_or(None);
    
    // Count transactions
    let tx_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM transactions 
         WHERE from_address = ? OR to_address = ?"
    )
    .bind(address.as_str())
    .bind(address.as_str())
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or(0);
    
    // Get last activity timestamp
    let last_activity: Option<i64> = sqlx::query_scalar(
        "SELECT MAX(timestamp) FROM transactions 
         WHERE from_address = ? OR to_address = ?"
    )
    .bind(address.as_str())
    .bind(address.as_str())
    .fetch_optional(pool.get_ref())
    .await
    .unwrap_or(None)
    .flatten();
    
    HttpResponse::Ok().json(AddressBalance {
        address: address.to_string(),
        balance: balance.unwrap_or(0.0),
        transaction_count: tx_count as usize,
        last_activity,
    })
}

/// GET /api/address/{address}/transactions
/// Get all transactions for an address
pub async fn get_address_transactions(
    pool: web::Data<SqlitePool>,
    address: web::Path<String>,
    query: web::Query<BlockRangeQuery>,
) -> HttpResponse {
    let limit = query.limit.unwrap_or(50).min(200); // Max 200 transactions
    
    match sqlx::query_as::<_, (String, Option<i64>, i64, Option<String>, String, f64, String)>(
        "SELECT tx_hash, NULL as block_height, timestamp, from_address, to_address, amount, status 
         FROM transactions 
         WHERE from_address = ? OR to_address = ?
         ORDER BY timestamp DESC 
         LIMIT ?"
    )
    .bind(address.as_str())
    .bind(address.as_str())
    .bind(limit as i64)
    .fetch_all(pool.get_ref())
    .await {
        Ok(txs) => {
            let transactions: Vec<TransactionInfo> = txs.iter().map(|tx| {
                TransactionInfo {
                    tx_hash: tx.0.clone(),
                    block_height: tx.1.map(|h| h as u64),
                    timestamp: tx.2,
                    from_address: tx.3.clone(),
                    to_address: tx.4.clone(),
                    amount: tx.5,
                    fee: 0.0001, // TODO: Calculate actual fee
                    status: tx.6.clone(),
                }
            }).collect();
            
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "address": address.to_string(),
                "count": transactions.len(),
                "transactions": transactions
            }))
        }
        Err(e) => {
            log::error!("Failed to query address transactions: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "DATABASE_ERROR".to_string(),
                message: "Failed to retrieve transactions".to_string(),
            })
        }
    }
}

/// GET /api/blockchain/stats
/// Get overall blockchain statistics
pub async fn get_blockchain_stats(
    pool: web::Data<SqlitePool>,
) -> HttpResponse {
    // Get current height
    let height = match db::get_current_height(pool.get_ref()).await {
        Ok(h) => h,
        Err(_) => 0
    };
    
    // Get total blocks
    let total_blocks: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM blocks"
    )
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or(0);
    
    // Get total transactions
    let total_txs: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM transactions"
    )
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or(0);
    
    // Get total addresses (wallets)
    let total_addresses: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM wallets"
    )
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or(0);
    
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "current_height": height,
        "total_blocks": total_blocks,
        "total_transactions": total_txs,
        "total_addresses": total_addresses,
        "network": "testnet"
    }))
}
