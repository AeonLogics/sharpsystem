use crate::shared::notifications::{NotificationLevel, SystemNotification};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum AuthError {
    #[error("Identity Conflict: This identifier is already registered.")]
    UserAlreadyExists,
    #[error("{0}")]
    InvalidInput(String),
    #[error("Security Breach: Invalid credentials provided.")]
    InvalidCredentials,
    #[error("System Error: The core service is temporarily unavailable.")]
    InternalError,
}

impl SystemNotification for AuthError {
    fn id(&self) -> String {
        format!("auth-error-{:?}", self)
    }

    fn title(&self) -> String {
        match self {
            Self::UserAlreadyExists => "Conflict Detected".to_string(),
            Self::InvalidInput(_) => "Data Invalid".to_string(),
            Self::InvalidCredentials => "Auth Failure".to_string(),
            Self::InternalError => "System Fault".to_string(),
        }
    }

    fn message(&self) -> String {
        self.to_string()
    }

    fn level(&self) -> NotificationLevel {
        match self {
            Self::UserAlreadyExists | Self::InvalidInput(_) | Self::InvalidCredentials => {
                NotificationLevel::Warning
            }
            Self::InternalError => NotificationLevel::Error,
        }
    }
}
