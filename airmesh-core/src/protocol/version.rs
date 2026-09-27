//! Protocol version negotiation and capabilities.

use serde::{Deserialize, Serialize};
use std::fmt;

/// AirMesh protocol version identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ProtocolVersion {
    /// Major version (breaking changes)
    pub major: u16,
    /// Minor version (backwards compatible additions)
    pub minor: u16,
    /// Patch version (bug fixes)
    pub patch: u16,
}

impl ProtocolVersion {
    /// Current protocol version (1.0.0).
    pub const CURRENT: Self = Self {
        major: 1,
        minor: 0,
        patch: 0,
    };

    /// Creates a new protocol version.
    #[must_use]
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self { major, minor, patch }
    }

    /// Determines if a remote peer's version is compatible with this node.
    ///
    /// Compatibility rule: Same major version, and remote minor <= local minor.
    #[must_use]
    pub fn is_compatible(&self, other: &Self) -> bool {
        self.major == other.major
    }
}

impl Default for ProtocolVersion {
    fn default() -> Self {
        Self::CURRENT
    }
}

impl fmt::Display for ProtocolVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "v{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Bitflag set of features supported by a node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Capabilities {
    /// Supports QUIC transport
    pub quic: bool,
    /// Supports TCP + TLS transport
    pub tcp_tls: bool,
    /// Supports WebRTC DataChannel
    pub webrtc: bool,
    /// Supports LZ4 compression
    pub lz4: bool,
    /// Supports FastCDC delta sync
    pub fastcdc: bool,
    /// Supports range streaming
    pub streaming: bool,
}

impl Default for Capabilities {
    fn default() -> Self {
        Self {
            quic: true,
            tcp_tls: true,
            webrtc: true,
            lz4: true,
            fastcdc: true,
            streaming: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_compatibility() {
        let v1_0 = ProtocolVersion::new(1, 0, 0);
        let v1_1 = ProtocolVersion::new(1, 1, 0);
        let v2_0 = ProtocolVersion::new(2, 0, 0);

        assert!(v1_0.is_compatible(&v1_1));
        assert!(v1_1.is_compatible(&v1_0));
        assert!(!v1_0.is_compatible(&v2_0));
    }
}
