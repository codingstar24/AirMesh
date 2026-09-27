//! Wire protocol message frames and payload envelopes.

use crate::protocol::error::ErrorCode;
use crate::protocol::version::{Capabilities, ProtocolVersion};
use bytes::Bytes;
use serde::{Deserialize, Serialize};

/// Magic bytes header prefix for all AirMesh wire frames: "AMESH\x01"
pub const FRAME_MAGIC: [u8; 6] = [b'A', b'M', b'E', b'S', b'H', 0x01];

/// Top-level frame envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Frame {
    /// Initial handshake request from initiator
    HandshakeInit(HandshakeInitPayload),
    /// Handshake response from responder
    HandshakeResponse(HandshakeResponsePayload),
    /// Handshake finalization
    HandshakeAck,

    /// Liveness ping
    Ping {
        /// Sequence nonce
        nonce: u64,
    },
    /// Liveness pong
    Pong {
        /// Mirrored nonce
        nonce: u64,
    },

    /// Request file manifest / catalog
    ManifestRequest {
        /// Optional search filter or specific file ID
        file_id: Option<String>,
    },
    /// Response with file manifest
    ManifestResponse(ManifestResponsePayload),

    /// Request specific chunk of a file
    ChunkRequest(ChunkRequestPayload),
    /// Response carrying chunk data
    ChunkResponse(ChunkResponsePayload),

    /// Streaming HTTP range request for real-time media
    StreamRequest(StreamRequestPayload),
    /// Streaming HTTP range data slice
    StreamResponse(StreamResponsePayload),

    /// Protocol or domain error frame
    Error {
        /// Error status code
        code: ErrorCode,
        /// Human-readable explanation
        message: String,
    },
}

/// Payload for handshake initialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeInitPayload {
    /// Client wire protocol version
    pub version: ProtocolVersion,
    /// Client advertised capabilities
    pub capabilities: Capabilities,
    /// Initiator's public node identity (Ed25519 public key hex or bytes)
    pub node_id: String,
    /// Ephemeral public key for Noise protocol (32 bytes)
    pub ephemeral_pubkey: [u8; 32],
    /// Human-readable friendly device name
    pub device_name: String,
}

/// Payload for handshake response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeResponsePayload {
    /// Server wire protocol version
    pub version: ProtocolVersion,
    /// Negotiated capabilities
    pub capabilities: Capabilities,
    /// Responder's public node identity
    pub node_id: String,
    /// Responder's ephemeral public key
    pub ephemeral_pubkey: [u8; 32],
    /// Human-readable friendly device name
    pub device_name: String,
}

/// Metadata describing a shared file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileMetadata {
    /// Unique content-addressed identifier (BLAKE3 hex of full content)
    pub id: String,
    /// Relative sanitized file path
    pub relative_path: String,
    /// File size in bytes
    pub size: u64,
    /// Content MIME type
    pub mime_type: String,
    /// Unix timestamp of last modification
    pub modified_at: i64,
    /// Total number of 1 MiB chunks
    pub chunk_count: u32,
    /// BLAKE3 root hash in hex
    pub root_hash: String,
}

/// Payload for manifest responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestResponsePayload {
    /// List of file entries available from this peer
    pub files: Vec<FileMetadata>,
}

/// Payload requesting a chunk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkRequestPayload {
    /// File ID being fetched
    pub file_id: String,
    /// 0-indexed chunk number
    pub chunk_index: u32,
}

/// Payload containing chunk data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkResponsePayload {
    /// Target file ID
    pub file_id: String,
    /// 0-indexed chunk number
    pub chunk_index: u32,
    /// BLAKE3 checksum of the chunk payload
    pub blake3_hash: [u8; 32],
    /// True if data payload is LZ4 compressed
    pub is_compressed: bool,
    /// Raw or compressed chunk bytes
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}

/// Streaming range request payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamRequestPayload {
    /// Target file ID
    pub file_id: String,
    /// Byte start offset
    pub start_offset: u64,
    /// Number of bytes requested
    pub length: u32,
}

/// Streaming range response payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamResponsePayload {
    /// Target file ID
    pub file_id: String,
    /// Byte start offset
    pub start_offset: u64,
    /// Total file size
    pub total_size: u64,
    /// Slice data
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}

mod serde_bytes {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(bytes)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Vec::<u8>::deserialize(deserializer)?;
        Ok(s)
    }
}
