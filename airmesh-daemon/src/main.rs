//! AirMesh Daemon binary entry point.

mod api;
mod config;
mod embedded_web;
mod state;

use airmesh_core::crypto::identity::NodeIdentity;
use airmesh_core::crypto::tls::TlsCertificateBundle;
use airmesh_core::index::IndexDatabase;
use airmesh_core::transport::quic::configure_quic_server;
use api::build_api_router;
use clap::Parser;
use config::DaemonConfig;
use embedded_web::serve_embedded_index;
use state::AppState;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// AirMesh Daemon Command Line Arguments
#[derive(Parser, Debug)]
#[command(name = "airmesh-daemon", author, version, about = "AirMesh P2P File Mesh Daemon", long_about = None)]
struct Args {
    /// HTTP/WebSocket port
    #[arg(short = 'p', long, default_value_t = 47823)]
    port: u16,

    /// UDP Discovery port
    #[arg(short = 'd', long, default_value_t = 47824)]
    discovery_port: u16,

    /// Shared directory path
    #[arg(short = 's', long)]
    shared: Option<PathBuf>,

    /// Friendly device name
    #[arg(short = 'n', long)]
    name: Option<String>,

    /// Database file path
    #[arg(long)]
    db: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let start_time = Instant::now();

    // Initialize structured logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "airmesh_daemon=info,airmesh_core=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting AirMesh Daemon v{}...", env!("CARGO_PKG_VERSION"));

    let args = Args::parse();
    let mut config = DaemonConfig::default();
    config.http_port = args.port;
    config.service_port = args.port;
    config.discovery_port = args.discovery_port;

    if let Some(shared) = args.shared {
        config.shared_dir = shared;
    }
    if let Some(name) = args.name {
        config.device_name = name;
    }
    if let Some(db_path) = args.db {
        config.database_path = db_path;
    }

    // Ensure shared and storage directories exist
    tokio::fs::create_dir_all(&config.shared_dir).await?;
    tokio::fs::create_dir_all(&config.download_dir).await?;

    // Load or generate identity keypair
    let identity = NodeIdentity::generate();
    info!("Local Node Identity: {}", identity.node_id());

    // Open SQLite FTS5 database
    let db = IndexDatabase::open(&config.database_path)?;
    info!("Database initialized at {:?}", config.database_path);

    // Build AppState
    let state = AppState::new(identity, config.clone(), db);

    // Initial cold filesystem scan
    let indexed_count = state.watcher.scan_and_index_all()?;
    info!("Pre-indexed {} files in shared directory", indexed_count);

    // Start background filesystem watcher
    let watcher_clone = Arc::clone(&state.watcher);
    tokio::spawn(async move {
        let _ = watcher_clone.start_live_watcher().await;
    });

    // Start discovery manager (mDNS + UDP Broadcast)
    let discovery_clone = Arc::clone(&state.discovery);
    discovery_clone.start().await?;

    // Build HTTP API router + embedded Web UI
    let app = build_api_router(state.clone())
        .fallback(serve_embedded_index);

    let bind_addr = SocketAddr::from(([0, 0, 0, 0], config.http_port));
    let listener = tokio::net::TcpListener::bind(bind_addr).await?;

    let cold_start_duration = start_time.elapsed();
    info!(
        "AirMesh Daemon running on http://127.0.0.1:{} (Cold start completed in {:.2?})",
        config.http_port, cold_start_duration
    );

    // Run web & API server until signal
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("AirMesh Daemon gracefully stopped.");
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C signal handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
