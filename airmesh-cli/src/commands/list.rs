//! `list` CLI command implementation.

use crate::output::{format_bytes, print_output};
use airmesh_core::index::IndexedFile;
use reqwest::Client;
use serde::Serialize;
use std::fmt;

/// Output wrapper for file listing.
#[derive(Serialize)]
pub struct ListOutput {
    pub files: Vec<IndexedFile>,
}

impl fmt::Display for ListOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.files.is_empty() {
            return writeln!(f, "No files currently indexed in shared catalog.");
        }

        writeln!(
            f,
            "{:<36} {:<12} {:<24} {:<12}",
            "VIRTUAL PATH", "SIZE", "MIME TYPE", "CHUNKS"
        )?;
        writeln!(f, "{}", "-".repeat(88))?;

        for file in &self.files {
            writeln!(
                f,
                "{:<36} {:<12} {:<24} {:<12}",
                file.virtual_path,
                format_bytes(file.size),
                file.mime_type,
                file.chunk_count
            )?;
        }
        Ok(())
    }
}

/// Executes `airmesh-cli list`.
pub async fn execute_list(api_url: &str, as_json: bool) -> anyhow::Result<()> {
    let client = Client::new();
    let res = client
        .get(format!("{api_url}/api/files"))
        .send()
        .await?;

    let body: serde_json::Value = res.json().await?;
    let files: Vec<IndexedFile> = serde_json::from_value(body["files"].clone())
        .unwrap_or_default();

    let out = ListOutput { files };
    print_output(&out, as_json);
    Ok(())
}
