//! Integration test for multi-node local discovery.

use airmesh_core::discovery::{DiscoveredPeer, DiscoveryManager, DiscoveryMethod};
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::test]
async fn test_in_process_discovery_registry() {
    let dm = Arc::new(DiscoveryManager::new(
        "node_alpha_123".into(),
        "Alpha Node".into(),
        47823,
        47824,
    ));

    // Register a simulated peer
    let peer = DiscoveredPeer {
        node_id: "node_beta_456".into(),
        device_name: "Beta Node".into(),
        address: "192.168.1.102:47823".parse::<SocketAddr>().unwrap(),
        method: DiscoveryMethod::UdpBroadcast,
        last_seen: chrono::Utc::now().timestamp(),
    };

    dm.add_peer(peer.clone()).await;

    let peers = dm.list_peers().await;
    assert_eq!(peers.len(), 1);
    assert_eq!(peers[0].node_id, "node_beta_456");
    assert_eq!(peers[0].device_name, "Beta Node");
}
