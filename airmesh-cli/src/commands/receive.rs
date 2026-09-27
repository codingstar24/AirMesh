//! `receive` CLI command implementation.

use crate::output::print_output;
use reqwest::Client;
use serde::Serialize;
use std::fmt;
use std::path::PathBuf;

/// Output wrapper for receive status.
#[derive(Serialize)]
pub struct ReceiveOutput {
    pub file_id: String,
    pub destination: String,
    pub bytes_downloaded: u64,
}

impl fmt::Display for ReceiveOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "File download finished:")?;
        writeln!(f, "  File ID: {}", self.file_id)?;
        writeln!(f, "  Saved to: {}", self.destination)?;
        writeln!(f, "  Size: {} bytes", self.bytes_downloaded)?;
        Ok(())
    }
}

/// Executes `airmesh-cli receive <file_id> <output_path>`.
pub async fn execute_receive(
    api_url: &str,
    file_id: &str,
    output_path: PathBuf,
    as_json: bool,
) -> anyhow::Result<()> {
    let client = Client::new();
    let res = client
        .get(format!("{api_url}/api/stream/{file_id}"))
        .send()
        .await?;

    let bytes = res.bytes().await?;
    let len = bytes.len() as u64;

    tokio::fs::write(&output_path, &bytes).await?;

    let out = ReceiveOutput {
        file_id: file_id.to_string(),
        destination: output_path.to_string_lossy().to_string(),
        bytes_downloaded: len,
    };

    print_output(&out, as_json);
    Ok(())
}
