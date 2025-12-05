use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;
use serde::{Deserialize, Serialize};
use crate::models::ErrorResponse;

const POOL_FEE_PERCENT: f64 = 2.0; // 2% pool fee

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeCollectionRequest {
    pub block_reward: f64,
    pub block_height: i64,
    pub miner_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeCollectionResponse {
    pub success: bool,
    pub fee_amount: f64,
    pub miner_payout: f64,
    pub master_wallet_balance: f64,
    pub transaction_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MasterWalletStats {
    pub balance: f64,
    pub total_fees_collected: f64,
    pub total_payouts: f64,
    pub last_updated: String,
}

/// Collect pool fees from block rewards and credit master wallet
pub async fn collect_fee(
    pool: web::Data<SqlitePool>,
    req: web::Json<FeeCollectionRequest>,
) -> HttpResponse {
    let fee_amount = req.block_reward * (POOL_FEE_PERCENT / 100.0);
    let miner_payout = req.block_reward - fee_amount;

    // Start transaction
    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(e) => {
            log::error!("Transaction start error: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "TRANSACTION_FAILED".to_string(),
                message: "Failed to start transaction".to_string(),
            });
        }
    };

    // Update master wallet balance
    match sqlx::query(
        "UPDATE master_wallet SET balance = balance + ?, total_fees_collected = total_fees_collected + ?, last_updated = datetime('now') WHERE id = 1"
    )
    .bind(fee_amount)
    .bind(fee_amount)
    .execute(&mut *tx)
    .await {
        Ok(_) => {},
        Err(e) => {
            log::error!("Master wallet update error: {}", e);
            let _ = tx.rollback().await;
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "UPDATE_FAILED".to_string(),
                message: format!("Failed to update master wallet: {}", e),
            });
        }
    }

    // Record fee transaction
    let transaction_id = match sqlx::query(
        "INSERT INTO fee_transactions (block_height, miner_address, block_reward, fee_amount, miner_payout, collected_at)
         VALUES (?, ?, ?, ?, ?, datetime('now'))"
    )
    .bind(req.block_height)
    .bind(&req.miner_address)
    .bind(req.block_reward)
    .bind(fee_amount)
    .bind(miner_payout)
    .execute(&mut *tx)
    .await {
        Ok(result) => result.last_insert_rowid(),
        Err(e) => {
            log::error!("Fee transaction record error: {}", e);
            let _ = tx.rollback().await;
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "RECORD_FAILED".to_string(),
                message: format!("Failed to record fee transaction: {}", e),
            });
        }
    };

    // Commit transaction
    if let Err(e) = tx.commit().await {
        log::error!("Transaction commit error: {}", e);
        return HttpResponse::InternalServerError().json(ErrorResponse {
            error: "COMMIT_FAILED".to_string(),
            message: "Failed to commit transaction".to_string(),
        });
    }

    // Get updated master wallet balance
    let master_balance: f64 = sqlx::query_scalar(
        "SELECT balance FROM master_wallet WHERE id = 1"
    )
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or(0.0);

    HttpResponse::Ok().json(FeeCollectionResponse {
        success: true,
        fee_amount,
        miner_payout,
        master_wallet_balance: master_balance,
        transaction_id: Some(transaction_id),
    })
}

/// Get master wallet statistics
pub async fn get_master_wallet_stats(
    pool: web::Data<SqlitePool>,
) -> HttpResponse {
    match sqlx::query_as::<_, (f64, f64, f64, String)>(
        "SELECT balance, total_fees_collected, total_payouts, last_updated FROM master_wallet WHERE id = 1"
    )
    .fetch_one(pool.get_ref())
    .await {
        Ok((balance, total_fees, total_payouts, last_updated)) => {
            HttpResponse::Ok().json(MasterWalletStats {
                balance,
                total_fees_collected: total_fees,
                total_payouts,
                last_updated,
            })
        }
        Err(e) => {
            log::error!("Get master wallet stats error: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "QUERY_FAILED".to_string(),
                message: format!("Failed to get master wallet stats: {}", e),
            })
        }
    }
}

/// Get fee transaction history
pub async fn get_fee_history(
    pool: web::Data<SqlitePool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    let limit: i32 = query.get("limit").and_then(|s| s.parse().ok()).unwrap_or(100);
    let offset: i32 = query.get("offset").and_then(|s| s.parse().ok()).unwrap_or(0);

    match sqlx::query_as::<_, (i64, i64, String, f64, f64, f64, String)>(
        "SELECT id, block_height, miner_address, block_reward, fee_amount, miner_payout, collected_at 
         FROM fee_transactions 
         ORDER BY collected_at DESC 
         LIMIT ? OFFSET ?"
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await {
        Ok(rows) => {
            let transactions: Vec<serde_json::Value> = rows.into_iter().map(|(id, height, miner, reward, fee, payout, time)| {
                serde_json::json!({
                    "id": id,
                    "block_height": height,
                    "miner_address": miner,
                    "block_reward": reward,
                    "fee_amount": fee,
                    "miner_payout": payout,
                    "collected_at": time
                })
            }).collect();
            HttpResponse::Ok().json(transactions)
        }
        Err(e) => {
            log::error!("Get fee history error: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "QUERY_FAILED".to_string(),
                message: format!("Failed to get fee history: {}", e),
            })
        }
    }
}
