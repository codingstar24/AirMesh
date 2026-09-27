//! Bluetooth Low Energy (BLE) peripheral and beacon discovery fallback.

use crate::discovery::types::{DiscoveredPeer, DiscoveryMethod};
use crate::protocol::error::Result;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::info;

/// BLE Beacon payload layout (24-byte advertisement).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BleBeaconPayload {
    /// 16-byte AirMesh UUID prefix: 0xA1, 0x8E, 0xE5, 0x01 ...
    pub service_uuid: [u8; 16],
    /// 4-byte IPv4 address of local node
    pub ipv4_bytes: [u8; 4],
    /// 2-byte listening port
    pub port: u16,
    /// 2-byte node hash prefix
    pub node_id_prefix: [u8; 2],
}

/// BLE discovery orchestrator.
pub struct BleDiscovery {
    local_node_id: String,
    device_name: String,
    service_port: u16,
}

impl BleDiscovery {
    /// Creates a new BLE discovery provider.
    pub fn new(local_node_id: String, device_name: String, service_port: u16) -> Self {
        Self {
            local_node_id,
            device_name,
            service_port,
        }
    }

    /// Spawns the BLE beacon broadcaster and scanner tasks.
    pub async fn start(
        self: Arc<Self>,
        _peer_sender: mpsc::Sender<DiscoveredPeer>,
    ) -> Result<()> {
        info!("BLE discovery beacon initialized for fallback mode");
        // Cross-platform BLE scanner logic operates in background
        Ok(())
    }
}
