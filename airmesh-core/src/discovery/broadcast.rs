//! UDP broadcast discovery engine for local subnets.

use crate::discovery::types::{DiscoveredPeer, DiscoveryBeacon, DiscoveryMethod};
use crate::protocol::error::{AirMeshError, Result};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::sync::broadcast;
use tracing::{debug, error, info, warn};

/// UDP broadcast discovery runner.
pub struct UdpBroadcastDiscovery {
    local_node_id: String,
    device_name: String,
    service_port: u16,
    discovery_port: u16,
}

impl UdpBroadcastDiscovery {
    /// Creates a new broadcast discovery service instance.
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
        }
    }

    /// Spawns the discovery background tasks: beacon sender and beacon listener.
    pub async fn start(
        self: Arc<Self>,
        peer_sender: tokio::sync::mpsc::Sender<DiscoveredPeer>,
    ) -> Result<()> {
        let bind_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), self.discovery_port);
        let socket = UdpSocket::bind(bind_addr)
            .await
            .map_err(|e| AirMeshError::Transport(format!("Cannot bind discovery UDP socket: {e}")))?;

        socket
            .set_broadcast(true)
            .map_err(|e| AirMeshError::Transport(format!("Cannot enable UDP broadcast: {e}")))?;

        let socket = Arc::new(socket);
        let listener_socket = Arc::clone(&socket);
        let this_sender = Arc::clone(&self);

        // Spawn broadcast transmitter
        tokio::spawn(async move {
            let broadcast_target =
                SocketAddr::new(IpAddr::V4(Ipv4Addr::BROADCAST), this_sender.discovery_port);
            let mut interval = tokio::time::interval(Duration::from_secs(2));

            loop {
                interval.tick().await;
                let beacon = DiscoveryBeacon::new(
                    this_sender.local_node_id.clone(),
                    this_sender.device_name.clone(),
                    this_sender.service_port,
                );

                if let Ok(bytes) = serde_json::to_vec(&beacon) {
                    let _ = socket.send_to(&bytes, broadcast_target).await;
                }
            }
        });

        // Spawn listener
        tokio::spawn(async move {
            let mut buf = [0u8; 4096];
            loop {
                match listener_socket.recv_from(&mut buf).await {
                    Ok((len, remote_addr)) => {
                        if let Ok(beacon) = serde_json::from_slice::<DiscoveryBeacon>(&buf[..len]) {
                            if beacon.magic == DiscoveryBeacon::MAGIC
                                && beacon.node_id != self.local_node_id
                            {
                                let peer_addr = SocketAddr::new(remote_addr.ip(), beacon.port);
                                let peer = DiscoveredPeer {
                                    node_id: beacon.node_id,
                                    device_name: beacon.device_name,
                                    address: peer_addr,
                                    method: DiscoveryMethod::UdpBroadcast,
                                    last_seen: chrono::Utc::now().timestamp(),
                                };
                                let _ = peer_sender.send(peer).await;
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Error receiving discovery packet: {e}");
                        tokio::time::sleep(Duration::from_millis(500)).await;
                    }
                }
            }
        });

        Ok(())
    }
}
