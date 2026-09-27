//! QUIC high-throughput transport engine utilizing Quinn.

use crate::crypto::tls::TlsCertificateBundle;
use crate::protocol::codec::FrameCodec;
use crate::protocol::error::{AirMeshError, Result};
use crate::protocol::frames::Frame;
use crate::transport::types::{PeerConnection, StreamSession, TransportType};
use async_trait::async_trait;
use bytes::BytesMut;
use quinn::{Connection, Endpoint, RecvStream, SendStream};
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// ALPN protocol tag for AirMesh QUIC connections
pub const ALPN_AIRMESH_QUIC: &[u8] = b"airmesh-v1";

/// Active QUIC stream session.
pub struct QuicStreamSession {
    send: SendStream,
    recv: RecvStream,
    read_buf: BytesMut,
}

impl QuicStreamSession {
    /// Wraps Quinn send and receive halves.
    pub fn new(send: SendStream, recv: RecvStream) -> Self {
        Self {
            send,
            recv,
            read_buf: BytesMut::with_capacity(64 * 1024),
        }
    }
}

#[async_trait]
impl StreamSession for QuicStreamSession {
    async fn send_frame(&mut self, frame: &Frame) -> Result<()> {
        let mut buf = BytesMut::new();
        FrameCodec::encode(frame, &mut buf)?;
        self.send
            .write_all(&buf)
            .await
            .map_err(|e| AirMeshError::Transport(format!("QUIC write error: {e}")))?;
        Ok(())
    }

    async fn recv_frame(&mut self) -> Result<Option<Frame>> {
        loop {
            if let Some(frame) = FrameCodec::decode(&mut self.read_buf)? {
                return Ok(Some(frame));
            }

            let mut chunk = [0u8; 16384];
            match self.recv.read(&mut chunk).await {
                Ok(Some(n)) if n > 0 => {
                    self.read_buf.extend_from_slice(&chunk[..n]);
                }
                Ok(_) => {
                    // EOF reached
                    if self.read_buf.is_empty() {
                        return Ok(None);
                    }
                    return FrameCodec::decode(&mut self.read_buf);
                }
                Err(e) => return Err(AirMeshError::Transport(format!("QUIC read error: {e}"))),
            }
        }
    }

    async fn close(&mut self) -> Result<()> {
        let _ = self.send.finish();
        Ok(())
    }
}

/// QUIC connection wrapper.
pub struct QuicPeerConnection {
    conn: Connection,
}

impl QuicPeerConnection {
    /// Creates wrapper around active Quinn Connection.
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl PeerConnection for QuicPeerConnection {
    fn remote_addr(&self) -> SocketAddr {
        self.conn.remote_address()
    }

    fn transport_type(&self) -> TransportType {
        TransportType::Quic
    }

    async fn open_stream(&self) -> Result<Box<dyn StreamSession>> {
        let (send, recv) = self
            .conn
            .open_bi()
            .await
            .map_err(|e| AirMeshError::Transport(format!("Failed to open QUIC stream: {e}")))?;
        Ok(Box::new(QuicStreamSession::new(send, recv)))
    }

    async fn accept_stream(&self) -> Result<Box<dyn StreamSession>> {
        let (send, recv) = self
            .conn
            .accept_bi()
            .await
            .map_err(|e| AirMeshError::Transport(format!("Failed to accept QUIC stream: {e}")))?;
        Ok(Box::new(QuicStreamSession::new(send, recv)))
    }
}

/// Builds Quinn server configuration with self-signed certificate.
pub fn configure_quic_server(bundle: &TlsCertificateBundle) -> Result<quinn::ServerConfig> {
    let certs = bundle
        .cert_chain_der
        .iter()
        .map(|c| CertificateDer::from(c.clone()))
        .collect();
    let key = PrivateKeyDer::try_from(bundle.private_key_der.clone())
        .map_err(|_| AirMeshError::Crypto("Invalid private key format for QUIC".into()))?;

    let mut server_crypto = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .map_err(|e| AirMeshError::Crypto(format!("Failed building TLS server config: {e}")))?;

    server_crypto.alpn_protocols = vec![ALPN_AIRMESH_QUIC.to_vec()];

    let server_config = quinn::ServerConfig::with_crypto(Arc::new(
        quinn::crypto::rustls::QuicServerConfig::try_from(server_crypto)
            .map_err(|e| AirMeshError::Crypto(format!("QUIC crypto config error: {e}")))?,
    ));

    Ok(server_config)
}
