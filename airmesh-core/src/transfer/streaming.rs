//! Range-based media streaming without pre-downloading full files.

use crate::protocol::error::{AirMeshError, Result};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

/// Stream slice reader for HTTP Range requests.
pub struct MediaStreamReader;

impl MediaStreamReader {
    /// Reads a slice of bytes from a local file starting at `offset` for `length` bytes.
    pub fn read_range(path: &Path, offset: u64, length: u32) -> Result<Vec<u8>> {
        let mut file = File::open(path).map_err(AirMeshError::Io)?;
        let file_size = file.metadata().map_err(AirMeshError::Io)?.len();

        if offset >= file_size {
            return Ok(Vec::new());
        }

        let available = file_size - offset;
        let to_read = (length as u64).min(available) as usize;

        file.seek(SeekFrom::Start(offset)).map_err(AirMeshError::Io)?;
        let mut buffer = vec![0u8; to_read];
        file.read_exact(&mut buffer).map_err(AirMeshError::Io)?;

        Ok(buffer)
    }

    /// Parses standard HTTP Range header (e.g. "bytes=0-1048575").
    pub fn parse_range_header(header: &str, file_size: u64) -> Option<(u64, u32)> {
        let trimmed = header.trim();
        if !trimmed.starts_with("bytes=") {
            return None;
        }

        let range_spec = &trimmed["bytes=".len()..];
        let parts: Vec<&str> = range_spec.split('-').collect();
        if parts.len() != 2 {
            return None;
        }

        let start: u64 = parts[0].parse().ok()?;
        let end: u64 = if parts[1].is_empty() {
            file_size.saturating_sub(1)
        } else {
            parts[1].parse().ok()?
        };

        if start > end || start >= file_size {
            return None;
        }

        let length = ((end - start + 1) as u32).min(32 * 1024 * 1024); // Cap max slice at 32MB
        Some((start, length))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_range_header() {
        let (start, len) = MediaStreamReader::parse_range_header("bytes=0-1048575", 10_000_000).unwrap();
        assert_eq!(start, 0);
        assert_eq!(len, 1048576);

        let (start2, len2) = MediaStreamReader::parse_range_header("bytes=1000-", 2000).unwrap();
        assert_eq!(start2, 1000);
        assert_eq!(len2, 1000);
    }
}
