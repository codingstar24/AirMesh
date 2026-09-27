//! Criterion benchmark measuring BLAKE3 hash and compression throughput.

use airmesh_core::crypto::integrity::hash_chunk;
use airmesh_core::transfer::compressor::ChunkCompressor;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

fn bench_chunk_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("chunk_throughput");
    let size = 1024 * 1024; // 1 MiB
    let data: Vec<u8> = (0..size).map(|i| (i % 255) as u8).collect();

    group.throughput(Throughput::Bytes(size as u64));

    group.bench_function(BenchmarkId::new("blake3_1mb", size), |b| {
        b.iter(|| hash_chunk(black_box(&data)))
    });

    group.bench_function(BenchmarkId::new("lz4_compress_1mb", size), |b| {
        b.iter(|| ChunkCompressor::compress_if_beneficial(black_box(&data)))
    });

    group.finish();
}

criterion_group!(benches, bench_chunk_operations);
criterion_main!(benches);
