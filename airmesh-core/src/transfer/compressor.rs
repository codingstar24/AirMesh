//! High-throughput LZ4 compression with entropy autodetection.

use crate::protocol::error::{AirMeshError, Result};

/// Threshold ratio: if compressed / uncompressed > 0.95, skip compression
pub const COMPRESSION_RATIO_THRESHOLD: f32 = 0.95;

/// Compressor utility.
pub struct ChunkCompressor;

impl ChunkCompressor {
    /// Compresses data using LZ4 if beneficial.
    ///
    /// Returns `(output_bytes, was_compressed)`.
    pub fn compress_if_beneficial(data: &[u8]) -> (Vec<u8>, bool) {
        if data.len() < 512 {
            // Tiny chunks don't benefit from compression
            return (data.to_vec(), false);
        }

        let compressed = lz4_flex::compress_prepend_size(data);
        let ratio = (compressed.len() as f32) / (data.len() as f32);

        if ratio < COMPRESSION_RATIO_THRESHOLD {
            (compressed, true)
        } else {
            // High entropy or already compressed
            (data.to_vec(), false)
        }
    }

    /// Decompresses LZ4-compressed chunk data.
    pub fn decompress(compressed: &[u8]) -> Result<Vec<u8>> {
        lz4_flex::decompress_size_prepended(compressed)
            .map_err(|e| AirMeshError::Transfer(format!("LZ4 decompression error: {e}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lz4_compression_roundtrip() {
        let text = b"AirMesh local mesh file transfer repetitive repetitive repetitive";
        let (compressed, was_compressed) = ChunkCompressor::compress_if_beneficial(text);
        assert!(was_compressed);

        let decompressed = ChunkCompressor::decompress(&compressed).unwrap();
        assert_eq!(text.as_slice(), decompressed.as_slice());
    }
}
