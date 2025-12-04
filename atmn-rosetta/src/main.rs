use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;
mod types;
mod error;
mod converters;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "atmn_rosetta=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Build CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router with all Rosetta endpoints
    let app = Router::new()
        // Network endpoints
        .route("/network/list", post(handlers::network_list))
        .route("/network/options", post(handlers::network_options))
        .route("/network/status", post(handlers::network_status))
        // Block endpoints
        .route("/block", post(handlers::block))
        .route("/block/transaction", post(handlers::block_transaction))
        // Mempool endpoints
        .route("/mempool", post(handlers::mempool))
        .route("/mempool/transaction", post(handlers::mempool_transaction))
        // Account endpoints
        .route("/account/balance", post(handlers::account_balance))
        .route("/account/coins", post(handlers::account_coins))
        // Construction endpoints
        .route("/construction/preprocess", post(handlers::construction_preprocess))
        .route("/construction/metadata", post(handlers::construction_metadata))
        .route("/construction/payloads", post(handlers::construction_payloads))
        .route("/construction/parse", post(handlers::construction_parse))
        .route("/construction/combine", post(handlers::construction_combine))
        .route("/construction/hash", post(handlers::construction_hash))
        .route("/construction/submit", post(handlers::construction_submit))
        // Health check
        .route("/health", get(handlers::health))
        .layer(cors)
        .layer(tower_http::trace::TraceLayer::new_for_http());

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Starting Antimony Rosetta API server on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
