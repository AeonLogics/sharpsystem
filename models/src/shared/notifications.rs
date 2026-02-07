use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum NotificationLevel {
    Success,
    Warning,
    Error,
    Info,
}

pub trait SystemNotification {
    fn title(&self) -> String;
    fn message(&self) -> String;
    fn level(&self) -> NotificationLevel;
}
