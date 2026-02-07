use crate::shared::notifications::SystemNotification;
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct SystemState {
    pub notification: Option<Arc<dyn SystemNotification + Send + Sync>>,
}

impl SystemState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_notification(&mut self, notification: Arc<dyn SystemNotification + Send + Sync>) {
        self.notification = Some(notification);
    }

    pub fn clear_notification(&mut self) {
        self.notification = None;
    }
}
