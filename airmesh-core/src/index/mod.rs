//! Content indexing, SQLite FTS5 database, and filesystem change monitor.

pub mod db;
pub mod search;
pub mod types;
pub mod watcher;

pub use db::IndexDatabase;
pub use search::SearchEngine;
pub use types::{IndexedFile, SearchQuery, SearchResult};
pub use watcher::FolderWatcher;
