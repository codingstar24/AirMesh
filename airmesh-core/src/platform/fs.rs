//! Abstract FileSystem trait with Native and Mock implementations.

use crate::protocol::error::{AirMeshError, Result};
use async_trait::async_trait;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// Trait abstracting filesystem operations for platform independence and testing.
#[async_trait]
pub trait FileSystem: Send + Sync {
    /// Reads file into byte vector.
    async fn read_file(&self, path: &Path) -> Result<Vec<u8>>;

    /// Writes byte vector to file.
    async fn write_file(&self, path: &Path, data: &[u8]) -> Result<()>;

    /// Checks if path exists.
    async fn exists(&self, path: &Path) -> bool;

    /// Sanitizes target path, preventing directory traversal outside root.
    fn sanitize_path(&self, root: &Path, user_path: &str) -> Result<PathBuf>;
}

/// Production implementation using standard operating system filesystem.
pub struct NativeFileSystem;

#[async_trait]
impl FileSystem for NativeFileSystem {
    async fn read_file(&self, path: &Path) -> Result<Vec<u8>> {
        tokio::fs::read(path).await.map_err(AirMeshError::Io)
    }

    async fn write_file(&self, path: &Path, data: &[u8]) -> Result<()> {
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await.map_err(AirMeshError::Io)?;
        }
        tokio::fs::write(path, data).await.map_err(AirMeshError::Io)
    }

    async fn exists(&self, path: &Path) -> bool {
        tokio::fs::metadata(path).await.is_ok()
    }

    fn sanitize_path(&self, root: &Path, user_path: &str) -> Result<PathBuf> {
        // Enforce strict path traversal check
        let cleaned = user_path.replace('\\', "/");
        if cleaned.contains("../") || cleaned.starts_with('/') || cleaned.starts_with("..") {
            return Err(AirMeshError::PathTraversal(format!(
                "Illegal path traversal attempt: {user_path}"
            )));
        }

        let combined = root.join(&cleaned);
        Ok(combined)
    }
}

/// In-memory mock implementation for tests and chaos simulation.
#[derive(Default, Clone)]
pub struct MockFileSystem {
    storage: Arc<Mutex<HashMap<PathBuf, Vec<u8>>>>,
}

impl MockFileSystem {
    /// Creates a fresh mock filesystem.
    pub fn new() -> Self {
        Self {
            storage: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl FileSystem for MockFileSystem {
    async fn read_file(&self, path: &Path) -> Result<Vec<u8>> {
        let guard = self.storage.lock().unwrap();
        guard
            .get(path)
            .cloned()
            .ok_or_else(|| AirMeshError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "Mock file not found")))
    }

    async fn write_file(&self, path: &Path, data: &[u8]) -> Result<()> {
        let mut guard = self.storage.lock().unwrap();
        guard.insert(path.to_path_buf(), data.to_vec());
        Ok(())
    }

    async fn exists(&self, path: &Path) -> bool {
        let guard = self.storage.lock().unwrap();
        guard.contains_key(path)
    }

    fn sanitize_path(&self, root: &Path, user_path: &str) -> Result<PathBuf> {
        let cleaned = user_path.replace('\\', "/");
        if cleaned.contains("../") || cleaned.starts_with('/') {
            return Err(AirMeshError::PathTraversal(format!(
                "Illegal path traversal attempt: {user_path}"
            )));
        }
        Ok(root.join(&cleaned))
    }
}
