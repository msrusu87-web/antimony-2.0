use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};

/// Maximum number of peer connections
pub const MAX_PEERS: usize = 50;

/// Minimum peer quality score to maintain connection
pub const MIN_PEER_SCORE: f64 = -10.0;

/// Time before peer is considered stale
pub const PEER_TIMEOUT_MINUTES: i64 = 30;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peer {
    pub id: String,
    pub address: String,
    pub port: u16,
    pub connected_at: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub best_block_height: u64,
    pub quality_score: f64,
    pub messages_sent: u64,
    pub messages_received: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}

impl Peer {
    pub fn new(id: String, address: String, port: u16) -> Self {
        let now = Utc::now();
        Self {
            id,
            address,
            port,
            connected_at: now,
            last_seen: now,
            best_block_height: 0,
            quality_score: 0.0,
            messages_sent: 0,
            messages_received: 0,
            bytes_sent: 0,
            bytes_received: 0,
        }
    }
    
    pub fn update_last_seen(&mut self) {
        self.last_seen = Utc::now();
    }
    
    pub fn is_stale(&self) -> bool {
        Utc::now() - self.last_seen > Duration::minutes(PEER_TIMEOUT_MINUTES)
    }
    
    pub fn increment_score(&mut self, amount: f64) {
        self.quality_score += amount;
    }
    
    pub fn decrement_score(&mut self, amount: f64) {
        self.quality_score -= amount;
    }
    
    pub fn is_low_quality(&self) -> bool {
        self.quality_score < MIN_PEER_SCORE
    }
}

/// Peer manager for maintaining peer list and connections
pub struct PeerManager {
    peers: Arc<RwLock<HashMap<String, Peer>>>,
    bootstrap_nodes: Vec<(String, u16)>,
}

impl PeerManager {
    pub fn new(bootstrap_nodes: Vec<(String, u16)>) -> Self {
        Self {
            peers: Arc::new(RwLock::new(HashMap::new())),
            bootstrap_nodes,
        }
    }
    
    /// Add a new peer
    pub async fn add_peer(&self, peer: Peer) -> bool {
        let mut peers = self.peers.write().await;
        if peers.len() >= MAX_PEERS {
            log::warn!("Maximum peer limit reached");
            return false;
        }
        let peer_id = peer.id.clone();
        peers.insert(peer_id, peer);
        true
    }
    
    /// Get a peer by ID
    pub async fn get_peer(&self, peer_id: &str) -> Option<Peer> {
        let peers = self.peers.read().await;
        peers.get(peer_id).cloned()
    }
    
    /// Update peer last seen
    pub async fn update_peer_activity(&self, peer_id: &str) {
        let mut peers = self.peers.write().await;
        if let Some(peer) = peers.get_mut(peer_id) {
            peer.update_last_seen();
        }
    }
    
    /// Update peer block height
    pub async fn update_peer_height(&self, peer_id: &str, height: u64) {
        let mut peers = self.peers.write().await;
        if let Some(peer) = peers.get_mut(peer_id) {
            peer.best_block_height = height;
            peer.update_last_seen();
        }
    }
    
    /// Remove a peer
    pub async fn remove_peer(&self, peer_id: &str) -> bool {
        let mut peers = self.peers.write().await;
        peers.remove(peer_id).is_some()
    }
    
    /// Get all connected peers
    pub async fn get_all_peers(&self) -> Vec<Peer> {
        let peers = self.peers.read().await;
        peers.values().cloned().collect()
    }
    
    /// Clean up stale and low-quality peers
    pub async fn cleanup_peers(&self) {
        let mut peers = self.peers.write().await;
        peers.retain(|id, peer| {
            let should_keep = !peer.is_stale() && !peer.is_low_quality();
            if !should_keep {
                log::info!("Removing peer {} (stale: {}, low_quality: {})", 
                    id, peer.is_stale(), peer.is_low_quality());
            }
            should_keep
        });
    }
    
    /// Get bootstrap nodes
    pub fn get_bootstrap_nodes(&self) -> Vec<(String, u16)> {
        self.bootstrap_nodes.clone()
    }
    
    /// Get best peer (highest block height with good quality)
    pub async fn get_best_peer(&self) -> Option<Peer> {
        let peers = self.peers.read().await;
        peers.values()
            .filter(|p| !p.is_stale() && !p.is_low_quality())
            .max_by_key(|p| p.best_block_height)
            .cloned()
    }
    
    /// Update peer score based on behavior
    pub async fn update_peer_score(&self, peer_id: &str, delta: f64) {
        let mut peers = self.peers.write().await;
        if let Some(peer) = peers.get_mut(peer_id) {
            if delta > 0.0 {
                peer.increment_score(delta);
            } else {
                peer.decrement_score(delta.abs());
            }
            log::debug!("Peer {} score updated: {} (delta: {})", peer_id, peer.quality_score, delta);
        }
    }
    
    /// Get peer count
    pub async fn peer_count(&self) -> usize {
        let peers = self.peers.read().await;
        peers.len()
    }
}
