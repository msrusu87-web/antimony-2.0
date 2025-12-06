/// Flood protection for P2P network
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct FloodProtection {
    recent_messages: Arc<Mutex<HashMap<SocketAddr, Vec<Instant>>>>,
    window: Duration,
    max_messages: usize,
}

impl FloodProtection {
    pub fn new(window: Duration, max_messages: usize) -> Self {
        Self {
            recent_messages: Arc::new(Mutex::new(HashMap::new())),
            window,
            max_messages,
        }
    }

    pub fn check_flood(&self, peer: SocketAddr) -> bool {
        let mut messages = self.recent_messages.lock().unwrap();
        let now = Instant::now();
        
        let peer_messages = messages.entry(peer).or_insert_with(Vec::new);
        
        // Remove old messages
        peer_messages.retain(|&time| now.duration_since(time) < self.window);
        
        if peer_messages.len() >= self.max_messages {
            return true; // Flood detected
        }
        
        peer_messages.push(now);
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn test_flood_detection() {
        let protection = FloodProtection::new(Duration::from_secs(1), 10);
        let peer = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

        for _ in 0..10 {
            assert!(!protection.check_flood(peer));
        }

        assert!(protection.check_flood(peer)); // 11th message should trigger flood
    }
}
