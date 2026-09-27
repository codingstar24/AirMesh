//! BLAKE3 cryptographic hash verification and integrity engine.

use crate::crypto::identity::hex;
use crate::protocol::error::{AirMeshError, ErrorCode, Result};

/// Computes BLAKE3 256-bit hash of an in-memory byte slice.
#[must_use]
pub fn hash_chunk(data: &[u8]) -> [u8; 32] {
    *blake3::hash(data).as_bytes()
}

/// Computes BLAKE3 256-bit hash and formats as hex string.
#[must_use]
pub fn hash_chunk_hex(data: &[u8]) -> String {
    hex::encode(&hash_chunk(data))
}

/// Verifies that data matches the expected BLAKE3 hash.
pub fn verify_chunk(data: &[u8], expected_hash: &[u8; 32]) -> Result<()> {
    let actual = hash_chunk(data);
    if &actual == expected_hash {
        Ok(())
    } else {
        Err(AirMeshError::Protocol {
            code: ErrorCode::ChecksumMismatch,
            message: format!(
                "BLAKE3 checksum mismatch: expected {}, got {}",
                hex::encode(expected_hash),
                hex::encode(&actual)
            ),
        })
    }
}

/// Computes incremental hash over multiple chunk hashes to produce root hash.
pub fn compute_root_hash(chunk_hashes: &[[u8; 32]]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    for hash in chunk_hashes {
        hasher.update(hash);
    }
    *hasher.finalize().as_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verification() {
        let sample = b"hello airmesh mesh network 2026";
        let h = hash_chunk(sample);
        assert!(verify_chunk(sample, &h).is_ok());

        let corrupted = b"hello airmesh mesh network 2027";
        assert!(verify_chunk(corrupted, &h).is_err());
    }
}
