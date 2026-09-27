//! Abstract Keychain trait for secure cryptographic seed persistence.

use crate::protocol::error::{AirMeshError, Result};
use async_trait::async_trait;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Trait abstracting hardware or OS credential store.
#[async_trait]
pub trait Keychain: Send + Sync {
    /// Stores a secret key by name.
    async fn set_secret(&self, key: &str, secret: &[u8]) -> Result<()>;

    /// Retrieves a secret key by name.
    async fn get_secret(&self, key: &str) -> Result<Option<Vec<u8>>>;

    /// Deletes a secret key by name.
    async fn delete_secret(&self, key: &str) -> Result<()>;
}

/// Production filesystem-backed secure key store.
pub struct NativeKeychain {
    base_dir: PathBuf,
}

impl NativeKeychain {
    /// Creates keychain at specified directory.
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }
}

#[async_trait]
impl Keychain for NativeKeychain {
    async fn set_secret(&self, key: &str, secret: &[u8]) -> Result<()> {
        tokio::fs::create_dir_all(&self.base_dir).await.map_err(AirMeshError::Io)?;
        let path = self.base_dir.join(format!("{key}.key"));
        tokio::fs::write(&path, secret).await.map_err(AirMeshError::Io)?;
        Ok(())
    }

    async fn get_secret(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let path = self.base_dir.join(format!("{key}.key"));
        if tokio::fs::metadata(&path).await.is_err() {
            return Ok(None);
        }
        let data = tokio::fs::read(&path).await.map_err(AirMeshError::Io)?;
        Ok(Some(data))
    }

    async fn delete_secret(&self, key: &str) -> Result<()> {
        let path = self.base_dir.join(format!("{key}.key"));
        let _ = tokio::fs::remove_file(&path).await;
        Ok(())
    }
}

/// In-memory mock keychain for testing.
#[derive(Default, Clone)]
pub struct MockKeychain {
    vault: Arc<Mutex<HashMap<String, Vec<u8>>>>,
}

impl MockKeychain {
    /// Creates fresh mock keychain.
    pub fn new() -> Self {
        Self {
            vault: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl Keychain for MockKeychain {
    async fn set_secret(&self, key: &str, secret: &[u8]) -> Result<()> {
        let mut guard = self.vault.lock().unwrap();
        guard.insert(key.to_string(), secret.to_vec());
        Ok(())
    }

    async fn get_secret(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let guard = self.vault.lock().unwrap();
        Ok(guard.get(key).cloned())
    }

    async fn delete_secret(&self, key: &str) -> Result<()> {
        let mut guard = self.vault.lock().unwrap();
        guard.remove(key);
        Ok(())
    }
}
