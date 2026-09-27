//! Metrics collection and Prometheus format renderer.

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

/// Global AirMesh runtime performance metrics.
pub struct MetricsCollector {
    pub bytes_sent_total: AtomicU64,
    pub bytes_received_total: AtomicU64,
    pub chunks_sent_total: AtomicU64,
    pub chunks_received_total: AtomicU64,
    pub active_transfers: AtomicUsize,
    pub connected_peers: AtomicUsize,
    pub search_queries_total: AtomicU64,
}

impl MetricsCollector {
    /// Creates a new metrics collector.
    pub const fn new() -> Self {
        Self {
            bytes_sent_total: AtomicU64::new(0),
            bytes_received_total: AtomicU64::new(0),
            chunks_sent_total: AtomicU64::new(0),
            chunks_received_total: AtomicU64::new(0),
            active_transfers: AtomicUsize::new(0),
            connected_peers: AtomicUsize::new(0),
            search_queries_total: AtomicU64::new(0),
        }
    }

    /// Renders all metrics in Prometheus text exposition format.
    pub fn render_prometheus(&self) -> String {
        format!(
            "# HELP airmesh_bytes_sent_total Total bytes transmitted across mesh\n\
             # TYPE airmesh_bytes_sent_total counter\n\
             airmesh_bytes_sent_total {}\n\
             # HELP airmesh_bytes_received_total Total bytes received across mesh\n\
             # TYPE airmesh_bytes_received_total counter\n\
             airmesh_bytes_received_total {}\n\
             # HELP airmesh_chunks_sent_total Total chunks sent\n\
             # TYPE airmesh_chunks_sent_total counter\n\
             airmesh_chunks_sent_total {}\n\
             # HELP airmesh_chunks_received_total Total chunks received\n\
             # TYPE airmesh_chunks_received_total counter\n\
             airmesh_chunks_received_total {}\n\
             # HELP airmesh_active_transfers Current in-flight transfers\n\
             # TYPE airmesh_active_transfers gauge\n\
             airmesh_active_transfers {}\n\
             # HELP airmesh_connected_peers Online discovered peers\n\
             # TYPE airmesh_connected_peers gauge\n\
             airmesh_connected_peers {}\n\
             # HELP airmesh_search_queries_total Total search queries served\n\
             # TYPE airmesh_search_queries_total counter\n\
             airmesh_search_queries_total {}\n",
            self.bytes_sent_total.load(Ordering::Relaxed),
            self.bytes_received_total.load(Ordering::Relaxed),
            self.chunks_sent_total.load(Ordering::Relaxed),
            self.chunks_received_total.load(Ordering::Relaxed),
            self.active_transfers.load(Ordering::Relaxed),
            self.connected_peers.load(Ordering::Relaxed),
            self.search_queries_total.load(Ordering::Relaxed),
        )
    }
}
