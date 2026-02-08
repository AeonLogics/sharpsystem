use crate::auth::AuthError;
use crate::shared::notifications::{NotificationLevel, SystemNotification};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error)]
pub enum SystemError {
    #[error("Authentication failed: {0}")]
    Auth(#[from] AuthError),
    #[error("Database error: {0}")]
    Database(String),
    #[error("Inventory error: {0}")]
    Inventory(String),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Unexpected error: {0}")]
    General(String),
}

impl SystemNotification for SystemError {
    fn id(&self) -> String {
        // We can use a consistent prefix or just generate a new ID
        // For errors, sometimes a static ID per type is useful, but UUID is safer for stacking toasts
        #[cfg(target_arch = "wasm32")]
        let id = format!("err-{}", js_sys::Math::random().to_string());
        #[cfg(not(target_arch = "wasm32"))]
        let id = format!("err-{}", uuid::Uuid::new_v4());

        id
    }

    fn title(&self) -> String {
        match self {
            SystemError::Auth(_) => "Login Error".to_string(),
            SystemError::Database(_) => "System Error".to_string(),
            SystemError::Inventory(_) => "Inventory Alert".to_string(),
            SystemError::Validation(_) => "Input Error".to_string(),
            SystemError::General(_) => "Error".to_string(),
        }
    }

    fn message(&self) -> String {
        self.to_string()
    }

    fn level(&self) -> NotificationLevel {
        NotificationLevel::Error
    }
}
