//! Per-peer access control list (ACL) and trust policies.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Peer trust relationship status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PeerTrustStatus {
    /// Peer is fully paired and trusted for all operations
    Trusted,
    /// Peer is permitted read-only browsing/streaming
    ReadOnly,
    /// Peer pairing request is pending confirmation
    Pending,
    /// Peer is explicitly blocked
    Blocked,
}

/// Entry in the ACL for a specific peer node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AclEntry {
    /// Public node identifier
    pub node_id: String,
    /// Trust authorization status
    pub status: PeerTrustStatus,
    /// Friendly alias assigned by local user
    pub alias: Option<String>,
    /// Unix timestamp when pairing was established
    pub paired_at: i64,
}

/// In-memory and persistent Access Control List manager.
#[derive(Debug, Default, Clone)]
pub struct AccessControlList {
    peers: HashMap<String, AclEntry>,
}

impl AccessControlList {
    /// Creates an empty ACL.
    pub fn new() -> Self {
        Self {
            peers: HashMap::new(),
        }
    }

    /// Sets or updates trust status for a peer.
    pub fn set_peer(&mut self, node_id: &str, status: PeerTrustStatus, alias: Option<String>) {
        self.peers.insert(
            node_id.to_string(),
            AclEntry {
                node_id: node_id.to_string(),
                status,
                alias,
                paired_at: chrono::Utc::now().timestamp(),
            },
        );
    }

    /// Retrieves status for a given peer node ID.
    #[must_use]
    pub fn get_status(&self, node_id: &str) -> PeerTrustStatus {
        self.peers
            .get(node_id)
            .map(|entry| entry.status)
            .unwrap_or(PeerTrustStatus::Pending)
    }

    /// Checks if a peer is allowed to read files.
    #[must_use]
    pub fn can_read(&self, node_id: &str) -> bool {
        matches!(
            self.get_status(node_id),
            PeerTrustStatus::Trusted | PeerTrustStatus::ReadOnly
        )
    }

    /// Checks if a peer is allowed to write or upload files.
    #[must_use]
    pub fn can_write(&self, node_id: &str) -> bool {
        matches!(self.get_status(node_id), PeerTrustStatus::Trusted)
    }

    /// Removes a peer from the ACL.
    pub fn remove_peer(&mut self, node_id: &str) -> bool {
        self.peers.remove(node_id).is_some()
    }

    /// Returns list of all entries in the ACL.
    #[must_use]
    pub fn entries(&self) -> Vec<AclEntry> {
        self.peers.values().cloned().collect()
    }
}
