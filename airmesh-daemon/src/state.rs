//! Global in-memory state shared across HTTP API, discovery, and transfers.

use crate::config::DaemonConfig;
use airmesh_core::crypto::{AccessControlList, NodeIdentity};
use airmesh_core::discovery::DiscoveryManager;
use airmesh_core::index::{FolderWatcher, IndexDatabase};
use airmesh_core::transfer::engine::TransferProgress;
use airmesh_core::util::MetricsCollector;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Application state container.
#[derive(Clone)]
pub struct AppState {
    pub identity: Arc<NodeIdentity>,
    pub config: Arc<DaemonConfig>,
    pub db: Arc<IndexDatabase>,
    pub discovery: Arc<DiscoveryManager>,
    pub acl: Arc<RwLock<AccessControlList>>,
    pub metrics: Arc<MetricsCollector>,
    pub watcher: Arc<FolderWatcher>,
    pub active_transfers: Arc<RwLock<HashMap<String, TransferProgress>>>,
}

impl AppState {
    /// Creates a fully initialized app state instance.
    pub fn new(
        identity: NodeIdentity,
        config: DaemonConfig,
        db: IndexDatabase,
    ) -> Self {
        let db = Arc::new(db);
        let watcher = Arc::new(FolderWatcher::new(&config.shared_dir, Arc::clone(&db)));
        let discovery = Arc::new(DiscoveryManager::new(
            identity.node_id().to_string(),
            config.device_name.clone(),
            config.service_port,
            config.discovery_port,
        ));

        Self {
            identity: Arc::new(identity),
            config: Arc::new(config),
            db,
            discovery,
            acl: Arc::new(RwLock::new(AccessControlList::new())),
            metrics: Arc::new(MetricsCollector::new()),
            watcher,
            active_transfers: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
