//! RFC 7807 problem details error responses.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};

/// RFC 7807 standard problem details JSON representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemDetails {
    #[serde(rename = "type")]
    pub problem_type: String,
    pub title: String,
    pub status: u16,
    pub detail: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
}

impl ProblemDetails {
    pub fn new(status: StatusCode, title: &str, detail: &str) -> Self {
        Self {
            problem_type: format!("https://airmesh.network/errors/{}", status.as_u16()),
            title: title.to_string(),
            status: status.as_u16(),
            detail: detail.to_string(),
            instance: None,
        }
    }
}

impl IntoResponse for ProblemDetails {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (
            status,
            [("content-type", "application/problem+json")],
            Json(self),
        )
            .into_response()
    }
}
