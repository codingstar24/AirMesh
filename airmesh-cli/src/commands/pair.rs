//! `pair` CLI command implementation.

use crate::output::print_output;
use airmesh_core::crypto::identity::hex;
use airmesh_core::crypto::pake::PakePairing;
use reqwest::Client;
use serde::Serialize;
use std::fmt;

/// Output wrapper for pairing status.
#[derive(Serialize)]
pub struct PairOutput {
    pub peer_node_id: String,
    pub status: String,
    pub token: String,
}

impl fmt::Display for PairOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Device Pairing Status: {}", self.status)?;
        writeln!(f, "  Peer: {}", self.peer_node_id)?;
        writeln!(f, "  Confirmation Token: {}", self.token)?;
        Ok(())
    }
}

/// Executes `airmesh-cli pair --peer <peer> --pin <code>`.
pub async fn execute_pair(
    api_url: &str,
    peer: &str,
    pin: &str,
    as_json: bool,
) -> anyhow::Result<()> {
    let client = Client::new();
    let salt = [0u8; 16];
    let payload = serde_json::json!({
        "peer_node_id": peer,
        "pin_code": pin,
        "salt_hex": hex::encode(&salt),
    });

    let res = client
        .post(format!("{api_url}/api/peers/pair"))
        .json(&payload)
        .send()
        .await?;

    let body: serde_json::Value = res.json().await?;
    let out = PairOutput {
        peer_node_id: peer.to_string(),
        status: body["status"].as_str().unwrap_or("unknown").to_string(),
        token: body["token"].as_str().unwrap_or("").to_string(),
    };

    print_output(&out, as_json);
    Ok(())
}
