mod protocol;
mod peer_manager;
mod mempool;
mod network;
mod blockchain;
mod error;

use clap::Parser;
use error::NodeError;

#[derive(Parser, Debug)]
#[command(name = "atmn-node")]
#[command(about = "ATMN Full Node - P2P blockchain node", long_about = None)]
struct Args {
    /// Listen port for P2P connections
    #[arg(short, long, default_value = "9000")]
    port: u16,
    
    /// Bootstrap node addresses (format: ip:port)
    #[arg(short, long)]
    bootstrap: Vec<String>,
    
    /// Node ID (generated if not provided)
    #[arg(long)]
    node_id: Option<String>,
    
    /// Database path
    #[arg(short, long, default_value = "/home/ubuntu/atmn-node.db")]
    database: String,
    
    /// Enable mining
    #[arg(short, long)]
    mining: bool,
    
    /// Miner address for block rewards
    #[arg(long)]
    miner_address: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), NodeError> {
    env_logger::init();
    
    let args = Args::parse();
    
    log::info!("🚀 Starting ATMN Full Node");
    log::info!("   Port: {}", args.port);
    log::info!("   Database: {}", args.database);
    log::info!("   Mining: {}", args.mining);
    
    // Parse bootstrap nodes
    let bootstrap_nodes: Vec<(String, u16)> = args.bootstrap
        .iter()
        .filter_map(|addr| {
            let parts: Vec<&str> = addr.split(':').collect();
            if parts.len() == 2 {
                if let Ok(port) = parts[1].parse::<u16>() {
                    return Some((parts[0].to_string(), port));
                }
            }
            log::warn!("Invalid bootstrap node address: {}", addr);
            None
        })
        .collect();
    
    log::info!("   Bootstrap nodes: {:?}", bootstrap_nodes);
    
    // Initialize peer manager
    let peer_manager = peer_manager::PeerManager::new(bootstrap_nodes);
    
    // Initialize mempool
    let mempool = mempool::Mempool::new();
    
    // Initialize blockchain storage
    let blockchain = blockchain::Blockchain::new(&args.database).await?;
    
    // Generate or use provided node ID
    let node_id = args.node_id.unwrap_or_else(|| {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}", args.port, chrono::Utc::now().timestamp()));
        hex::encode(hasher.finalize())[..16].to_string()
    });
    
    log::info!("   Node ID: {}", node_id);
    
    // Start network service
    let network = network::NetworkService::new(
        node_id.clone(),
        args.port,
        peer_manager,
        mempool,
        blockchain,
    );
    
    // Run network service
    network.run().await?;
    
    Ok(())
}
