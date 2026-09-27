//! Embedded Web UI server handlers.

pub mod assets;

use assets::EMBEDDED_INDEX_HTML;
use axum::http::header::CONTENT_TYPE;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};

/// Serves the embedded HTML index page.
pub async fn serve_embedded_index() -> impl IntoResponse {
    Html(EMBEDDED_INDEX_HTML)
}
