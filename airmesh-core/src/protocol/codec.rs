//! Length-prefixed binary frame encoder and decoder.

use crate::protocol::error::{AirMeshError, ErrorCode, Result};
use crate::protocol::frames::{Frame, FRAME_MAGIC};
use bytes::{Buf, BufMut, BytesMut};

/// Maximum allowable frame size (16 MiB) to prevent memory exhaustion DoS
pub const MAX_FRAME_SIZE: usize = 16 * 1024 * 1024;

/// Frame encoder / decoder utility.
#[derive(Debug, Default)]
pub struct FrameCodec;

impl FrameCodec {
    /// Encodes a `Frame` into bytes with magic prefix and 4-byte big-endian length.
    pub fn encode(frame: &Frame, dst: &mut BytesMut) -> Result<()> {
        let serialized = serde_json::to_vec(frame)
            .map_err(|e| AirMeshError::Serialization(e.to_string()))?;

        if serialized.len() > MAX_FRAME_SIZE {
            return Err(AirMeshError::Protocol {
                code: ErrorCode::MalformedFrame,
                message: format!("Frame payload {} exceeds maximum {}", serialized.len(), MAX_FRAME_SIZE),
            });
        }

        dst.reserve(FRAME_MAGIC.len() + 4 + serialized.len());
        dst.put_slice(&FRAME_MAGIC);
        dst.put_u32(serialized.len() as u32);
        dst.put_slice(&serialized);
        Ok(())
    }

    /// Decodes a frame from incoming byte buffer if a full frame is available.
    pub fn decode(src: &mut BytesMut) -> Result<Option<Frame>> {
        let header_len = FRAME_MAGIC.len() + 4;
        if src.len() < header_len {
            return Ok(None);
        }

        // Validate magic prefix
        if &src[..FRAME_MAGIC.len()] != FRAME_MAGIC {
            return Err(AirMeshError::Protocol {
                code: ErrorCode::MalformedFrame,
                message: "Invalid frame magic header".to_string(),
            });
        }

        let payload_len = u32::from_be_bytes([
            src[FRAME_MAGIC.len()],
            src[FRAME_MAGIC.len() + 1],
            src[FRAME_MAGIC.len() + 2],
            src[FRAME_MAGIC.len() + 3],
        ]) as usize;

        if payload_len > MAX_FRAME_SIZE {
            return Err(AirMeshError::Protocol {
                code: ErrorCode::MalformedFrame,
                message: format!("Frame size {} exceeds maximum allowable limit", payload_len),
            });
        }

        let total_len = header_len + payload_len;
        if src.len() < total_len {
            // Need more data
            return Ok(None);
        }

        // Advance past header
        src.advance(header_len);
        let payload = src.split_to(payload_len);

        let frame: Frame = serde_json::from_slice(&payload)
            .map_err(|e| AirMeshError::Serialization(e.to_string()))?;

        Ok(Some(frame))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codec_roundtrip() {
        let frame = Frame::Ping { nonce: 123456789 };
        let mut buf = BytesMut::new();
        FrameCodec::encode(&frame, &mut buf).unwrap();

        let decoded = FrameCodec::decode(&mut buf).unwrap();
        assert!(decoded.is_some());
        match decoded.unwrap() {
            Frame::Ping { nonce } => assert_eq!(nonce, 123456789),
            _ => panic!("Expected Ping frame"),
        }
    }
}
