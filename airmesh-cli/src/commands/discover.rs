//! `discover` CLI command implementation.

use crate::output::print_output;
use airmesh_core::discovery::DiscoveredPeer;
use reqwest::Client;
use serde::Serialize;
use std::fmt;

/// Output wrapper for discovered peers.
#[derive(Serialize)]
pub struct DiscoverOutput {
    pub peers: Vec<DiscoveredPeer>,
}

impl fmt::Display for DiscoverOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.peers.is_empty() {
            return writeln!(f, "No peers discovered on local WiFi network yet.");
        }

        writeln!(
            f,
            "{:<20} {:<18} {:<15} {:<12}",
            "DEVICE NAME", "NODE ID", "ADDRESS", "METHOD"
        )?;
        writeln!(f, "{}", "-".repeat(68))?;

        for p in &self.peers {
            let short_id = if p.node_id.len() > 12 {
                &p.node_id[..12]
            } else {
                &p.node_id
            };
            writeln!(
                f,
                "{:<20} {:<18} {:<15} {:<12?}",
                p.device_name, short_id, p.address, p.method
            )?;
        }
        Ok(())
    }
}

/// Executes `airmesh-cli discover`.
pub async fn execute_discover(api_url: &str, as_json: bool) -> anyhow::Result<()> {
    let client = Client::new();
    let res = client
        .get(format!("{api_url}/api/peers"))
        .send()
        .await?;

    let peers: Vec<DiscoveredPeer> = res.json().await?;
    let out = DiscoverOutput { peers };
    print_output(&out, as_json);
    Ok(())
}
