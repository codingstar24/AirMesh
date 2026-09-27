//! Token-bucket rate limiter for ingress request throttling.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

/// Token-bucket rate limiter state.
pub struct RateLimiter {
    capacity: u32,
    refill_rate_per_sec: u32,
    buckets: Mutex<HashMap<String, (f64, Instant)>>,
}

impl RateLimiter {
    /// Creates a rate limiter with bucket capacity and refill rate per second.
    pub fn new(capacity: u32, refill_rate_per_sec: u32) -> Self {
        Self {
            capacity,
            refill_rate_per_sec,
            buckets: Mutex::new(HashMap::new()),
        }
    }

    /// Checks if a client with identifier `key` is allowed to proceed with 1 token.
    pub fn allow(&self, key: &str) -> bool {
        let mut guard = self.buckets.lock().unwrap();
        let now = Instant::now();

        let entry = guard.entry(key.to_string()).or_insert((self.capacity as f64, now));
        let (tokens, last_refill) = entry;

        let elapsed = now.duration_since(*last_refill).as_secs_f64();
        *tokens = (*tokens + elapsed * (self.refill_rate_per_sec as f64)).min(self.capacity as f64);
        *last_refill = now;

        if *tokens >= 1.0 {
            *tokens -= 1.0;
            true
        } else {
            false
        }
    }
}
