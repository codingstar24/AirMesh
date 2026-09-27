//! Data structures for local and mesh file indexing.

use serde::{Deserialize, Serialize};

/// Indexed file record in local database.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexedFile {
    /// Content-addressed identifier (BLAKE3 root hash hex)
    pub id: String,
    /// Absolute filesystem path on local disk
    pub local_path: String,
    /// Virtual relative path shared to mesh
    pub virtual_path: String,
    /// File size in bytes
    pub size: u64,
    /// MIME type
    pub mime_type: String,
    /// Modification timestamp
    pub modified_at: i64,
    /// Total number of 1 MiB chunks
    pub chunk_count: u32,
    /// Root BLAKE3 hash
    pub root_hash: String,
}

/// Search filter query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    /// Full-text query string
    pub query: String,
    /// Optional MIME type filter (e.g., "video/", "image/")
    pub mime_filter: Option<String>,
    /// Minimum file size
    pub min_size: Option<u64>,
    /// Maximum file size
    pub max_size: Option<u64>,
    /// Maximum number of results (default 50)
    pub limit: usize,
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            query: String::new(),
            mime_filter: None,
            min_size: None,
            max_size: None,
            limit: 50,
        }
    }
}

/// Search hit result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchResult {
    /// Content ID
    pub id: String,
    /// File name or virtual path
    pub virtual_path: String,
    /// File size
    pub size: u64,
    /// MIME type
    pub mime_type: String,
    /// BLAKE3 root hash
    pub root_hash: String,
    /// Search relevance rank score
    pub score: i64,
}
