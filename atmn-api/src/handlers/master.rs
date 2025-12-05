use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;
use crate::models::{MasterTransferRequest, ErrorResponse};
use crate::db;

pub async fn get_master_info(
    pool: web::Data<SqlitePool>,
) -> HttpResponse {
    match db::get_master_wallet(pool.get_ref()).await {
        Ok(master) => HttpResponse::Ok().json(master),
        Err(e) => {
            log::error!("Get master wallet error: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "INTERNAL_ERROR".to_string(),
                message: format!("Failed to get master wallet: {}", e),
            })
        }
    }
}

pub async fn transfer_premine(
    pool: web::Data<SqlitePool>,
    req: web::Json<MasterTransferRequest>,
) -> HttpResponse {
    // Verify master wallet has sufficient balance
    match db::get_master_wallet(pool.get_ref()).await {
        Ok(master) => {
            if master.balance < req.amount {
                return HttpResponse::BadRequest().json(ErrorResponse {
                    error: "INSUFFICIENT_BALANCE".to_string(),
                    message: format!("Master wallet balance insufficient: {} < {}", master.balance, req.amount),
                });
            }
            
            // Transfer from master
            match db::transfer_from_master(
                pool.get_ref(),
                &req.to_address,
                req.amount,
                &req.purpose,
            ).await {
                Ok(transfer_id) => {
                    // Update master wallet balance
                    let _ = db::update_wallet_balance(
                        pool.get_ref(),
                        "ATMN_MASTER",
                        master.balance - req.amount,
                    ).await;
                    
                    HttpResponse::Ok().json(serde_json::json!({
                        "transfer_id": transfer_id,
                        "status": "completed",
                        "amount": req.amount,
                        "to": req.to_address,
                    }))
                }
                Err(e) => {
                    log::error!("Transfer from master error: {}", e);
                    HttpResponse::InternalServerError().json(ErrorResponse {
                        error: "TRANSFER_FAILED".to_string(),
                        message: format!("Failed to transfer from master: {}", e),
                    })
                }
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "INTERNAL_ERROR".to_string(),
            message: format!("Failed to get master wallet: {}", e),
        }),
    }
}

pub async fn get_transfers(
    pool: web::Data<SqlitePool>,
) -> HttpResponse {
    match sqlx::query_as::<_, crate::models::MasterTransfer>(
        "SELECT transfer_id, to_address, amount, purpose, created_at, status FROM master_transfers ORDER BY created_at DESC"
    )
    .fetch_all(pool.get_ref())
    .await {
        Ok(transfers) => HttpResponse::Ok().json(transfers),
        Err(e) => {
            log::error!("Get transfers error: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "INTERNAL_ERROR".to_string(),
                message: format!("Failed to get transfers: {}", e),
            })
        }
    }
}
