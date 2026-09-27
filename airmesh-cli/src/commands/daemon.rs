//! `daemon` management and status inspection command.

use crate::output::print_output;
use reqwest::Client;
use serde::Serialize;
use std::fmt;

/// Output wrapper for daemon status.
#[derive(Serialize)]
pub struct DaemonStatusOutput {
    pub status: String,
    pub node_id: String,
    pub device_name: String,
    pub version: String,
    pub peer_count: usize,
}

impl fmt::Display for DaemonStatusOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "AirMesh Daemon Status: {}", self.status)?;
        writeln!(f, "  Version:     {}", self.version)?;
        writeln!(f, "  Node ID:     {}", self.node_id)?;
        writeln!(f, "  Device Name: {}", self.device_name)?;
        writeln!(f, "  Peers:       {}", self.peer_count)?;
        Ok(())
    }
}

/// Executes `airmesh-cli daemon status`.
pub async fn execute_daemon_status(api_url: &str, as_json: bool) -> anyhow::Result<()> {
    let client = Client::new();
    let res = client
        .get(format!("{api_url}/api/health"))
        .send()
        .await?;

    let body: serde_json::Value = res.json().await?;
    let out = DaemonStatusOutput {
        status: body["status"].as_str().unwrap_or("unknown").to_string(),
        node_id: body["node_id"].as_str().unwrap_or("").to_string(),
        device_name: body["device_name"].as_str().unwrap_or("").to_string(),
        version: body["version"].as_str().unwrap_or("").to_string(),
        peer_count: body["peer_count"].as_u64().unwrap_or(0) as usize,
    };

    print_output(&out, as_json);
    Ok(())
}
