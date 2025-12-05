use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Network protocol version
pub const PROTOCOL_VERSION: u32 = 1;

/// Magic bytes to identify ATMN network packets
pub const NETWORK_MAGIC: [u8; 4] = [0x41, 0x54, 0x4D, 0x4E]; // "ATMN"

/// P2P message types for the ATMN network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    /// Handshake to establish connection
    Handshake(HandshakeMessage),
    
    /// Announce a new block
    BlockAnnounce(BlockAnnounceMessage),
    
    /// Request a specific block
    BlockRequest(BlockRequestMessage),
    
    /// Response with block data
    BlockResponse(BlockResponseMessage),
    
    /// Broadcast a new transaction
    TransactionBroadcast(TransactionMessage),
    
    /// Request peer list
    GetPeers,
    
    /// Response with peer list
    PeersResponse(PeersMessage),
    
    /// Ping to check connection
    Ping(u64),
    
    /// Pong response to ping
    Pong(u64),
    
    /// Request blockchain sync from height
    SyncRequest(SyncRequestMessage),
    
    /// Batch of blocks for sync
    SyncResponse(SyncResponseMessage),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeMessage {
    pub protocol_version: u32,
    pub node_id: String,
    pub listen_port: u16,
    pub best_block_height: u64,
    pub best_block_hash: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockAnnounceMessage {
    pub block_hash: String,
    pub block_height: u64,
    pub previous_hash: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockRequestMessage {
    pub block_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockResponseMessage {
    pub block_hash: String,
    pub block_height: u64,
    pub previous_hash: String,
    pub merkle_root: String,
    pub timestamp: DateTime<Utc>,
    pub difficulty_target: u64,
    pub nonce: u64,
    pub miner_address: String,
    pub transactions: Vec<TransactionData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionMessage {
    pub tx_hash: String,
    pub from_address: String,
    pub to_address: String,
    pub amount: f64,
    pub fee: f64,
    pub timestamp: DateTime<Utc>,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionData {
    pub tx_hash: String,
    pub from_address: String,
    pub to_address: String,
    pub amount: f64,
    pub fee: f64,
    pub timestamp: DateTime<Utc>,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeersMessage {
    pub peers: Vec<PeerInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub ip: String,
    pub port: u16,
    pub node_id: String,
    pub last_seen: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncRequestMessage {
    pub from_height: u64,
    pub max_blocks: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResponseMessage {
    pub blocks: Vec<BlockResponseMessage>,
    pub has_more: bool,
}

impl NetworkMessage {
    /// Serialize message to bytes with magic header
    pub fn to_bytes(&self) -> Result<Vec<u8>, String> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&NETWORK_MAGIC);
        let payload = bincode::serialize(self)
            .map_err(|e| format!("Serialization error: {}", e))?;
        bytes.extend_from_slice(&payload);
        Ok(bytes)
    }
    
    /// Deserialize message from bytes (with magic header)
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 4 || &bytes[0..4] != &NETWORK_MAGIC {
            return Err("Invalid magic bytes".to_string());
        }
        let message = bincode::deserialize(&bytes[4..])
            .map_err(|e| format!("Deserialization error: {}", e))?;
        Ok(message)
    }
}
