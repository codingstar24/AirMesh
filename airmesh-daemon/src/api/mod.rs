//! HTTP API module router and server setup.

pub mod auth;
pub mod errors;
pub mod routes;
pub mod sse;
pub mod websocket;

use crate::state::AppState;
use auth::localhost_cors_layer;
use axum::routing::{get, post};
use axum::Router;

/// Builds complete axum router for AirMesh API and services.
pub fn build_api_router(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(routes::handle_health))
        .route("/api/metrics", get(routes::handle_metrics))
        .route("/api/peers", get(routes::handle_list_peers))
        .route("/api/peers/pair", post(routes::handle_pair_peer))
        .route("/api/files", get(routes::handle_list_files))
        .route("/api/search", get(routes::handle_search))
        .route("/api/stream/:file_id", get(routes::handle_stream_file))
        .route("/api/transfers", get(routes::handle_list_transfers))
        .route("/api/ws", get(websocket::handle_websocket_upgrade))
        .route("/api/sse", get(sse::handle_sse))
        .route("/debug", get(routes::handle_debug))
        .layer(localhost_cors_layer())
        .with_state(state)
}
