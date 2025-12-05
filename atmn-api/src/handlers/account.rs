use actix_web::{web, HttpResponse};
use sqlx::{SqlitePool, Row};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountBalanceResponse {
    pub success: bool,
    pub address: String,
    pub balance: f64,
    pub utxo_count: i64,
    pub last_updated: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UTXO {
    pub tx_hash: String,
    pub output_index: i64,
    pub amount: f64,
    pub block_height: i64,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountCoinsResponse {
    pub success: bool,
    pub address: String,
    pub utxos: Vec<UTXO>,
    pub total_amount: f64,
}

// Get account balance
pub async fn get_account_balance(
    pool: web::Data<SqlitePool>,
    address: web::Path<String>,
) -> HttpResponse {
    let addr = address.into_inner();
    
    // Check if address exists in cache
    match sqlx::query(
        "SELECT balance, utxo_count, last_updated 
         FROM address_balances 
         WHERE address = ?"
    )
    .bind(&addr)
    .fetch_optional(pool.get_ref())
    .await {
        Ok(Some(row)) => {
            let balance: f64 = row.get("balance");
            let utxo_count: i64 = row.get("utxo_count");
            let last_updated: String = row.get("last_updated");
            
            HttpResponse::Ok().json(AccountBalanceResponse {
                success: true,
                address: addr,
                balance,
                utxo_count,
                last_updated,
            })
        },
        Ok(None) => {
            // Calculate balance from UTXOs if not in cache
            match calculate_and_cache_balance(pool.get_ref(), &addr).await {
                Ok(balance_info) => {
                    HttpResponse::Ok().json(balance_info)
                },
                Err(e) => {
                    log::error!("Balance calculation error: {}", e);
                    HttpResponse::Ok().json(AccountBalanceResponse {
                        success: true,
                        address: addr,
                        balance: 0.0,
                        utxo_count: 0,
                        last_updated: chrono::Utc::now().to_rfc3339(),
                    })
                }
            }
        },
        Err(e) => {
            log::error!("Get balance error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "message": "Failed to get balance"
            }))
        }
    }
}

// Get account UTXOs (coins)
pub async fn get_account_coins(
    pool: web::Data<SqlitePool>,
    address: web::Path<String>,
) -> HttpResponse {
    let addr = address.into_inner();
    
    match sqlx::query(
        "SELECT tx_hash, output_index, amount, block_height, created_at
         FROM utxos
         WHERE address = ? AND is_spent = 0
         ORDER BY block_height DESC, created_at DESC"
    )
    .bind(&addr)
    .fetch_all(pool.get_ref())
    .await {
        Ok(rows) => {
            let mut total_amount = 0.0;
            let utxos: Vec<UTXO> = rows.iter().map(|row| {
                let amount: f64 = row.get("amount");
                total_amount += amount;
                
                UTXO {
                    tx_hash: row.get("tx_hash"),
                    output_index: row.get("output_index"),
                    amount,
                    block_height: row.get("block_height"),
                    created_at: row.get("created_at"),
                }
            }).collect();

            HttpResponse::Ok().json(AccountCoinsResponse {
                success: true,
                address: addr,
                utxos,
                total_amount,
            })
        },
        Err(e) => {
            log::error!("Get UTXOs error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "message": "Failed to get UTXOs"
            }))
        }
    }
}

// Helper function to calculate and cache balance
async fn calculate_and_cache_balance(
    pool: &SqlitePool,
    address: &str,
) -> Result<AccountBalanceResponse, sqlx::Error> {
    // Calculate balance from UTXOs
    let result = sqlx::query(
        "SELECT COALESCE(SUM(amount), 0.0) as balance, COUNT(*) as utxo_count
         FROM utxos
         WHERE address = ? AND is_spent = 0"
    )
    .bind(address)
    .fetch_one(pool)
    .await?;

    let balance: f64 = result.get("balance");
    let utxo_count: i64 = result.get("utxo_count");
    let now = chrono::Utc::now().to_rfc3339();

    // Cache the balance
    let _ = sqlx::query(
        "INSERT OR REPLACE INTO address_balances (address, balance, utxo_count, last_updated)
         VALUES (?, ?, ?, ?)"
    )
    .bind(address)
    .bind(balance)
    .bind(utxo_count)
    .bind(&now)
    .execute(pool)
    .await;

    Ok(AccountBalanceResponse {
        success: true,
        address: address.to_string(),
        balance,
        utxo_count,
        last_updated: now,
    })
}

// Add UTXO (called when transaction is confirmed)
#[derive(Debug, Deserialize)]
pub struct AddUTXORequest {
    pub tx_hash: String,
    pub output_index: i64,
    pub address: String,
    pub amount: f64,
    pub block_height: i64,
}

pub async fn add_utxo(
    pool: web::Data<SqlitePool>,
    req: web::Json<AddUTXORequest>,
) -> HttpResponse {
    match sqlx::query(
        "INSERT OR IGNORE INTO utxos (tx_hash, output_index, address, amount, block_height)
         VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&req.tx_hash)
    .bind(req.output_index)
    .bind(&req.address)
    .bind(req.amount)
    .bind(req.block_height)
    .execute(pool.get_ref())
    .await {
        Ok(_) => {
            // Invalidate cache
            let _ = sqlx::query("DELETE FROM address_balances WHERE address = ?")
                .bind(&req.address)
                .execute(pool.get_ref())
                .await;

            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "message": "UTXO added"
            }))
        },
        Err(e) => {
            log::error!("Add UTXO error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "message": "Failed to add UTXO"
            }))
        }
    }
}

// Spend UTXO (called when transaction spends an output)
#[derive(Debug, Deserialize)]
pub struct SpendUTXORequest {
    pub tx_hash: String,
    pub output_index: i64,
    pub spent_in_tx: String,
}

pub async fn spend_utxo(
    pool: web::Data<SqlitePool>,
    req: web::Json<SpendUTXORequest>,
) -> HttpResponse {
    match sqlx::query(
        "UPDATE utxos 
         SET is_spent = 1, spent_in_tx = ?, spent_at = CURRENT_TIMESTAMP
         WHERE tx_hash = ? AND output_index = ?"
    )
    .bind(&req.spent_in_tx)
    .bind(&req.tx_hash)
    .bind(req.output_index)
    .execute(pool.get_ref())
    .await {
        Ok(_) => {
            // Get address to invalidate cache
            if let Ok(Some(row)) = sqlx::query("SELECT address FROM utxos WHERE tx_hash = ? AND output_index = ?")
                .bind(&req.tx_hash)
                .bind(req.output_index)
                .fetch_optional(pool.get_ref())
                .await {
                let address: String = row.get("address");
                let _ = sqlx::query("DELETE FROM address_balances WHERE address = ?")
                    .bind(&address)
                    .execute(pool.get_ref())
                    .await;
            }

            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "message": "UTXO spent"
            }))
        },
        Err(e) => {
            log::error!("Spend UTXO error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "message": "Failed to spend UTXO"
            }))
        }
    }
}
