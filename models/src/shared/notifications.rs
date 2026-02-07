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
    pub title: String,
    pub message: String,
    pub level: NotificationLevel,
}

impl Notification {
    pub fn new(
        title: impl Into<String>,
        message: impl Into<String>,
        level: NotificationLevel,
    ) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            level,
        }
    }
}

pub trait SystemNotification {
    fn title(&self) -> String;
    fn message(&self) -> String;
    fn level(&self) -> NotificationLevel;
}

impl SystemNotification for Notification {
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
