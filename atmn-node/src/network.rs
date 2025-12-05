use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{sleep, Duration};
use crate::protocol::{NetworkMessage, HandshakeMessage, BlockAnnounceMessage, PROTOCOL_VERSION};
use crate::peer_manager::{PeerManager, Peer};
use crate::mempool::Mempool;
use crate::blockchain::Blockchain;
use chrono::Utc;

use crate::error::NodeError;

pub struct NetworkService {
    node_id: String,
    listen_port: u16,
    peer_manager: Arc<PeerManager>,
    mempool: Arc<Mempool>,
    blockchain: Arc<Blockchain>,
}

impl NetworkService {
    pub fn new(
        node_id: String,
        listen_port: u16,
        peer_manager: PeerManager,
        mempool: Mempool,
        blockchain: Blockchain,
    ) -> Self {
        Self {
            node_id,
            listen_port,
            peer_manager: Arc::new(peer_manager),
            mempool: Arc::new(mempool),
            blockchain: Arc::new(blockchain),
        }
    }
    
    pub async fn run(&self) -> Result<(), NodeError> {
        // Start listening for incoming connections
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.listen_port)).await?;
        log::info!("📡 Listening for P2P connections on port {}", self.listen_port);
        
        // Spawn peer cleanup task
        let pm_cleanup = self.peer_manager.clone();
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(300)).await; // Every 5 minutes
                pm_cleanup.cleanup_peers().await;
            }
        });
        
        // Spawn mempool cleanup task
        let mempool_cleanup = self.mempool.clone();
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(600)).await; // Every 10 minutes
                mempool_cleanup.cleanup_old_transactions(60).await; // Remove txs older than 1 hour
            }
        });
        
        // Connect to bootstrap nodes
        let bootstrap_nodes = self.peer_manager.get_bootstrap_nodes();
        for (ip, port) in bootstrap_nodes {
            let peer_manager = self.peer_manager.clone();
            let node_id = self.node_id.clone();
            let blockchain = self.blockchain.clone();
            tokio::spawn(async move {
                if let Err(e) = Self::connect_to_peer(ip.clone(), port, node_id, peer_manager, blockchain).await {
                    log::error!("Failed to connect to bootstrap node {}:{}: {}", ip, port, e);
                }
            });
        }
        
        // Accept incoming connections
        loop {
            match listener.accept().await {
                Ok((socket, addr)) => {
                    log::info!("New connection from: {}", addr);
                    let peer_manager = self.peer_manager.clone();
                    let mempool = self.mempool.clone();
                    let blockchain = self.blockchain.clone();
                    let node_id = self.node_id.clone();
                    
                    tokio::spawn(async move {
                        if let Err(e) = Self::handle_connection(socket, node_id, peer_manager, mempool, blockchain).await {
                            log::error!("Connection error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    log::error!("Failed to accept connection: {}", e);
                }
            }
        }
    }
    
    async fn connect_to_peer(
        ip: String,
        port: u16,
        node_id: String,
        peer_manager: Arc<PeerManager>,
        blockchain: Arc<Blockchain>,
    ) -> Result<(), NodeError> {
        log::info!("Connecting to peer: {}:{}", ip, port);
        
        let mut stream = TcpStream::connect(format!("{}:{}", ip, port)).await?;
        
        // Send handshake
        let height = blockchain.get_height().await?;
        let best_hash = if height > 0 {
            blockchain.get_best_block_hash().await?
        } else {
            "0000000000000000000000000000000000000000000000000000000000000000".to_string()
        };
        
        let handshake = NetworkMessage::Handshake(HandshakeMessage {
            protocol_version: PROTOCOL_VERSION,
            node_id: node_id.clone(),
            listen_port: port,
            best_block_height: height,
            best_block_hash: best_hash,
            timestamp: Utc::now(),
        });
        
        let bytes = handshake.to_bytes()?;
        stream.write_all(&bytes).await?;
        
        log::info!("Sent handshake to {}:{}", ip, port);
        
        // Add to peer list
        let peer = Peer::new(format!("{}:{}", ip, port), ip.clone(), port);
        peer_manager.add_peer(peer).await;
        
        Ok(())
    }
    
    async fn handle_connection(
        mut socket: TcpStream,
        node_id: String,
        peer_manager: Arc<PeerManager>,
        mempool: Arc<Mempool>,
        blockchain: Arc<Blockchain>,
    ) -> Result<(), NodeError> {
        let mut buffer = vec![0u8; 65536]; // 64KB buffer
        
        loop {
            let n = socket.read(&mut buffer).await?;
            
            if n == 0 {
                log::info!("Connection closed");
                break;
            }
            
            // Parse message
            match NetworkMessage::from_bytes(&buffer[..n]) {
                Ok(message) => {
                    Self::handle_message(message, &mut socket, &node_id, &peer_manager, &mempool, &blockchain).await?;
                }
                Err(e) => {
                    log::warn!("Failed to parse message: {}", e);
                }
            }
        }
        
        Ok(())
    }
    
    async fn handle_message(
        message: NetworkMessage,
        socket: &mut TcpStream,
        node_id: &str,
        peer_manager: &Arc<PeerManager>,
        mempool: &Arc<Mempool>,
        blockchain: &Arc<Blockchain>,
    ) -> Result<(), NodeError> {
        match message {
            NetworkMessage::Handshake(handshake) => {
                log::info!("Received handshake from node: {} (height: {})", 
                    handshake.node_id, handshake.best_block_height);
                
                // Update peer info
                let peer = Peer::new(
                    handshake.node_id.clone(),
                    "unknown".to_string(), // Would need to extract from socket
                    handshake.listen_port,
                );
                peer_manager.add_peer(peer).await;
                peer_manager.update_peer_height(&handshake.node_id, handshake.best_block_height).await;
                
                // Send our handshake back
                let height = blockchain.get_height().await?;
                let best_hash = if height > 0 {
                    blockchain.get_best_block_hash().await?
                } else {
                    "0000000000000000000000000000000000000000000000000000000000000000".to_string()
                };
                
                let response = NetworkMessage::Handshake(HandshakeMessage {
                    protocol_version: PROTOCOL_VERSION,
                    node_id: node_id.to_string(),
                    listen_port: 9000, // TODO: get from config
                    best_block_height: height,
                    best_block_hash: best_hash,
                    timestamp: Utc::now(),
                });
                
                let bytes = response.to_bytes()?;
                socket.write_all(&bytes).await?;
            }
            
            NetworkMessage::BlockAnnounce(announce) => {
                log::info!("Received block announcement: {} at height {}", 
                    announce.block_hash, announce.block_height);
                
                // Check if we have this block
                if let Ok(None) = blockchain.get_block(&announce.block_hash).await {
                    // Request the block
                    let request = NetworkMessage::BlockRequest(
                        crate::protocol::BlockRequestMessage {
                            block_hash: announce.block_hash.clone(),
                        }
                    );
                    let bytes = request.to_bytes()?;
                    socket.write_all(&bytes).await?;
                }
            }
            
            NetworkMessage::BlockRequest(request) => {
                log::info!("Received block request: {}", request.block_hash);
                
                if let Ok(Some(block)) = blockchain.get_block(&request.block_hash).await {
                    let response = NetworkMessage::BlockResponse(block);
                    let bytes = response.to_bytes()?;
                    socket.write_all(&bytes).await?;
                }
            }
            
            NetworkMessage::BlockResponse(block) => {
                log::info!("Received block: {} at height {}", block.block_hash, block.block_height);
                
                // Validate and add block
                if let Err(e) = blockchain.add_block(&block).await {
                    log::error!("Failed to add block: {}", e);
                } else {
                    // Remove transactions from mempool
                    for tx in &block.transactions {
                        mempool.remove_transaction(&tx.tx_hash).await;
                    }
                }
            }
            
            NetworkMessage::TransactionBroadcast(tx) => {
                log::info!("Received transaction: {} (amount: {}, fee: {})", 
                    tx.tx_hash, tx.amount, tx.fee);
                
                if let Err(e) = mempool.add_transaction(tx).await {
                    log::warn!("Failed to add transaction to mempool: {}", e);
                }
            }
            
            NetworkMessage::Ping(nonce) => {
                let pong = NetworkMessage::Pong(nonce);
                let bytes = pong.to_bytes()?;
                socket.write_all(&bytes).await?;
            }
            
            NetworkMessage::Pong(nonce) => {
                log::debug!("Received pong: {}", nonce);
            }
            
            _ => {
                log::debug!("Received unhandled message type");
            }
        }
        
        Ok(())
    }
}
