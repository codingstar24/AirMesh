//! REST HTTP routes and request handlers.

use crate::api::errors::ProblemDetails;
use crate::state::AppState;
use airmesh_core::crypto::pake::PakePairing;
use airmesh_core::crypto::PeerTrustStatus;
use airmesh_core::index::types::SearchQuery;
use airmesh_core::transfer::streaming::MediaStreamReader;
use axum::extract::{Path, Query, State};
use axum::http::header::{CONTENT_LENGTH, CONTENT_RANGE, CONTENT_TYPE, RANGE};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Health check response.
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub node_id: String,
    pub device_name: String,
    pub version: &'static str,
    pub peer_count: usize,
}

/// Handler for `GET /api/health`.
pub async fn handle_health(State(state): State<AppState>) -> Json<HealthResponse> {
    let peers = state.discovery.list_peers().await;
    Json(HealthResponse {
        status: "ok",
        node_id: state.identity.node_id().to_string(),
        device_name: state.config.device_name.clone(),
        version: "1.0.0",
        peer_count: peers.len(),
    })
}

/// Handler for `GET /api/metrics`.
pub async fn handle_metrics(State(state): State<AppState>) -> impl IntoResponse {
    let output = state.metrics.render_prometheus();
    (
        StatusCode::OK,
        [("content-type", "text/plain; version=0.0.4; charset=utf-8")],
        output,
    )
}

/// Handler for `GET /api/peers`.
pub async fn handle_list_peers(State(state): State<AppState>) -> impl IntoResponse {
    let peers = state.discovery.list_peers().await;
    Json(peers)
}

/// Request body for pairing.
#[derive(Deserialize)]
pub struct PairRequest {
    pub peer_node_id: String,
    pub pin_code: String,
    pub salt_hex: String,
}

/// Handler for `POST /api/peers/pair`.
pub async fn handle_pair_peer(
    State(state): State<AppState>,
    Json(req): Json<PairRequest>,
) -> Result<Json<serde_json::Value>, ProblemDetails> {
    let salt_bytes = airmesh_core::crypto::hex::decode(&req.salt_hex)
        .map_err(|e| ProblemDetails::new(StatusCode::BAD_REQUEST, "Invalid Salt", &e))?;

    if salt_bytes.len() != 16 {
        return Err(ProblemDetails::new(
            StatusCode::BAD_REQUEST,
            "Invalid Salt Length",
            "Salt must be 16 bytes hex",
        ));
    }

    let mut salt = [0u8; 16];
    salt.copy_from_slice(&salt_bytes);

    let pairing = PakePairing::from_code(&req.pin_code, salt);
    let confirmation_token = pairing.derive_confirmation_token(
        state.identity.node_id(),
        &req.peer_node_id,
    );

    let mut acl = state.acl.write().await;
    acl.set_peer(&req.peer_node_id, PeerTrustStatus::Trusted, None);

    Ok(Json(serde_json::json!({
        "status": "paired",
        "peer_node_id": req.peer_node_id,
        "token": airmesh_core::crypto::hex::encode(&confirmation_token)
    })))
}

/// Handler for `GET /api/files`.
pub async fn handle_list_files(State(state): State<AppState>) -> Result<Json<serde_json::Value>, ProblemDetails> {
    let files = state
        .db
        .list_files(0, 1000)
        .map_err(|e| ProblemDetails::new(StatusCode::INTERNAL_SERVER_ERROR, "Database Error", &e.to_string()))?;
    Ok(Json(serde_json::json!({ "files": files })))
}

/// Query parameters for search.
#[derive(Deserialize)]
pub struct SearchParams {
    pub q: Option<String>,
    pub mime: Option<String>,
    pub limit: Option<usize>,
}

/// Handler for `GET /api/search`.
pub async fn handle_search(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Result<Json<serde_json::Value>, ProblemDetails> {
    let query = SearchQuery {
        query: params.q.unwrap_or_default(),
        mime_filter: params.mime,
        limit: params.limit.unwrap_or(50),
        ..Default::default()
    };

    let hits = state
        .db
        .search(&query)
        .map_err(|e| ProblemDetails::new(StatusCode::INTERNAL_SERVER_ERROR, "Search Error", &e.to_string()))?;

    Ok(Json(serde_json::json!({ "results": hits })))
}

/// Handler for `GET /api/stream/:file_id` supporting HTTP Range.
pub async fn handle_stream_file(
    State(state): State<AppState>,
    Path(file_id): Path<String>,
    headers: HeaderMap,
) -> Result<Response, ProblemDetails> {
    let file = state
        .db
        .get_file_by_id(&file_id)
        .map_err(|e| ProblemDetails::new(StatusCode::INTERNAL_SERVER_ERROR, "DB Error", &e.to_string()))?
        .ok_or_else(|| ProblemDetails::new(StatusCode::NOT_FOUND, "Not Found", "File ID does not exist"))?;

    let file_path = PathBuf::from(&file.local_path);
    let total_size = file.size;

    if let Some(range_val) = headers.get(RANGE).and_then(|v| v.to_str().ok()) {
        if let Some((start, length)) = MediaStreamReader::parse_range_header(range_val, total_size) {
            let data = MediaStreamReader::read_range(&file_path, start, length)
                .map_err(|e| ProblemDetails::new(StatusCode::INTERNAL_SERVER_ERROR, "Read Error", &e.to_string()))?;

            let end = start + (data.len() as u64).saturating_sub(1);
            let content_range = format!("bytes {start}-{end}/{total_size}");

            let mut resp = Response::builder()
                .status(StatusCode::PARTIAL_CONTENT)
                .header(CONTENT_TYPE, file.mime_type)
                .header(CONTENT_LENGTH, data.len().to_string())
                .header(CONTENT_RANGE, content_range)
                .header("Accept-Ranges", "bytes")
                .body(axum::body::Body::from(data))
                .unwrap();
            return Ok(resp);
        }
    }

    // Default full read or initial stream chunk
    let data = MediaStreamReader::read_range(&file_path, 0, (32 * 1024 * 1024).min(total_size as u32))
        .map_err(|e| ProblemDetails::new(StatusCode::INTERNAL_SERVER_ERROR, "Read Error", &e.to_string()))?;

    let resp = Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, file.mime_type)
        .header(CONTENT_LENGTH, data.len().to_string())
        .header("Accept-Ranges", "bytes")
        .body(axum::body::Body::from(data))
        .unwrap();

    Ok(resp)
}

/// Handler for `GET /api/transfers`.
pub async fn handle_list_transfers(State(state): State<AppState>) -> Json<serde_json::Value> {
    let transfers = state.active_transfers.read().await;
    let list: Vec<_> = transfers.values().cloned().collect();
    Json(serde_json::json!({ "transfers": list }))
}

/// Handler for `GET /debug`.
pub async fn handle_debug(State(state): State<AppState>) -> Json<serde_json::Value> {
    let peers = state.discovery.list_peers().await;
    let acl = state.acl.read().await;
    Json(serde_json::json!({
        "node_id": state.identity.node_id(),
        "device_name": state.config.device_name,
        "shared_dir": state.config.shared_dir.to_string_lossy(),
        "download_dir": state.config.download_dir.to_string_lossy(),
        "peers": peers,
        "acl_entries": acl.entries(),
    }))
}
