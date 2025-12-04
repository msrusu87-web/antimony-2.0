use axum::{
    routing::{get, post},
    Router,
    extract::State,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use atmn_core::Storage;

mod handlers;
mod types;
mod error;
mod converters;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub storage: Arc<Storage>,
}

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

    // Initialize storage
    let storage_path = std::env::var("ATMN_DATA_DIR")
        .unwrap_or_else(|_| "./data".to_string());
    tracing::info!("Opening storage at: {}", storage_path);
    
    let storage = Storage::new(&storage_path)
        .expect("Failed to initialize storage");
    
    // Initialize genesis block if needed
    atmn_core::initialize_genesis(&storage)
        .expect("Failed to initialize genesis block");
    tracing::info!("Genesis block initialized");
    
    let state = AppState {
        storage: Arc::new(storage),
    };

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
        .with_state(state)
        .layer(cors)
        .layer(tower_http::trace::TraceLayer::new_for_http());

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Starting Antimony Rosetta API server on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
