//! High-performance SQLite + FTS5 indexing engine.

use crate::index::types::{IndexedFile, SearchQuery, SearchResult};
use crate::protocol::error::{AirMeshError, Result};
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// SQLite database engine managing indexed files and chunk catalogs.
pub struct IndexDatabase {
    conn: Arc<Mutex<Connection>>,
}

impl IndexDatabase {
    /// Opens or creates SQLite database with optimized PRAGMAs.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)
            .map_err(|e| AirMeshError::Database(format!("Failed opening SQLite database: {e}")))?;

        // Performance tuning pragmas
        conn.execute_batch(
            "
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;
            PRAGMA temp_store = MEMORY;
            PRAGMA mmap_size = 30000000000;
            PRAGMA cache_size = -64000;
            ",
        )
        .map_err(|e| AirMeshError::Database(format!("Pragma configuration failed: {e}")))?;

        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.init_schema()?;
        Ok(db)
    }

    /// Initializes schema with FTS5 search index.
    fn init_schema(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS files (
                id TEXT PRIMARY KEY,
                local_path TEXT NOT NULL,
                virtual_path TEXT NOT NULL,
                size INTEGER NOT NULL,
                mime_type TEXT NOT NULL,
                modified_at INTEGER NOT NULL,
                chunk_count INTEGER NOT NULL,
                root_hash TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_files_virtual_path ON files(virtual_path);
            CREATE INDEX IF NOT EXISTS idx_files_mime ON files(mime_type);

            CREATE VIRTUAL TABLE IF NOT EXISTS files_fts USING fts5(
                id UNINDEXED,
                virtual_path,
                content='files',
                content_rowid='rowid'
            );

            CREATE TRIGGER IF NOT EXISTS files_ai AFTER INSERT ON files BEGIN
                INSERT INTO files_fts(rowid, id, virtual_path) VALUES (new.rowid, new.id, new.virtual_path);
            END;

            CREATE TRIGGER IF NOT EXISTS files_ad AFTER DELETE ON files BEGIN
                INSERT INTO files_fts(files_fts, rowid, id, virtual_path) VALUES('delete', old.rowid, old.id, old.virtual_path);
            END;

            CREATE TRIGGER IF NOT EXISTS files_au AFTER UPDATE ON files BEGIN
                INSERT INTO files_fts(files_fts, rowid, id, virtual_path) VALUES('delete', old.rowid, old.id, old.virtual_path);
                INSERT INTO files_fts(rowid, id, virtual_path) VALUES (new.rowid, new.id, new.virtual_path);
            END;

            CREATE TABLE IF NOT EXISTS transfer_bitmaps (
                file_id TEXT PRIMARY KEY,
                total_chunks INTEGER NOT NULL,
                bitmap_blob BLOB NOT NULL,
                updated_at INTEGER NOT NULL
            );
            ",
        )
        .map_err(|e| AirMeshError::Database(format!("Schema initialization failed: {e}")))?;

        Ok(())
    }

    /// Upserts an indexed file record.
    pub fn upsert_file(&self, file: &IndexedFile) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "
            INSERT INTO files (id, local_path, virtual_path, size, mime_type, modified_at, chunk_count, root_hash)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            ON CONFLICT(id) DO UPDATE SET
                local_path = excluded.local_path,
                virtual_path = excluded.virtual_path,
                size = excluded.size,
                mime_type = excluded.mime_type,
                modified_at = excluded.modified_at,
                chunk_count = excluded.chunk_count,
                root_hash = excluded.root_hash;
            ",
            params![
                file.id,
                file.local_path,
                file.virtual_path,
                file.size as i64,
                file.mime_type,
                file.modified_at,
                file.chunk_count,
                file.root_hash
            ],
        )
        .map_err(|e| AirMeshError::Database(format!("File upsert failed: {e}")))?;

        Ok(())
    }

    /// Fetches file by content ID.
    pub fn get_file_by_id(&self, file_id: &str) -> Result<Option<IndexedFile>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT id, local_path, virtual_path, size, mime_type, modified_at, chunk_count, root_hash
                 FROM files WHERE id = ?1 LIMIT 1",
            )
            .map_err(|e| AirMeshError::Database(e.to_string()))?;

        let mut rows = stmt
            .query(params![file_id])
            .map_err(|e| AirMeshError::Database(e.to_string()))?;

        if let Some(row) = rows.next().map_err(|e| AirMeshError::Database(e.to_string()))? {
            let size: i64 = row.get(3).unwrap_or(0);
            Ok(Some(IndexedFile {
                id: row.get(0).unwrap_or_default(),
                local_path: row.get(1).unwrap_or_default(),
                virtual_path: row.get(2).unwrap_or_default(),
                size: size as u64,
                mime_type: row.get(4).unwrap_or_default(),
                modified_at: row.get(5).unwrap_or(0),
                chunk_count: row.get(6).unwrap_or(0),
                root_hash: row.get(7).unwrap_or_default(),
            }))
        } else {
            Ok(None)
        }
    }

    /// Searches files using SQLite FTS5.
    pub fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        let conn = self.conn.lock().unwrap();
        let mut results = Vec::new();

        if query.query.trim().is_empty() {
            let mut stmt = conn
                .prepare("SELECT id, virtual_path, size, mime_type, root_hash FROM files ORDER BY modified_at DESC LIMIT ?1")
                .map_err(|e| AirMeshError::Database(e.to_string()))?;

            let rows = stmt
                .query_map(params![query.limit as i64], |row| {
                    let sz: i64 = row.get(2)?;
                    Ok(SearchResult {
                        id: row.get(0)?,
                        virtual_path: row.get(1)?,
                        size: sz as u64,
                        mime_type: row.get(3)?,
                        root_hash: row.get(4)?,
                        score: 100,
                    })
                })
                .map_err(|e| AirMeshError::Database(e.to_string()))?;

            for r in rows {
                if let Ok(item) = r {
                    results.push(item);
                }
            }
        } else {
            let sanitized = query.query.replace('"', "\"\"") + "*";
            let mut stmt = conn
                .prepare(
                    "
                    SELECT f.id, f.virtual_path, f.size, f.mime_type, f.root_hash, bm25(files_fts) as rank
                    FROM files_fts
                    JOIN files f ON f.id = files_fts.id
                    WHERE files_fts MATCH ?1
                    ORDER BY rank ASC
                    LIMIT ?2;
                    ",
                )
                .map_err(|e| AirMeshError::Database(e.to_string()))?;

            let rows = stmt
                .query_map(params![sanitized, query.limit as i64], |row| {
                    let sz: i64 = row.get(2)?;
                    let rank: f64 = row.get(5).unwrap_or(0.0);
                    Ok(SearchResult {
                        id: row.get(0)?,
                        virtual_path: row.get(1)?,
                        size: sz as u64,
                        mime_type: row.get(3)?,
                        root_hash: row.get(4)?,
                        score: (rank * -10.0) as i64,
                    })
                })
                .map_err(|e| AirMeshError::Database(e.to_string()))?;

            for r in rows {
                if let Ok(item) = r {
                    results.push(item);
                }
            }
        }

        Ok(results)
    }

    /// Lists all files for directory browsing.
    pub fn list_files(&self, offset: usize, limit: usize) -> Result<Vec<IndexedFile>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT id, local_path, virtual_path, size, mime_type, modified_at, chunk_count, root_hash
                 FROM files ORDER BY virtual_path ASC LIMIT ?1 OFFSET ?2",
            )
            .map_err(|e| AirMeshError::Database(e.to_string()))?;

        let rows = stmt
            .query_map(params![limit as i64, offset as i64], |row| {
                let sz: i64 = row.get(3)?;
                Ok(IndexedFile {
                    id: row.get(0)?,
                    local_path: row.get(1)?,
                    virtual_path: row.get(2)?,
                    size: sz as u64,
                    mime_type: row.get(4)?,
                    modified_at: row.get(5)?,
                    chunk_count: row.get(6)?,
                    root_hash: row.get(7)?,
                })
            })
            .map_err(|e| AirMeshError::Database(e.to_string()))?;

        let mut list = Vec::new();
        for r in rows {
            if let Ok(item) = r {
                list.push(item);
            }
        }
        Ok(list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqlite_indexing_and_search() {
        let db = IndexDatabase::open(":memory:").unwrap();
        let file = IndexedFile {
            id: "blake3_abc123".into(),
            local_path: "/data/docs/architecture.pdf".into(),
            virtual_path: "docs/architecture.pdf".into(),
            size: 2048576,
            mime_type: "application/pdf".into(),
            modified_at: 1700000000,
            chunk_count: 2,
            root_hash: "blake3_abc123".into(),
        };

        db.upsert_file(&file).unwrap();

        let queried = db.get_file_by_id("blake3_abc123").unwrap();
        assert!(queried.is_some());
        assert_eq!(queried.unwrap().virtual_path, "docs/architecture.pdf");

        let q = SearchQuery {
            query: "architecture".into(),
            ..Default::default()
        };
        let search_hits = db.search(&q).unwrap();
        assert_eq!(search_hits.len(), 1);
        assert_eq!(search_hits[0].id, "blake3_abc123");
    }
}
