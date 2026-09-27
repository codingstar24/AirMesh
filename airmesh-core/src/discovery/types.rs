//! Types and structures for local network peer discovery.

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

/// Channel through which peer discovery was achieved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DiscoveryMethod {
    /// Multicast DNS (_airmesh._udp.local)
    Mdns,
    /// Direct UDP broadcast (port 47824)
    UdpBroadcast,
    /// Visual QR code optical bootstrap
    QrCode,
    /// Bluetooth Low Energy beacon
    Ble,
    /// Manual direct IP entry
    Manual,
}

/// Representation of a discovered network peer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscoveredPeer {
    /// Remote node public identity (hex)
    pub node_id: String,
    /// Friendly device name
    pub device_name: String,
    /// Network socket address (IP + Port)
    pub address: SocketAddr,
    /// Primary discovery mechanism
    pub method: DiscoveryMethod,
    /// Unix timestamp when last heartbeat was received
    pub last_seen: i64,
}

/// Wire payload for broadcast discovery beacons.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryBeacon {
    /// Magic identifier "AIRMESH_BEACON_V1"
    pub magic: String,
    /// Node public ID
    pub node_id: String,
    /// Friendly device name
    pub device_name: String,
    /// QUIC/TCP service port
    pub port: u16,
    /// Node timestamp
    pub timestamp: i64,
}

impl DiscoveryBeacon {
    /// Magic string constant
    pub const MAGIC: &'static str = "AIRMESH_BEACON_V1";

    /// Creates a new announcement beacon.
    pub fn new(node_id: String, device_name: String, port: u16) -> Self {
        Self {
            magic: Self::MAGIC.to_string(),
            node_id,
            device_name,
            port,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
}
