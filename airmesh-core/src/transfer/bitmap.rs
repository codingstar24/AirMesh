//! Persistent transfer progress bitmap for zero-overhead resume.

use crate::protocol::error::{AirMeshError, Result};
use serde::{Deserialize, Serialize};

/// Bitmap tracking completed chunks of a file transfer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferBitmap {
    /// Target file ID
    pub file_id: String,
    /// Total chunks in target file
    pub total_chunks: u32,
    /// Packed bit array (1 bit per chunk)
    pub bits: Vec<u8>,
}

impl TransferBitmap {
    /// Creates a new empty bitmap for a file with `total_chunks`.
    pub fn new(file_id: String, total_chunks: u32) -> Self {
        let byte_count = ((total_chunks + 7) / 8) as usize;
        Self {
            file_id,
            total_chunks,
            bits: vec![0u8; byte_count],
        }
    }

    /// Marks chunk as successfully downloaded and verified.
    pub fn mark_completed(&mut self, chunk_index: u32) {
        if chunk_index >= self.total_chunks {
            return;
        }
        let byte_idx = (chunk_index / 8) as usize;
        let bit_idx = (chunk_index % 8) as u8;
        if byte_idx < self.bits.len() {
            self.bits[byte_idx] |= 1 << bit_idx;
        }
    }

    /// Checks if a chunk is marked completed.
    #[must_use]
    pub fn is_chunk_completed(&self, chunk_index: u32) -> bool {
        if chunk_index >= self.total_chunks {
            return false;
        }
        let byte_idx = (chunk_index / 8) as usize;
        let bit_idx = (chunk_index % 8) as u8;
        if byte_idx < self.bits.len() {
            (self.bits[byte_idx] & (1 << bit_idx)) != 0
        } else {
            false
        }
    }

    /// Finds the next uncompleted chunk index starting from `from_index`.
    #[must_use]
    pub fn next_missing_chunk(&self, from_index: u32) -> Option<u32> {
        for idx in from_index..self.total_chunks {
            if !self.is_chunk_completed(idx) {
                return Some(idx);
            }
        }
        None
    }

    /// Returns list of all remaining missing chunk indices.
    #[must_use]
    pub fn missing_chunks(&self) -> Vec<u32> {
        (0..self.total_chunks)
            .filter(|&idx| !self.is_chunk_completed(idx))
            .collect()
    }

    /// Counts total completed chunks.
    #[must_use]
    pub fn completed_count(&self) -> u32 {
        let mut count = 0;
        for i in 0..self.total_chunks {
            if self.is_chunk_completed(i) {
                count += 1;
            }
        }
        count
    }

    /// Determines whether the entire transfer has completed.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.completed_count() == self.total_chunks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitmap_progress_and_completion() {
        let mut bitmap = TransferBitmap::new("file_123".into(), 10);
        assert_eq!(bitmap.completed_count(), 0);
        assert!(!bitmap.is_complete());

        bitmap.mark_completed(0);
        bitmap.mark_completed(1);
        bitmap.mark_completed(9);

        assert_eq!(bitmap.completed_count(), 3);
        assert!(bitmap.is_chunk_completed(0));
        assert!(bitmap.is_chunk_completed(1));
        assert!(!bitmap.is_chunk_completed(2));
        assert!(bitmap.is_chunk_completed(9));

        assert_eq!(bitmap.next_missing_chunk(0), Some(2));
    }
}
