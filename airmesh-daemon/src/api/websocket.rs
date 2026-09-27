//! WebSocket real-time subscription endpoint for events.

use crate::state::AppState;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::IntoResponse;
use std::time::Duration;
use tracing::info;

/// Upgrade handler for `GET /api/ws`.
pub async fn handle_websocket_upgrade(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_websocket_connection(socket, state))
}

async fn handle_websocket_connection(mut socket: WebSocket, state: AppState) {
    let mut interval = tokio::time::interval(Duration::from_millis(500));

    loop {
        tokio::select! {
            _ = interval.tick() => {
                let peers = state.discovery.list_peers().await;
                let transfers = state.active_transfers.read().await;
                let event = serde_json::json!({
                    "type": "heartbeat",
                    "timestamp": chrono::Utc::now().timestamp(),
                    "peer_count": peers.len(),
                    "active_transfers": transfers.len(),
                });

                if let Ok(msg) = serde_json::to_string(&event) {
                    if socket.send(Message::Text(msg)).await.is_err() {
                        break;
                    }
                }
            }
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        // Handle client commands
                        if text == "ping" {
                            let _ = socket.send(Message::Text("pong".into())).await;
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {}
                }
            }
        }
    }
}
