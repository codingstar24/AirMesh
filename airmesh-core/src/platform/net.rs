//! Abstract Network trait with Native OS and Mock implementations.

use crate::protocol::error::{AirMeshError, Result};
use async_trait::async_trait;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

/// Network interface abstraction.
#[async_trait]
pub trait Network: Send + Sync {
    /// Detects current primary local IP address.
    async fn local_ip(&self) -> Result<IpAddr>;

    /// Tests if a local port is free.
    async fn is_port_available(&self, port: u16) -> bool;
}

/// Production network implementation.
pub struct NativeNetwork;

#[async_trait]
impl Network for NativeNetwork {
    async fn local_ip(&self) -> Result<IpAddr> {
        // Probe via UDP connect to internal broadcast target
        let socket = std::net::UdpSocket::bind("0.0.0.0:0")
            .map_err(|e| AirMeshError::Transport(e.to_string()))?;
        socket
            .connect("192.168.1.1:80")
            .or_else(|_| socket.connect("10.0.0.1:80"))
            .or_else(|_| socket.connect("172.16.0.1:80"))
            .map_err(|e| AirMeshError::Transport(format!("Cannot detect local IP: {e}")))?;

        let local_addr = socket
            .local_addr()
            .map_err(|e| AirMeshError::Transport(e.to_string()))?;
        Ok(local_addr.ip())
    }

    async fn is_port_available(&self, port: u16) -> bool {
        tokio::net::TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port))
            .await
            .is_ok()
    }
}

/// In-memory mock network for testing.
pub struct MockNetwork {
    simulated_ip: IpAddr,
}

impl MockNetwork {
    /// Creates a mock network with predetermined IP.
    pub fn new(ip: IpAddr) -> Self {
        Self { simulated_ip: ip }
    }
}

#[async_trait]
impl Network for MockNetwork {
    async fn local_ip(&self) -> Result<IpAddr> {
        Ok(self.simulated_ip)
    }

    async fn is_port_available(&self, _port: u16) -> bool {
        true
    }
}
