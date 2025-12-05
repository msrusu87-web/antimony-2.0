use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;
use sha2::{Sha256, Digest};
use hex;
use crate::models::{CreateWalletRequest, CreateWalletResponse, ErrorResponse};
use crate::db;

pub async fn create_wallet(
    pool: web::Data<SqlitePool>,
    req: web::Json<CreateWalletRequest>,
) -> HttpResponse {
    // Hash the private key
    let mut hasher = Sha256::new();
    hasher.update(req.private_key.as_bytes());
    let key_hash = hex::encode(hasher.finalize());
    
    // Generate address from hash
    let address = format!("ATMN_{}", &key_hash[..40]);
    
    match db::create_wallet(pool.get_ref(), &address, &key_hash).await {
        Ok(wallet) => HttpResponse::Ok().json(CreateWalletResponse {
            address: wallet.wallet_address,
            private_key_hash: wallet.private_key_hash,
        }),
        Err(e) => {
            log::error!("Create wallet error: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "CREATE_WALLET_FAILED".to_string(),
                message: format!("Failed to create wallet: {}", e),
            })
        }
    }
}

pub async fn get_wallet(
    pool: web::Data<SqlitePool>,
    address: web::Path<String>,
) -> HttpResponse {
    match db::get_wallet(pool.get_ref(), &address).await {
        Ok(wallet) => HttpResponse::Ok().json(wallet),
        Err(e) => {
            log::error!("Get wallet error: {}", e);
            HttpResponse::NotFound().json(ErrorResponse {
                error: "WALLET_NOT_FOUND".to_string(),
                message: format!("Wallet not found: {}", e),
            })
        }
    }
}

pub async fn get_balance(
    pool: web::Data<SqlitePool>,
    address: web::Path<String>,
) -> HttpResponse {
    match db::get_wallet(pool.get_ref(), &address).await {
        Ok(wallet) => HttpResponse::Ok().json(serde_json::json!({
            "address": wallet.wallet_address,
            "balance": wallet.balance,
        })),
        Err(e) => {
            log::error!("Get balance error: {}", e);
            HttpResponse::NotFound().json(ErrorResponse {
                error: "WALLET_NOT_FOUND".to_string(),
                message: format!("Wallet not found: {}", e),
            })
        }
    }
}

pub async fn verify_wallet(
    pool: web::Data<SqlitePool>,
    req: web::Json<serde_json::Value>,
) -> HttpResponse {
    if let (Some(address), Some(private_key)) = (
        req.get("address").and_then(|v| v.as_str()),
        req.get("private_key").and_then(|v| v.as_str()),
    ) {
        let mut hasher = Sha256::new();
        hasher.update(private_key.as_bytes());
        let key_hash = hex::encode(hasher.finalize());
        
        match db::get_wallet(pool.get_ref(), address).await {
            Ok(wallet) => {
                if wallet.private_key_hash == key_hash {
                    HttpResponse::Ok().json(serde_json::json!({
                        "valid": true,
                        "message": "Private key is valid",
                    }))
                } else {
                    HttpResponse::Unauthorized().json(ErrorResponse {
                        error: "INVALID_KEY".to_string(),
                        message: "Private key does not match".to_string(),
                    })
                }
            }
            Err(e) => HttpResponse::NotFound().json(ErrorResponse {
                error: "WALLET_NOT_FOUND".to_string(),
                message: format!("Wallet not found: {}", e),
            }),
        }
    } else {
        HttpResponse::BadRequest().json(ErrorResponse {
            error: "MISSING_FIELDS".to_string(),
            message: "address and private_key required".to_string(),
        })
    }
}
