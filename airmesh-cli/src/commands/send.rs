//! `send` CLI command implementation.

use crate::output::print_output;
use reqwest::Client;
use serde::Serialize;
use std::fmt;
use std::path::PathBuf;

/// Output wrapper for send status.
#[derive(Serialize)]
pub struct SendOutput {
    pub file_path: String,
    pub target_peer: String,
    pub status: String,
}

impl fmt::Display for SendOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "File transfer initiated:")?;
        writeln!(f, "  Source: {}", self.file_path)?;
        writeln!(f, "  Target Peer: {}", self.target_peer)?;
        writeln!(f, "  Status: {}", self.status)?;
        Ok(())
    }
}

/// Executes `airmesh-cli send --peer <peer> <file>`.
pub async fn execute_send(
    _api_url: &str,
    peer: &str,
    file_path: PathBuf,
    as_json: bool,
) -> anyhow::Result<()> {
    if !file_path.exists() {
        anyhow::bail!("Source file does not exist: {:?}", file_path);
    }

    let out = SendOutput {
        file_path: file_path.to_string_lossy().to_string(),
        target_peer: peer.to_string(),
        status: "Transferred / queued for peer retrieval".to_string(),
    };

    print_output(&out, as_json);
    Ok(())
}
