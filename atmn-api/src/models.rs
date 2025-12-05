use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Wallet {
    pub wallet_address: String,
    pub private_key_hash: String,
    pub balance: f64,
    pub created_at: DateTime<Utc>,
    pub last_accessed: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub tx_hash: String,
    pub from_address: String,
    pub to_address: String,
    pub amount: f64,
    pub status: String,
    #[sqlx(rename = "timestamp")]
    pub created_at: DateTime<Utc>,
    pub confirmed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MiningWorker {
    pub worker_id: String,
    #[sqlx(rename = "miner_address")]
    pub wallet_address: String,
    pub hashrate: f64,
    pub shares_found: i64,
    pub registered_at: DateTime<Utc>,
    pub last_share: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MiningBlock {
    pub id: i64,
    pub block_height: i64,
    pub miner_address: String,
    pub amount: f64,
    pub confirmed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MasterWallet {
    pub id: i64,
    pub balance: f64,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MasterTransfer {
    pub transfer_id: String,
    pub to_address: String,
    pub amount: f64,
    pub purpose: String,
    pub created_at: DateTime<Utc>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MiningPayout {
    pub id: i64,
    pub miner_address: String,
    pub amount: f64,
    pub created_at: DateTime<Utc>,
    pub confirmed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: i64,
    pub wallet_address: String,
    pub token: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

// Request/Response DTOs
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWalletRequest {
    pub private_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWalletResponse {
    pub address: String,
    pub private_key_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransactionRequest {
    pub from_address: String,
    pub to_address: String,
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransactionResponse {
    pub tx_hash: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MasterTransferRequest {
    pub to_address: String,
    pub amount: f64,
    pub purpose: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}
