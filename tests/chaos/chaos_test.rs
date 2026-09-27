//! Chaos engineering test suite: corruption, path traversal, dropouts, bitflips.

use airmesh_core::crypto::integrity::{hash_chunk, verify_chunk};
use airmesh_core::platform::{FileSystem, NativeFileSystem};
use airmesh_core::protocol::codec::FrameCodec;
use airmesh_core::transfer::bitmap::TransferBitmap;
use bytes::BytesMut;
use std::path::Path;

#[test]
fn test_chaos_corrupt_chunk_bitflip_rejection() {
    let original = b"High-value encrypted document payload";
    let valid_hash = hash_chunk(original);

    let mut corrupted = original.to_vec();
    // Invert a single bit
    corrupted[5] ^= 0x01;

    let res = verify_chunk(&corrupted, &valid_hash);
    assert!(res.is_err(), "Corrupt chunk MUST be rejected by BLAKE3");
}

#[test]
fn test_chaos_path_traversal_attacks() {
    let fs = NativeFileSystem;
    let root = Path::new("/var/airmesh/shared");

    let attacks = vec![
        "../../etc/passwd",
        "..\\..\\Windows\\System32\\cmd.exe",
        "/absolute/path/escape",
        "nested/../../secret.key",
        "....//....//config.toml",
    ];

    for malicious in attacks {
        let res = fs.sanitize_path(root, malicious);
        assert!(res.is_err(), "Path traversal attack must fail: {malicious}");
    }
}

#[test]
fn test_chaos_interrupted_transfer_resumption() {
    // 100 chunks total
    let total = 100;
    let mut bitmap = TransferBitmap::new("chaos_file".into(), total);

    // Simulate transfer crashing at chunk 47
    for i in 0..47 {
        bitmap.mark_completed(i);
    }
    assert_eq!(bitmap.completed_count(), 47);
    assert!(!bitmap.is_complete());

    // Next missing chunk should be exactly 47
    assert_eq!(bitmap.next_missing_chunk(0), Some(47));

    // Simulate resume continuing from 47 to 100
    for i in 47..total {
        bitmap.mark_completed(i);
    }
    assert_eq!(bitmap.completed_count(), 100);
    assert!(bitmap.is_complete());
}

#[test]
fn test_chaos_malformed_wire_frames() {
    let mut bad_buf = BytesMut::from(&b"MALFORMED_GARBAGE_HEADER_DATA_12345"[..]);
    let res = FrameCodec::decode(&mut bad_buf);
    assert!(res.is_err(), "Malformed magic header must error");
}
