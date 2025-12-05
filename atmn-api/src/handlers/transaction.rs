use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;
use crate::models::{CreateTransactionRequest, CreateTransactionResponse, ErrorResponse};
use crate::db;

pub async fn create_transaction(
    pool: web::Data<SqlitePool>,
    req: web::Json<CreateTransactionRequest>,
) -> HttpResponse {
    // Check if sender has sufficient balance
    match db::get_wallet(pool.get_ref(), &req.from_address).await {
        Ok(wallet) => {
            if wallet.balance < req.amount {
                return HttpResponse::BadRequest().json(ErrorResponse {
                    error: "INSUFFICIENT_BALANCE".to_string(),
                    message: format!("Insufficient balance: {} < {}", wallet.balance, req.amount),
                });
            }
            
            // Create transaction
            match db::create_transaction(
                pool.get_ref(),
                &req.from_address,
                &req.to_address,
                req.amount,
            ).await {
                Ok(tx) => {
                    // Deduct from sender
                    let _ = db::update_wallet_balance(
                        pool.get_ref(),
                        &req.from_address,
                        wallet.balance - req.amount,
                    ).await;
                    
                    // Add to recipient
                    if let Ok(recipient) = db::get_wallet(pool.get_ref(), &req.to_address).await {
                        let _ = db::update_wallet_balance(
                            pool.get_ref(),
                            &req.to_address,
                            recipient.balance + req.amount,
                        ).await;
                    }
                    
                    HttpResponse::Ok().json(CreateTransactionResponse {
                        tx_hash: tx.tx_hash,
                        status: tx.status,
                    })
                }
                Err(e) => {
                    log::error!("Create transaction error: {}", e);
                    HttpResponse::InternalServerError().json(ErrorResponse {
                        error: "TRANSACTION_FAILED".to_string(),
                        message: format!("Failed to create transaction: {}", e),
                    })
                }
            }
        }
        Err(e) => HttpResponse::NotFound().json(ErrorResponse {
            error: "WALLET_NOT_FOUND".to_string(),
            message: format!("Sender wallet not found: {}", e),
        }),
    }
}

pub async fn get_transactions(
    pool: web::Data<SqlitePool>,
    address: web::Path<String>,
) -> HttpResponse {
    match db::get_wallet_transactions(pool.get_ref(), &address).await {
        Ok(txs) => HttpResponse::Ok().json(txs),
        Err(e) => {
            log::error!("Get transactions error: {}", e);
            HttpResponse::NotFound().json(ErrorResponse {
                error: "NOT_FOUND".to_string(),
                message: format!("No transactions found: {}", e),
            })
        }
    }
}

pub async fn get_transaction(
    pool: web::Data<SqlitePool>,
    tx_hash: web::Path<String>,
) -> HttpResponse {
    match db::get_transaction(pool.get_ref(), &tx_hash).await {
        Ok(tx) => HttpResponse::Ok().json(tx),
        Err(e) => {
            log::error!("Get transaction error: {}", e);
            HttpResponse::NotFound().json(ErrorResponse {
                error: "NOT_FOUND".to_string(),
                message: format!("Transaction not found: {}", e),
            })
        }
    }
}
