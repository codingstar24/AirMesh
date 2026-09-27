//! Time and clock helper functions.

use std::time::{SystemTime, UNIX_EPOCH};

/// Returns current Unix timestamp in seconds.
#[must_use]
pub fn current_timestamp_secs() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// Returns current Unix timestamp in milliseconds.
#[must_use]
pub fn current_timestamp_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}
