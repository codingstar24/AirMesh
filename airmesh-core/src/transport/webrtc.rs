//! WebRTC DataChannel transport abstraction for browser-only peers.

use crate::protocol::error::{AirMeshError, Result};
use crate::protocol::frames::Frame;
use crate::transport::types::{PeerConnection, StreamSession, TransportType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::sync::mpsc;

/// WebRTC signaling message exchange structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebRtcSignal {
    /// SDP Offer
    Offer {
        /// Session description string
        sdp: String,
    },
    /// SDP Answer
    Answer {
        /// Session description string
        sdp: String,
    },
    /// Trickle ICE candidate
    Candidate {
        /// Candidate description
        candidate: String,
        /// SDP MLIne Index
        sdp_m_line_index: Option<u16>,
    },
}

/// WebRTC simulated data-channel stream session.
pub struct WebRtcStreamSession {
    tx: mpsc::Sender<Frame>,
    rx: mpsc::Receiver<Frame>,
}

impl WebRtcStreamSession {
    /// Creates in-memory data channel stream session.
    pub fn new(tx: mpsc::Sender<Frame>, rx: mpsc::Receiver<Frame>) -> Self {
        Self { tx, rx }
    }
}

#[async_trait]
impl StreamSession for WebRtcStreamSession {
    async fn send_frame(&mut self, frame: &Frame) -> Result<()> {
        self.tx
            .send(frame.clone())
            .await
            .map_err(|e| AirMeshError::Transport(format!("DataChannel send failed: {e}")))?;
        Ok(())
    }

    async fn recv_frame(&mut self) -> Result<Option<Frame>> {
        Ok(self.rx.recv().await)
    }

    async fn close(&mut self) -> Result<()> {
        Ok(())
    }
}
