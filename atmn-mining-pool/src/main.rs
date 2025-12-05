// atmn-mining-pool/src/main.rs
// ATMN Mining Pool Server with Enhanced Connection Instructions

use axum::{
    routing::{get, post},
    Router, Json, extract::State,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{CorsLayer, Any};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║      ANTIMONY MINING POOL - Enhanced Version              ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    
    let pool_state = Arc::new(RwLock::new(PoolState::new()));

    let app = Router::new()
        .route("/", get(serve_dashboard))
        .route("/api/stats", get(get_pool_stats))
        .route("/api/workers", get(get_workers))
        .route("/api/blocks", get(get_blocks))
        .route("/api/connect", post(connect_worker))
        .route("/api/submit", post(submit_share))
        .route("/api/config", get(get_pool_config))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .with_state(pool_state);

    let addr = "0.0.0.0:3001";
    println!("🚀 Enhanced mining pool at https://miningpool.carphatian.ro");
    println!("📖 Connection instructions available on dashboard");
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct PoolState {
    workers: HashMap<String, Worker>,
    blocks_found: Vec<BlockFound>,
    shares_submitted: u64,
    total_hashrate: f64,
    pool_fee: f64,
    started_at: DateTime<Utc>,
    difficulty: u64,
}

impl PoolState {
    fn new() -> Self {
        PoolState {
            workers: HashMap::new(),
            blocks_found: Vec::new(),
            shares_submitted: 0,
            total_hashrate: 0.0,
            pool_fee: 0.02,
            started_at: Utc::now(),
            difficulty: 1000000,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct Worker {
    id: String,
    address: String,
    hashrate: f64,
    shares_submitted: u64,
    shares_accepted: u64,
    shares_rejected: u64,
    last_share: DateTime<Utc>,
    connected_at: DateTime<Utc>,
    is_active: bool,
}

#[derive(Clone, Serialize, Deserialize)]
struct BlockFound {
    height: u64,
    hash: String,
    reward: u64,
    timestamp: DateTime<Utc>,
    miner: String,
    difficulty: u64,
}

#[derive(Serialize)]
struct PoolConfig {
    pool_url: String,
    algorithm: String,
    pool_fee: f64,
    min_payout: f64,
    block_reward: u64,
    examples: Vec<String>,
}

#[derive(Serialize)]
struct PoolStats {
    workers_count: usize,
    active_workers: usize,
    total_hashrate: f64,
    shares_submitted: u64,
    shares_accepted: u64,
    shares_rejected: u64,
    blocks_found: usize,
    pool_fee: f64,
    uptime_seconds: i64,
    difficulty: u64,
}

#[derive(Deserialize)]
struct ConnectRequest {
    worker_name: String,
    address: String,
}

#[derive(Serialize)]
struct ConnectResponse {
    worker_id: String,
    status: String,
    message: String,
}

#[derive(Deserialize)]
struct SubmitShareRequest {
    worker_id: String,
    nonce: u32,
    block_height: u64,
    hash: String,
}

#[derive(Serialize)]
struct SubmitShareResponse {
    accepted: bool,
    message: String,
}


