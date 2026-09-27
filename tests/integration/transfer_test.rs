//! Integration test verifying full chunking, hashing, and reassembly pipeline.

use airmesh_core::crypto::integrity::{hash_chunk, verify_chunk};
use airmesh_core::transfer::bitmap::TransferBitmap;
use airmesh_core::transfer::chunker::CHUNK_SIZE_1MIB;
use airmesh_core::transfer::compressor::ChunkCompressor;

#[test]
fn test_end_to_end_chunk_transfer_pipeline() {
    // 1. Synthesize 2.5 MiB test data payload
    let total_size = (2.5 * 1024.0 * 1024.0) as usize;
    let mut original_data = Vec::with_capacity(total_size);
    for i in 0..total_size {
        original_data.push((i % 251) as u8);
    }

    let chunk_count = ((total_size + CHUNK_SIZE_1MIB - 1) / CHUNK_SIZE_1MIB) as u32;
    assert_eq!(chunk_count, 3);

    // 2. Sender partitions and hashes
    let mut chunks = Vec::new();
    for i in 0..chunk_count {
        let start = (i as usize) * CHUNK_SIZE_1MIB;
        let end = ((start + CHUNK_SIZE_1MIB).min(total_size)) as usize;
        let slice = &original_data[start..end];
        let h = hash_chunk(slice);
        let (compressed, was_compressed) = ChunkCompressor::compress_if_beneficial(slice);
        chunks.push((i, compressed, was_compressed, h));
    }

    // 3. Receiver receives chunks, decompresses, verifies, writes to target
    let mut bitmap = TransferBitmap::new("file_test".into(), chunk_count);
    let mut reconstructed = vec![0u8; total_size];

    for (idx, comp_data, was_comp, expected_hash) in chunks {
        let raw = if was_comp {
            ChunkCompressor::decompress(&comp_data).expect("Decompression failed")
        } else {
            comp_data
        };

        // Verify BLAKE3 hash
        assert!(verify_chunk(&raw, &expected_hash).is_ok());

        let offset = (idx as usize) * CHUNK_SIZE_1MIB;
        reconstructed[offset..offset + raw.len()].copy_from_slice(&raw);
        bitmap.mark_completed(idx);
    }

    // 4. Verify completion
    assert!(bitmap.is_complete());
    assert_eq!(original_data, reconstructed);
}
