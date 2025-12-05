mod db;
mod errors;
mod handlers;
mod models;

use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use dotenv::dotenv;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:///home/ubuntu/atmn.db".to_string());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    log::info!("Starting ATMN API server...");
    log::info!("Database: {}", database_url);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .wrap(cors)
            .route("/health", web::get().to(handlers::health::health_check))
            // Authentication endpoints
            .route("/api/auth/register", web::post().to(handlers::auth::register))
            .route("/api/auth/login", web::post().to(handlers::auth::login))
            .route("/api/auth/verify-2fa", web::post().to(handlers::auth::verify_2fa))
            .route("/api/auth/enable-2fa", web::post().to(handlers::auth::enable_2fa))
            .route("/api/auth/confirm-2fa", web::post().to(handlers::auth::confirm_2fa))
            .route("/api/auth/disable-2fa", web::post().to(handlers::auth::disable_2fa))
            // Wallet endpoints
            .route("/api/wallets/create", web::post().to(handlers::wallet::create_wallet))
            .route("/api/wallets/{address}", web::get().to(handlers::wallet::get_wallet))
            .route("/api/wallets/{address}/balance", web::get().to(handlers::wallet::get_balance))
            .route("/api/wallets/verify", web::post().to(handlers::wallet::verify_wallet))
            // Transaction endpoints
            .route("/api/transactions", web::post().to(handlers::transaction::create_transaction))
            .route("/api/transactions/{address}", web::get().to(handlers::transaction::get_transactions))
            .route("/api/transactions/{tx_hash}", web::get().to(handlers::transaction::get_transaction))
            // Master wallet endpoints
            .route("/api/master-wallet/info", web::get().to(handlers::master::get_master_info))
            .route("/api/master-wallet/transfer", web::post().to(handlers::master::transfer_premine))
            .route("/api/master-wallet/transfers", web::get().to(handlers::master::get_transfers))
            // Mining endpoints
            .route("/api/mining/worker/register", web::post().to(handlers::mining::register_worker))
            .route("/api/mining/workers", web::get().to(handlers::mining::get_workers))
            .route("/api/mining/stats", web::get().to(handlers::mining::get_pool_stats))
            .route("/api/mining/payouts/{address}", web::get().to(handlers::mining::get_payouts))
            // Fee collection endpoints
            .route("/api/fees/collect", web::post().to(handlers::fees::collect_fee))
            .route("/api/fees/master-wallet", web::get().to(handlers::fees::get_master_wallet_stats))
            .route("/api/fees/history", web::get().to(handlers::fees::get_fee_history))
            // User wallet management endpoints
            .route("/api/user-wallets/{email}", web::get().to(handlers::wallets_new::get_user_wallets))
            .route("/api/user-wallets/create", web::post().to(handlers::wallets_new::create_new_wallet))
            .route("/api/user-wallets/{email}/default/{address}", web::post().to(handlers::wallets_new::set_default_wallet))
            .route("/api/auth/2fa/status/{token}", web::get().to(handlers::auth::get_2fa_status))
            // Account balance endpoints (Phase 2d)
            .route("/api/account/balance/{address}", web::get().to(handlers::account::get_account_balance))
            .route("/api/account/coins/{address}", web::get().to(handlers::account::get_account_coins))
            .route("/api/account/utxo/add", web::post().to(handlers::account::add_utxo))
            .route("/api/account/utxo/spend", web::post().to(handlers::account::spend_utxo))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
