//! Fixed-size 1 MiB chunk partitioning and hashing.

use crate::crypto::integrity::hash_chunk;
use crate::protocol::error::{AirMeshError, Result};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

/// Standard chunk size: 1 MiB (1,048,576 bytes)
pub const CHUNK_SIZE_1MIB: usize = 1024 * 1024;

/// Chunk descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkInfo {
    /// 0-indexed chunk index
    pub index: u32,
    /// Byte offset within file
    pub offset: u64,
    /// Size of this chunk in bytes
    pub size: usize,
    /// Expected BLAKE3 256-bit hash
    pub hash: [u8; 32],
}

/// Chunker reading 1 MiB slices from disk.
pub struct Chunker;

impl Chunker {
    /// Reads a specific 1 MiB chunk from a local file.
    pub fn read_chunk(path: &Path, chunk_index: u32) -> Result<(Vec<u8>, [u8; 32])> {
        let mut file = File::open(path).map_err(AirMeshError::Io)?;
        let offset = (chunk_index as u64) * (CHUNK_SIZE_1MIB as u64);
        file.seek(SeekFrom::Start(offset)).map_err(AirMeshError::Io)?;

        let mut buf = vec![0u8; CHUNK_SIZE_1MIB];
        let mut total_read = 0;

        while total_read < CHUNK_SIZE_1MIB {
            let n = file.read(&mut buf[total_read..]).map_err(AirMeshError::Io)?;
            if n == 0 {
                break;
            }
            total_read += n;
        }

        buf.truncate(total_read);
        let hash = hash_chunk(&buf);
        Ok((buf, hash))
    }

    /// Computes total chunks required for a file of given byte size.
    #[must_use]
    pub fn total_chunks(file_size: u64) -> u32 {
        if file_size == 0 {
            1
        } else {
            ((file_size + (CHUNK_SIZE_1MIB as u64) - 1) / (CHUNK_SIZE_1MIB as u64)) as u32
        }
    }
}
