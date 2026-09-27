//! Protocol framing, versioning, serialization and error handling.

pub mod codec;
pub mod error;
pub mod frames;
pub mod version;

pub use codec::{FrameCodec, MAX_FRAME_SIZE};
pub use error::{AirMeshError, ErrorCode, Result};
pub use frames::{
    ChunkRequestPayload, ChunkResponsePayload, FileMetadata, Frame, HandshakeInitPayload,
    HandshakeResponsePayload, ManifestResponsePayload, StreamRequestPayload, StreamResponsePayload,
    FRAME_MAGIC,
};
pub use version::{Capabilities, ProtocolVersion};
