//! Daemon configuration management and defaults.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Complete daemon runtime configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonConfig {
    /// Local HTTP/WebSocket API port (default 47823)
    pub http_port: u16,
    /// QUIC/TCP service transport port (default 47823)
    pub service_port: u16,
    /// UDP broadcast discovery port (default 47824)
    pub discovery_port: u16,
    /// Friendly device name
    pub device_name: String,
    /// Directory watched and served to the mesh
    pub shared_dir: PathBuf,
    /// Directory where downloaded files are saved
    pub download_dir: PathBuf,
    /// Path to SQLite index database
    pub database_path: PathBuf,
    /// Number of parallel streams for transfers (8-32)
    pub parallel_streams: usize,
}

impl Default for DaemonConfig {
    fn default() -> Self {
        let base_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let shared_dir = base_dir.join("shared");
        let download_dir = base_dir.join("downloads");
        let database_path = base_dir.join("airmesh.db");

        let device_name = format!(
            "Node-{}",
            &uuid::Uuid::new_v4().to_string()[..6]
        );

        Self {
            http_port: 47823,
            service_port: 47823,
            discovery_port: 47824,
            device_name,
            shared_dir,
            download_dir,
            database_path,
            parallel_streams: 16,
        }
    }
}
