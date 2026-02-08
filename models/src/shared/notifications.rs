use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum NotificationLevel {
    Success,
    Warning,
    Error,
    Info,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    pub id: String,
    pub title: String,
    pub message: String,
    pub level: NotificationLevel,
    pub created_at: i64,
}

impl Notification {
    pub fn new(
        title: impl Into<String>,
        message: impl Into<String>,
        level: NotificationLevel,
    ) -> Self {
        #[cfg(target_arch = "wasm32")]
        let id = js_sys::Math::random().to_string();
        #[cfg(not(target_arch = "wasm32"))]
        let id = uuid::Uuid::new_v4().to_string();

        Self {
            id,
            title: title.into(),
            message: message.into(),
            level,
            created_at: 0, // Simplified for now
        }
    }
}

pub trait SystemNotification {
    fn id(&self) -> String;
    fn title(&self) -> String;
    fn message(&self) -> String;
    fn level(&self) -> NotificationLevel;
}

impl SystemNotification for Notification {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn message(&self) -> String {
        self.message.clone()
    }

    fn level(&self) -> NotificationLevel {
        self.level
    }
}
