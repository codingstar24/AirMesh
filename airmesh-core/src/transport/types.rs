//! Abstract connection and stream representations across QUIC, TCP, and WebRTC.

use crate::protocol::error::Result;
use crate::protocol::frames::Frame;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

/// Wire transport protocol type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransportType {
    /// QUIC over UDP (primary high-performance transport)
    Quic,
    /// TCP with TLS 1.3 framing (fallback for UDP-filtered networks)
    TcpTls,
    /// WebRTC DataChannel (browser-to-device and browser-to-browser)
    WebRtc,
}

/// Abstract bi-directional stream across peer connections.
#[async_trait]
pub trait StreamSession: Send + Sync {
    /// Sends a protocol frame over the stream.
    async fn send_frame(&mut self, frame: &Frame) -> Result<()>;

    /// Receives the next protocol frame from the stream.
    async fn recv_frame(&mut self) -> Result<Option<Frame>>;

    /// Closes the stream gracefully.
    async fn close(&mut self) -> Result<()>;
}

/// Connection handle to a remote peer.
#[async_trait]
pub trait PeerConnection: Send + Sync {
    /// Remote socket address.
    fn remote_addr(&self) -> SocketAddr;

    /// Transport mechanism utilized by this connection.
    fn transport_type(&self) -> TransportType;

    /// Opens a new bidirectional stream to the remote peer.
    async fn open_stream(&self) -> Result<Box<dyn StreamSession>>;

    /// Accepts an incoming bidirectional stream initiated by the remote peer.
    async fn accept_stream(&self) -> Result<Box<dyn StreamSession>>;
}
