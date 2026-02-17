use crate::auth::AuthError;
use crate::shared::notifications::{NotificationLevel, SystemNotification};
use leptos::prelude::*;
use leptos::server_fn::codec::JsonEncoding;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error)]
pub enum SystemErrorKind {
    #[error("Authentication failed: {0}")]
    Auth(#[from] AuthError),
    #[error("Database error: {0}")]
    Database(String),
    #[error("Inventory error: {0}")]
    Inventory(String),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Unexpected error: {0}")]
    General(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, Error)]
#[error("{kind}")]
pub struct SystemError {
    pub id: String,
    #[source]
    pub kind: SystemErrorKind,
}

impl SystemError {
    fn new(kind: SystemErrorKind) -> Self {
        #[cfg(target_arch = "wasm32")]
        let id = format!("err-{}", js_sys::Math::random().to_string());
        #[cfg(not(target_arch = "wasm32"))]
        let id = format!("err-{}", uuid::Uuid::new_v4());

        Self { id, kind }
    }

    pub fn auth(err: AuthError) -> Self {
        Self::new(SystemErrorKind::Auth(err))
    }

    pub fn database(msg: impl Into<String>) -> Self {
        Self::new(SystemErrorKind::Database(msg.into()))
    }

    pub fn inventory(msg: impl Into<String>) -> Self {
        Self::new(SystemErrorKind::Inventory(msg.into()))
    }

    pub fn validation(msg: impl Into<String>) -> Self {
        Self::new(SystemErrorKind::Validation(msg.into()))
    }

    pub fn unauthorized(msg: impl Into<String>) -> Self {
        Self::new(SystemErrorKind::Unauthorized(msg.into()))
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::new(SystemErrorKind::NotFound(msg.into()))
    }

    pub fn general(msg: impl Into<String>) -> Self {
        Self::new(SystemErrorKind::General(msg.into()))
    }
}

impl SystemNotification for SystemError {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn title(&self) -> String {
        match &self.kind {
            SystemErrorKind::Auth(_) => "Login Error".to_string(),
            SystemErrorKind::Database(_) => "System Error".to_string(),
            SystemErrorKind::Inventory(_) => "Inventory Alert".to_string(),
            SystemErrorKind::Validation(_) => "Input Error".to_string(),
            SystemErrorKind::Unauthorized(_) => "Unauthorized".to_string(),
            SystemErrorKind::NotFound(_) => "Not Found".to_string(),
            SystemErrorKind::General(_) => "Error".to_string(),
        }
    }

    fn message(&self) -> String {
        self.kind.to_string()
    }

    fn level(&self) -> NotificationLevel {
        NotificationLevel::Error
    }
}

impl FromServerFnError for SystemError {
    type Encoder = JsonEncoding;
    fn from_server_fn_error(err: ServerFnErrorErr) -> Self {
        Self::general(err.to_string())
    }
}

impl From<AuthError> for SystemError {
    fn from(err: AuthError) -> Self {
        Self::auth(err)
    }
}
