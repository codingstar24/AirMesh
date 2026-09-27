//! Criterion benchmark measuring SQLite FTS5 search latency.

use airmesh_core::index::types::{IndexedFile, SearchQuery};
use airmesh_core::index::IndexDatabase;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_fts5_search(c: &mut Criterion) {
    let db = IndexDatabase::open(":memory:").expect("In-memory db setup");

    // Populate with 10,000 files
    for i in 0..10_000 {
        let f = IndexedFile {
            id: format!("hash_{i}"),
            local_path: format!("/shared/dir/document_report_{i}.pdf"),
            virtual_path: format!("dir/document_report_{i}.pdf"),
            size: 1024 * 1024 * (i as u64 % 50 + 1),
            mime_type: "application/pdf".into(),
            modified_at: 1700000000 + i as i64,
            chunk_count: (i as u32 % 50) + 1,
            root_hash: format!("hash_{i}"),
        };
        db.upsert_file(&f).unwrap();
    }

    let query = SearchQuery {
        query: "report".into(),
        limit: 50,
        ..Default::default()
    };

    c.bench_function("search_10k_files_fts5", |b| {
        b.iter(|| db.search(black_box(&query)).unwrap())
    });
}

criterion_group!(benches, bench_fts5_search);
criterion_main!(benches);
