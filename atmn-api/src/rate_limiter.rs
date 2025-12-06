// Rate limiting middleware for API protection
use actix_web::{dev::ServiceRequest, Error, HttpResponse};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct RateLimiter {
    requests: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
    max_requests: usize,
    window_secs: u64,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_secs: u64) -> Self {
        RateLimiter {
            requests: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window_secs,
        }
    }
    
    pub fn check_rate_limit(&self, ip: &str) -> bool {
        let mut requests = self.requests.lock().unwrap();
        let now = Instant::now();
        let window = Duration::from_secs(self.window_secs);
        
        let entry = requests.entry(ip.to_string()).or_insert_with(Vec::new);
        
        // Remove old requests outside the window
        entry.retain(|&time| now.duration_since(time) < window);
        
        // Check if limit exceeded
        if entry.len() >= self.max_requests {
            false
        } else {
            entry.push(now);
            true
        }
    }
    
    pub fn cleanup_old_entries(&self) {
        let mut requests = self.requests.lock().unwrap();
        let now = Instant::now();
        let window = Duration::from_secs(self.window_secs);
        
        requests.retain(|_, times| {
            times.retain(|&time| now.duration_since(time) < window);
            !times.is_empty()
        });
    }
}

// Global rate limiter instance
lazy_static::lazy_static! {
    pub static ref GLOBAL_RATE_LIMITER: RateLimiter = RateLimiter::new(100, 60);
}
