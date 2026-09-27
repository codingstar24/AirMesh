//! Multicast DNS (mDNS) discovery provider for AirMesh.

use crate::discovery::types::{DiscoveredPeer, DiscoveryBeacon, DiscoveryMethod};
use crate::protocol::error::{AirMeshError, Result};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::UdpSocket;
use tracing::{debug, warn};

/// Multicast group IPv4 for mDNS (224.0.0.251)
pub const MDNS_IPV4_MULTICAST: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 251);
/// Standard mDNS port
pub const MDNS_PORT: u16 = 5353;

/// mDNS peer discovery coordinator.
pub struct MdnsDiscovery {
    local_node_id: String,
    device_name: String,
    service_port: u16,
}

impl MdnsDiscovery {
    /// Creates a new mDNS discovery instance.
    pub fn new(local_node_id: String, device_name: String, service_port: u16) -> Self {
        Self {
            local_node_id,
            device_name,
            service_port,
        }
    }

    /// Starts mDNS announcer and listener loops.
    pub async fn start(
        self: Arc<Self>,
        peer_sender: tokio::sync::mpsc::Sender<DiscoveredPeer>,
    ) -> Result<()> {
        let bind_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), MDNS_PORT);

        // Standard multicast bind with reuse_address / reuse_port if supported
        let socket = match UdpSocket::bind(bind_addr).await {
            Ok(s) => s,
            Err(e) => {
                warn!("Cannot bind standard mDNS port 5353 ({e}), falling back to OS allocated port");
                UdpSocket::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0))
                    .await
                    .map_err(|e| AirMeshError::Transport(format!("Cannot bind mDNS socket: {e}")))?
            }
        };

        let _ = socket.join_multicast_v4(MDNS_IPV4_MULTICAST, Ipv4Addr::UNSPECIFIED);
        let socket = Arc::new(socket);
        let listener_socket = Arc::clone(&socket);
        let this_sender = Arc::clone(&self);

        // Transmitter task
        tokio::spawn(async move {
            let mcast_target = SocketAddr::new(IpAddr::V4(MDNS_IPV4_MULTICAST), MDNS_PORT);
            let mut interval = tokio::time::interval(Duration::from_secs(3));

            loop {
                interval.tick().await;
                let beacon = DiscoveryBeacon::new(
                    this_sender.local_node_id.clone(),
                    this_sender.device_name.clone(),
                    this_sender.service_port,
                );

                if let Ok(bytes) = serde_json::to_vec(&beacon) {
                    let _ = socket.send_to(&bytes, mcast_target).await;
                }
            }
        });

        // Receiver task
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
                                    method: DiscoveryMethod::Mdns,
                                    last_seen: chrono::Utc::now().timestamp(),
                                };
                                let _ = peer_sender.send(peer).await;
                            }
                        }
                    }
                    Err(e) => {
                        warn!("mDNS receive error: {e}");
                        tokio::time::sleep(Duration::from_millis(500)).await;
                    }
                }
            }
        });

        Ok(())
    }
}
