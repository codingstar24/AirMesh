//! Abstract UserInterfaceBridge trait with Headless and Mock implementations.

use async_trait::async_trait;
use std::sync::{Arc, Mutex};

/// Notification level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiNotificationLevel {
    /// Informational notice
    Info,
    /// Warning message
    Warning,
    /// Critical failure
    Error,
}

/// Abstract UI bridge for system-tray notifications, prompts, and pairing popups.
#[async_trait]
pub trait UserInterfaceBridge: Send + Sync {
    /// Dispatches a notification to the user.
    async fn notify(&self, title: &str, message: &str, level: UiNotificationLevel);

    /// Displays pairing request prompt returning true if accepted.
    async fn prompt_pairing(&self, peer_name: &str, pin_code: &str) -> bool;
}

/// Headless / daemon implementation.
pub struct HeadlessBridge;

#[async_trait]
impl UserInterfaceBridge for HeadlessBridge {
    async fn notify(&self, title: &str, message: &str, level: UiNotificationLevel) {
        tracing::info!("[UI {:?}] {title}: {message}", level);
    }

    async fn prompt_pairing(&self, _peer_name: &str, _pin_code: &str) -> bool {
        // In headless mode, pairings must be approved via API or CLI
        false
    }
}

/// Mock UI bridge for automated tests.
#[derive(Default, Clone)]
pub struct MockUiBridge {
    pub auto_approve_pairing: bool,
    pub notifications: Arc<Mutex<Vec<(String, String)>>>,
}

impl MockUiBridge {
    /// Creates a mock UI bridge.
    pub fn new(auto_approve: bool) -> Self {
        Self {
            auto_approve_pairing: auto_approve,
            notifications: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait]
impl UserInterfaceBridge for MockUiBridge {
    async fn notify(&self, title: &str, message: &str, _level: UiNotificationLevel) {
        let mut guard = self.notifications.lock().unwrap();
        guard.push((title.to_string(), message.to_string()));
    }

    async fn prompt_pairing(&self, _peer_name: &str, _pin_code: &str) -> bool {
        self.auto_approve_pairing
    }
}
