//! Localhost CORS and origin security verification.

use axum::http::{HeaderMap, StatusCode};
use tower_http::cors::{AllowOrigin, Any, CorsLayer};

/// Builds strict localhost-only CORS layer to protect against Drive-By local network attacks.
pub fn localhost_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(|origin, _| {
            let s = origin.to_str().unwrap_or("");
            s.starts_with("http://localhost")
                || s.starts_with("http://127.0.0.1")
                || s.starts_with("app://")
                || s.starts_with("tauri://")
        }))
        .allow_methods(Any)
        .allow_headers(Any)
}
