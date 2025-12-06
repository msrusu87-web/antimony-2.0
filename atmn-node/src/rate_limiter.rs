/// Rate Limiter for P2P Network
/// Prevents DDoS attacks and spam by limiting messages per peer
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Rate limit configuration
#[derive(Clone, Debug)]
pub struct RateLimitConfig {
    /// Maximum messages per second per peer
    pub max_messages_per_sec: u32,
    /// Maximum connections per IP
    pub max_connections_per_ip: u32,
    /// Time window for rate limiting (seconds)
    pub window_secs: u64,
    /// Ban duration for violating peers (seconds)
    pub ban_duration_secs: u64,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_messages_per_sec: 100,
            max_connections_per_ip: 3,
            window_secs: 1,
            ban_duration_secs: 300, // 5 minutes
        }
    }
}

/// Tracks message counts for a peer
#[derive(Debug, Clone)]
struct PeerRateData {
    message_count: u32,
    window_start: Instant,
    violations: u32,
    banned_until: Option<Instant>,
}

impl PeerRateData {
    fn new() -> Self {
        Self {
            message_count: 0,
            window_start: Instant::now(),
            violations: 0,
            banned_until: None,
        }
    }

    fn is_banned(&self) -> bool {
        if let Some(ban_time) = self.banned_until {
            Instant::now() < ban_time
        } else {
            false
        }
    }

    fn reset_window(&mut self) {
        self.message_count = 0;
        self.window_start = Instant::now();
    }
}

/// Rate limiter for P2P connections
pub struct RateLimiter {
    config: RateLimitConfig,
    peer_data: Arc<Mutex<HashMap<SocketAddr, PeerRateData>>>,
    ip_connections: Arc<Mutex<HashMap<std::net::IpAddr, u32>>>,
    blacklist: Arc<Mutex<HashMap<std::net::IpAddr, Instant>>>,
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            peer_data: Arc::new(Mutex::new(HashMap::new())),
            ip_connections: Arc::new(Mutex::new(HashMap::new())),
            blacklist: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Check if a message from peer should be allowed
    pub fn check_message(&self, peer: SocketAddr) -> bool {
        let mut data = self.peer_data.lock().unwrap();
        
        // Get or create peer data
        let peer_data = data.entry(peer).or_insert_with(PeerRateData::new);

        // Check if peer is banned
        if peer_data.is_banned() {
            return false;
        }

        // Check if IP is blacklisted
        if self.is_blacklisted(peer.ip()) {
            return false;
        }

        // Check if time window has passed
        let elapsed = peer_data.window_start.elapsed();
        if elapsed >= Duration::from_secs(self.config.window_secs) {
            peer_data.reset_window();
        }

        // Check message count
        peer_data.message_count += 1;
        if peer_data.message_count > self.config.max_messages_per_sec {
            // Rate limit exceeded
            peer_data.violations += 1;
            
            // Ban after 3 violations
            if peer_data.violations >= 3 {
                peer_data.banned_until = Some(
                    Instant::now() + Duration::from_secs(self.config.ban_duration_secs)
                );
                log::warn!("Peer {} banned for rate limit violations", peer);
            }
            
            return false;
        }

        true
    }

    /// Check if a new connection should be allowed
    pub fn check_connection(&self, peer: SocketAddr) -> bool {
        // Check blacklist
        if self.is_blacklisted(peer.ip()) {
            return false;
        }

        let mut connections = self.ip_connections.lock().unwrap();
        let count = connections.entry(peer.ip()).or_insert(0);
        
        if *count >= self.config.max_connections_per_ip {
            log::warn!("Connection limit exceeded for IP: {}", peer.ip());
            return false;
        }

        *count += 1;
        true
    }

    /// Release a connection
    pub fn release_connection(&self, peer: SocketAddr) {
        let mut connections = self.ip_connections.lock().unwrap();
        if let Some(count) = connections.get_mut(&peer.ip()) {
            if *count > 0 {
                *count -= 1;
            }
            if *count == 0 {
                connections.remove(&peer.ip());
            }
        }
    }

    /// Add IP to blacklist
    pub fn blacklist_ip(&self, ip: std::net::IpAddr, duration: Duration) {
        let mut blacklist = self.blacklist.lock().unwrap();
        blacklist.insert(ip, Instant::now() + duration);
        log::warn!("IP {} blacklisted for {} seconds", ip, duration.as_secs());
    }

    /// Check if IP is blacklisted
    pub fn is_blacklisted(&self, ip: std::net::IpAddr) -> bool {
        let blacklist = self.blacklist.lock().unwrap();
        if let Some(&until) = blacklist.get(&ip) {
            Instant::now() < until
        } else {
            false
        }
    }

    /// Remove expired blacklist entries
    pub fn cleanup_blacklist(&self) {
        let mut blacklist = self.blacklist.lock().unwrap();
        let now = Instant::now();
        blacklist.retain(|_, &mut until| now < until);
    }

    /// Get peer statistics
    pub fn get_peer_stats(&self, peer: SocketAddr) -> Option<(u32, u32, bool)> {
        let data = self.peer_data.lock().unwrap();
        data.get(&peer).map(|p| {
            (p.message_count, p.violations, p.is_banned())
        })
    }

    /// Reset peer data (for testing)
    pub fn reset_peer(&self, peer: SocketAddr) {
        let mut data = self.peer_data.lock().unwrap();
        data.remove(&peer);
    }

    /// Get total number of tracked peers
    pub fn tracked_peer_count(&self) -> usize {
        let data = self.peer_data.lock().unwrap();
        data.len()
    }

    /// Get blacklist size
    pub fn blacklist_size(&self) -> usize {
        let blacklist = self.blacklist.lock().unwrap();
        blacklist.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn test_rate_limiter() {
        let config = RateLimitConfig {
            max_messages_per_sec: 5,
            max_connections_per_ip: 2,
            window_secs: 1,
            ban_duration_secs: 60,
        };
        
        let limiter = RateLimiter::new(config);
        let peer = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

        // Should allow first 5 messages
        for _ in 0..5 {
            assert!(limiter.check_message(peer));
        }

        // 6th message should be blocked
        assert!(!limiter.check_message(peer));
    }

    #[test]
    fn test_connection_limit() {
        let config = RateLimitConfig::default();
        let limiter = RateLimiter::new(config);
        
        let peer1 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let peer2 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8081);
        let peer3 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8082);
        let peer4 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8083);

        assert!(limiter.check_connection(peer1));
        assert!(limiter.check_connection(peer2));
        assert!(limiter.check_connection(peer3));
        assert!(!limiter.check_connection(peer4)); // 4th connection should fail
    }

    #[test]
    fn test_blacklist() {
        let limiter = RateLimiter::new(RateLimitConfig::default());
        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1));

        assert!(!limiter.is_blacklisted(ip));
        
        limiter.blacklist_ip(ip, Duration::from_secs(60));
        assert!(limiter.is_blacklisted(ip));
    }
}
