//! Filesystem change watcher and incremental catalog updater.

use crate::crypto::integrity::hash_chunk;
use crate::index::db::IndexDatabase;
use crate::index::types::IndexedFile;
use crate::protocol::error::{AirMeshError, Result};
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{error, info, warn};

/// Shared folder watcher service.
pub struct FolderWatcher {
    root_dir: PathBuf,
    db: Arc<IndexDatabase>,
}

impl FolderWatcher {
    /// Creates a new watcher for a shared directory.
    pub fn new<P: AsRef<Path>>(root_dir: P, db: Arc<IndexDatabase>) -> Self {
        Self {
            root_dir: root_dir.as_ref().to_path_buf(),
            db,
        }
    }

    /// Recursively scans shared folder and populates index immediately on cold start.
    pub fn scan_and_index_all(&self) -> Result<usize> {
        let mut count = 0;
        self.walk_dir(&self.root_dir, &mut count)?;
        info!("Cold scan complete: indexed {} files", count);
        Ok(count)
    }

    fn walk_dir(&self, dir: &Path, count: &mut usize) -> Result<()> {
        if !dir.is_dir() {
            return Ok(());
        }

        let entries = std::fs::read_dir(dir)
            .map_err(|e| AirMeshError::Io(e))?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                self.walk_dir(&path, count)?;
            } else if path.is_file() {
                if let Ok(file_meta) = self.index_single_file(&path) {
                    if self.db.upsert_file(&file_meta).is_ok() {
                        *count += 1;
                    }
                }
            }
        }
        Ok(())
    }

    /// Indexes a single file calculating BLAKE3 root hash.
    pub fn index_single_file(&self, path: &Path) -> Result<IndexedFile> {
        let meta = std::fs::metadata(path)
            .map_err(|e| AirMeshError::Io(e))?;

        let size = meta.len();
        let modified_at = meta
            .modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        let relative_path = path
            .strip_prefix(&self.root_dir)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");

        let mut file = File::open(path).map_err(AirMeshError::Io)?;
        let mut hasher = blake3::Hasher::new();
        let mut buf = [0u8; 64 * 1024];

        loop {
            let n = file.read(&mut buf).map_err(AirMeshError::Io)?;
            if n == 0 {
                break;
            }
            hasher.update(&buf[..n]);
        }

        let root_hash = hasher.finalize().to_hex().to_string();
        let chunk_size = 1024 * 1024;
        let chunk_count = ((size + chunk_size - 1) / chunk_size).max(1) as u32;

        let mime_type = mime_guess::from_path(path)
            .first_or_octet_stream()
            .to_string();

        Ok(IndexedFile {
            id: root_hash.clone(),
            local_path: path.to_string_lossy().to_string(),
            virtual_path: relative_path,
            size,
            mime_type,
            modified_at,
            chunk_count,
            root_hash,
        })
    }

    /// Starts active filesystem watcher in a background task.
    pub async fn start_live_watcher(self: Arc<Self>) -> Result<()> {
        let (tx, mut rx) = mpsc::channel(256);

        let mut watcher = RecommendedWatcher::new(
            move |res: notify::Result<Event>| {
                if let Ok(event) = res {
                    let _ = tx.blocking_send(event);
                }
            },
            Config::default(),
        )
        .map_err(|e| AirMeshError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

        watcher
            .watch(&self.root_dir, RecursiveMode::Recursive)
            .map_err(|e| AirMeshError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

        tokio::spawn(async move {
            let _watcher = watcher; // Keep alive
            while let Some(event) = rx.recv().await {
                match event.kind {
                    EventKind::Create(_) | EventKind::Modify(_) => {
                        for path in event.paths {
                            if path.is_file() {
                                if let Ok(indexed) = self.index_single_file(&path) {
                                    let _ = self.db.upsert_file(&indexed);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        });

        Ok(())
    }
}

// Minimal fallback for MIME guessing
mod mime_guess {
    use std::path::Path;

    pub struct MimeGuess(&'static str);
    impl MimeGuess {
        pub fn first_or_octet_stream(&self) -> &'static str {
            self.0
        }
    }

    pub fn from_path(path: &Path) -> MimeGuess {
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        let mime = match ext.as_str() {
            "mp4" | "m4v" => "video/mp4",
            "mkv" => "video/x-matroska",
            "webm" => "video/webm",
            "mp3" => "audio/mpeg",
            "flac" => "audio/flac",
            "wav" => "audio/wav",
            "jpg" | "jpeg" => "image/jpeg",
            "png" => "image/png",
            "webp" => "image/webp",
            "gif" => "image/gif",
            "pdf" => "application/pdf",
            "zip" => "application/zip",
            "tar" | "gz" | "zst" => "application/octet-stream",
            "txt" | "md" | "rs" | "js" | "ts" | "json" => "text/plain",
            _ => "application/octet-stream",
        };
        MimeGuess(mime)
    }
}
