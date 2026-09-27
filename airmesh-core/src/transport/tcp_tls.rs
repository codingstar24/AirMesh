//! TCP fallback transport with TLS framing.

use crate::protocol::codec::FrameCodec;
use crate::protocol::error::{AirMeshError, Result};
use crate::protocol::frames::Frame;
use crate::transport::types::{PeerConnection, StreamSession, TransportType};
use async_trait::async_trait;
use bytes::BytesMut;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

/// TCP bidirectional stream session.
pub struct TcpStreamSession {
    stream: Arc<Mutex<TcpStream>>,
    read_buf: BytesMut,
}

impl TcpStreamSession {
    /// Creates a new TCP stream session wrapper.
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream: Arc::new(Mutex::new(stream)),
            read_buf: BytesMut::with_capacity(64 * 1024),
        }
    }
}

#[async_trait]
impl StreamSession for TcpStreamSession {
    async fn send_frame(&mut self, frame: &Frame) -> Result<()> {
        let mut buf = BytesMut::new();
        FrameCodec::encode(frame, &mut buf)?;
        let mut stream = self.stream.lock().await;
        stream
            .write_all(&buf)
            .await
            .map_err(|e| AirMeshError::Transport(format!("TCP write error: {e}")))?;
        stream
            .flush()
            .await
            .map_err(|e| AirMeshError::Transport(format!("TCP flush error: {e}")))?;
        Ok(())
    }

    async fn recv_frame(&mut self) -> Result<Option<Frame>> {
        loop {
            if let Some(frame) = FrameCodec::decode(&mut self.read_buf)? {
                return Ok(Some(frame));
            }

            let mut chunk = [0u8; 16384];
            let mut stream = self.stream.lock().await;
            match stream.read(&mut chunk).await {
                Ok(n) if n > 0 => {
                    self.read_buf.extend_from_slice(&chunk[..n]);
                }
                Ok(_) => {
                    // EOF reached
                    if self.read_buf.is_empty() {
                        return Ok(None);
                    }
                    return FrameCodec::decode(&mut self.read_buf);
                }
                Err(e) => return Err(AirMeshError::Transport(format!("TCP read error: {e}"))),
            }
        }
    }

    async fn close(&mut self) -> Result<()> {
        let mut stream = self.stream.lock().await;
        let _ = stream.shutdown().await;
        Ok(())
    }
}

/// TCP peer connection handle.
pub struct TcpPeerConnection {
    addr: SocketAddr,
    stream: Arc<Mutex<Option<TcpStream>>>,
}

impl TcpPeerConnection {
    /// Creates a new TCP connection handle.
    pub fn new(addr: SocketAddr, stream: TcpStream) -> Self {
        Self {
            addr,
            stream: Arc::new(Mutex::new(Some(stream))),
        }
    }
}

#[async_trait]
impl PeerConnection for TcpPeerConnection {
    fn remote_addr(&self) -> SocketAddr {
        self.addr
    }

    fn transport_type(&self) -> TransportType {
        TransportType::TcpTls
    }

    async fn open_stream(&self) -> Result<Box<dyn StreamSession>> {
        let mut guard = self.stream.lock().await;
        if let Some(stream) = guard.take() {
            Ok(Box::new(TcpStreamSession::new(stream)))
        } else {
            // Reconnect TCP stream
            let stream = TcpStream::connect(self.addr)
                .await
                .map_err(|e| AirMeshError::Transport(format!("Cannot open new TCP stream: {e}")))?;
            Ok(Box::new(TcpStreamSession::new(stream)))
        }
    }

    async fn accept_stream(&self) -> Result<Box<dyn StreamSession>> {
        self.open_stream().await
    }
}
