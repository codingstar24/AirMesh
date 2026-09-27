//! Wire protocol and domain error representations.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Protocol error code sent across the wire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u16)]
pub enum ErrorCode {
    /// Generic or unexpected internal error
    InternalError = 1000,
    /// Protocol version mismatch or unsupported
    IncompatibleVersion = 1001,
    /// Handshake or cryptographic authentication failure
    AuthenticationFailed = 1002,
    /// Peer rejected pairing or access unauthorized
    AccessDenied = 1003,
    /// Requested file or chunk was not found
    NotFound = 1004,
    /// BLAKE3 checksum verification failed
    ChecksumMismatch = 1005,
    /// Target path attempts to escape sandbox (directory traversal)
    InvalidPath = 1006,
    /// Peer rate limit exceeded
    RateLimited = 1007,
    /// Connection or transfer timed out
    Timeout = 1008,
    /// Storage disk full or quota exceeded
    StorageFull = 1009,
    /// Corrupt frame or serialization failure
    MalformedFrame = 1010,
    /// Transfer was cancelled by user
    TransferCancelled = 1011,
}

/// Core AirMesh library error.
#[derive(Debug, Error)]
pub enum AirMeshError {
    /// IO errors
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization/deserialization failure
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Cryptographic failure
    #[error("Cryptographic error: {0}")]
    Crypto(String),

    /// Protocol error code received from remote peer
    #[error("Protocol error [{code:?}]: {message}")]
    Protocol {
        /// Wire error code
        code: ErrorCode,
        /// Detail message
        message: String,
    },

    /// Database errors
    #[error("Database error: {0}")]
    Database(String),

    /// Network or transport error
    #[error("Transport error: {0}")]
    Transport(String),

    /// Path traversal security violation
    #[error("Security violation: path traversal detected: {0}")]
    PathTraversal(String),

    /// Peer not found or offline
    #[error("Peer not found: {0}")]
    PeerNotFound(String),

    /// Transfer error
    #[error("Transfer failure: {0}")]
    Transfer(String),
}

/// Convenience result type for AirMesh operations.
pub type Result<T> = std::result::Result<T, AirMeshError>;
