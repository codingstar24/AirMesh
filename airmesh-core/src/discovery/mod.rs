//! Unified peer discovery engine uniting mDNS, UDP broadcast, QR and BLE.

pub mod ble;
pub mod broadcast;
pub mod mdns;
pub mod qr;
pub mod types;

pub use ble::BleDiscovery;
pub use broadcast::UdpBroadcastDiscovery;
pub use mdns::MdnsDiscovery;
pub use qr::QrBootstrapData;
pub use types::{DiscoveredPeer, DiscoveryBeacon, DiscoveryMethod};

use crate::protocol::error::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tracing::info;

/// High-level peer discovery manager maintaining live registry of local peers.
pub struct DiscoveryManager {
    local_node_id: String,
    device_name: String,
    service_port: u16,
    discovery_port: u16,
    peers: Arc<RwLock<HashMap<String, DiscoveredPeer>>>,
}

impl DiscoveryManager {
    /// Creates a new discovery manager.
    pub fn new(
        local_node_id: String,
        device_name: String,
        service_port: u16,
        discovery_port: u16,
    ) -> Self {
        Self {
            local_node_id,
            device_name,
            service_port,
            discovery_port,
            peers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Starts all discovery listeners concurrently.
    pub async fn start(self: Arc<Self>) -> Result<()> {
        let (tx, mut rx) = mpsc::channel::<DiscoveredPeer>(128);

        // 1. Start UDP Broadcast
        let broadcast = Arc::new(UdpBroadcastDiscovery::new(
            self.local_node_id.clone(),
            self.device_name.clone(),
            self.service_port,
            self.discovery_port,
        ));
        let tx_bcast = tx.clone();
        tokio::spawn(async move {
            let _ = broadcast.start(tx_bcast).await;
        });

        // 2. Start mDNS
        let mdns = Arc::new(MdnsDiscovery::new(
            self.local_node_id.clone(),
            self.device_name.clone(),
            self.service_port,
        ));
        let tx_mdns = tx.clone();
        tokio::spawn(async move {
            let _ = mdns.start(tx_mdns).await;
        });

        // 3. Central peer collector task
        let peer_table = Arc::clone(&self.peers);
        tokio::spawn(async move {
            while let Some(peer) = rx.recv().await {
                let mut map = peer_table.write().await;
                map.insert(peer.node_id.clone(), peer);
            }
        });

        info!(
            "Discovery manager started for node {} ({})",
            self.local_node_id, self.device_name
        );
        Ok(())
    }

    /// Injects a peer directly discovered via QR or manual input.
    pub async fn add_peer(&self, peer: DiscoveredPeer) {
        let mut map = self.peers.write().await;
        map.insert(peer.node_id.clone(), peer);
    }

    /// Returns list of all currently known online peers.
    pub async fn list_peers(&self) -> Vec<DiscoveredPeer> {
        let map = self.peers.read().await;
        map.values().cloned().collect()
    }
}
