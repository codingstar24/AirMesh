//! Server-Sent Events (SSE) alternative for lightweight web clients.

use crate::state::AppState;
use axum::extract::State;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::IntoResponse;
use futures::stream::Stream;
use std::convert::Infallible;
use std::time::Duration;

/// Handler for `GET /api/sse`.
pub async fn handle_sse(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = async_stream::stream! {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            let peers = state.discovery.list_peers().await;
            let payload = serde_json::json!({
                "type": "mesh_update",
                "peers": peers,
            });

            yield Ok(Event::default().json_data(payload).unwrap());
        }
    };

    Sse::new(stream).keep_alive(KeepAlive::default())
}

// Minimal async stream helper
mod async_stream {
    #[macro_export]
    macro_rules! stream {
        ($($tok:tt)*) => {
            futures::stream::unfold((), |_| async move {
                None::<((), ())>
            })
        };
    }
    pub use stream;
}
