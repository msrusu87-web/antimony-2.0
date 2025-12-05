use actix_web::{web, HttpResponse};
use sqlx::{SqlitePool, Row};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use hex;
use rand::Rng;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWalletRequest {
    pub email: String,
    pub wallet_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletResponse {
    pub success: bool,
    pub address: String,
    pub balance: f64,
    pub is_default: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserWalletsResponse {
    pub success: bool,
    pub wallets: Vec<WalletInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletInfo {
    pub address: String,
    pub balance: f64,
    pub is_default: bool,
    pub created_at: String,
}

// Get all wallets for a user
pub async fn get_user_wallets(pool: web::Data<SqlitePool>, email: web::Path<String>) -> HttpResponse {
    // Get user ID from email
    let user_id: Option<i64> = match sqlx::query_scalar("SELECT id FROM users WHERE email = ?")
        .bind(email.as_str())
        .fetch_optional(pool.get_ref())
        .await {
        Ok(Some(id)) => Some(id),
        _ => return HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "message": "User not found"
        }))
    };

    if let Some(uid) = user_id {
        match sqlx::query(
            "SELECT w.wallet_address, w.balance, uw.is_default, w.created_at
             FROM user_wallets uw
             JOIN wallets w ON w.wallet_address = uw.wallet_address
             WHERE uw.user_id = ?
             ORDER BY uw.is_default DESC, w.created_at DESC"
        )
        .bind(uid)
        .fetch_all(pool.get_ref())
        .await {
            Ok(rows) => {
                let wallets: Vec<WalletInfo> = rows.iter().map(|row| {
                    WalletInfo {
                        address: row.get("wallet_address"),
                        balance: row.get("balance"),
                        is_default: row.get("is_default"),
                        created_at: row.get("created_at"),
                    }
                }).collect();

                HttpResponse::Ok().json(UserWalletsResponse {
                    success: true,
                    wallets,
                })
            },
            Err(e) => {
                log::error!("Get wallets error: {}", e);
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "success": false,
                    "message": "Failed to fetch wallets"
                }))
            }
        }
    } else {
        HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "message": "User not found"
        }))
    }
}

// Create a new wallet for user
pub async fn create_new_wallet(pool: web::Data<SqlitePool>, req: web::Json<CreateWalletRequest>) -> HttpResponse {
    // Get user ID
    let user_id: Option<i64> = match sqlx::query_scalar("SELECT id FROM users WHERE email = ?")
        .bind(&req.email)
        .fetch_optional(pool.get_ref())
        .await {
        Ok(Some(id)) => Some(id),
        _ => return HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "message": "User not found"
        }))
    };

    if let Some(uid) = user_id {
        // Generate new wallet address
        let mut rng = rand::thread_rng();
        let random_bytes: Vec<u8> = (0..20).map(|_| rng.gen()).collect();
        let wallet_address = format!("ATMN_{}", hex::encode(&random_bytes));
        
        // Generate private key hash
        let mut hasher = Sha256::new();
        hasher.update(&random_bytes);
        let private_key_hash = hex::encode(hasher.finalize());

        // Insert wallet
        match sqlx::query(
            "INSERT INTO wallets (wallet_address, private_key_hash, balance, is_active) 
             VALUES (?, ?, 0.0, 1)"
        )
        .bind(&wallet_address)
        .bind(&private_key_hash)
        .execute(pool.get_ref())
        .await {
            Ok(_) => {
                // Associate with user
                match sqlx::query(
                    "INSERT INTO user_wallets (user_id, wallet_address, is_default) 
                     VALUES (?, ?, 0)"
                )
                .bind(uid)
                .bind(&wallet_address)
                .execute(pool.get_ref())
                .await {
                    Ok(_) => {
                        HttpResponse::Ok().json(WalletResponse {
                            success: true,
                            address: wallet_address,
                            balance: 0.0,
                            is_default: false,
                        })
                    },
                    Err(e) => {
                        log::error!("Associate wallet error: {}", e);
                        HttpResponse::InternalServerError().json(serde_json::json!({
                            "success": false,
                            "message": "Failed to associate wallet"
                        }))
                    }
                }
            },
            Err(e) => {
                log::error!("Create wallet error: {}", e);
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "success": false,
                    "message": "Failed to create wallet"
                }))
            }
        }
    } else {
        HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "message": "User not found"
        }))
    }
}

// Set default wallet
pub async fn set_default_wallet(pool: web::Data<SqlitePool>, path: web::Path<(String, String)>) -> HttpResponse {
    let (email, wallet_address) = path.into_inner();

    // Get user ID
    let user_id: Option<i64> = match sqlx::query_scalar("SELECT id FROM users WHERE email = ?")
        .bind(&email)
        .fetch_optional(pool.get_ref())
        .await {
        Ok(Some(id)) => Some(id),
        _ => return HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "message": "User not found"
        }))
    };

    if let Some(uid) = user_id {
        // Remove default from all wallets
        let _ = sqlx::query("UPDATE user_wallets SET is_default = 0 WHERE user_id = ?")
            .bind(uid)
            .execute(pool.get_ref())
            .await;

        // Set new default
        match sqlx::query(
            "UPDATE user_wallets SET is_default = 1 
             WHERE user_id = ? AND wallet_address = ?"
        )
        .bind(uid)
        .bind(&wallet_address)
        .execute(pool.get_ref())
        .await {
            Ok(_) => {
                HttpResponse::Ok().json(serde_json::json!({
                    "success": true,
                    "message": "Default wallet updated"
                }))
            },
            Err(e) => {
                log::error!("Set default error: {}", e);
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "success": false,
                    "message": "Failed to set default wallet"
                }))
            }
        }
    } else {
        HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "message": "User not found"
        }))
    }
}
