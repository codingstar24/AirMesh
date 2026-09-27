//! `search` CLI command implementation.

use crate::output::{format_bytes, print_output};
use airmesh_core::index::SearchResult;
use reqwest::Client;
use serde::Serialize;
use std::fmt;

/// Output wrapper for search hits.
#[derive(Serialize)]
pub struct SearchOutput {
    pub query: String,
    pub results: Vec<SearchResult>,
}

impl fmt::Display for SearchOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.results.is_empty() {
            return writeln!(f, "No results matching '{}'.", self.query);
        }

        writeln!(
            f,
            "{:<36} {:<12} {:<24} {:<8}",
            "VIRTUAL PATH", "SIZE", "MIME TYPE", "SCORE"
        )?;
        writeln!(f, "{}", "-".repeat(84))?;

        for hit in &self.results {
            writeln!(
                f,
                "{:<36} {:<12} {:<24} {:<8}",
                hit.virtual_path,
                format_bytes(hit.size),
                hit.mime_type,
                hit.score
            )?;
        }
        Ok(())
    }
}

/// Executes `airmesh-cli search <query>`.
pub async fn execute_search(
    api_url: &str,
    query: &str,
    mime: Option<String>,
    as_json: bool,
) -> anyhow::Result<()> {
    let client = Client::new();
    let mut req = client.get(format!("{api_url}/api/search")).query(&[("q", query)]);

    if let Some(m) = mime {
        req = req.query(&[("mime", m)]);
    }

    let res = req.send().await?;
    let body: serde_json::Value = res.json().await?;
    let results: Vec<SearchResult> = serde_json::from_value(body["results"].clone())
        .unwrap_or_default();

    let out = SearchOutput {
        query: query.to_string(),
        results,
    };
    print_output(&out, as_json);
    Ok(())
}
