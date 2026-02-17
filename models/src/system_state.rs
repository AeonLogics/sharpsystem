use crate::entities::User;
use crate::shared::notifications::SystemNotification;
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct SystemState {
    pub toasts: Vec<Arc<dyn SystemNotification + Send + Sync>>,
    pub modal: Option<Arc<dyn SystemNotification + Send + Sync>>,
    pub user: Option<User>,
    pub auth_initialized: bool,
}

impl SystemState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_toast(&mut self, notification: Arc<dyn SystemNotification + Send + Sync>) {
        self.toasts.push(notification);
    }

    pub fn remove_toast(&mut self, id: &str) {
        self.toasts.retain(|t| t.id() != id);
    }

    pub fn set_modal(&mut self, notification: Arc<dyn SystemNotification + Send + Sync>) {
        self.modal = Some(notification);
    }

    pub fn clear_modal(&mut self) {
        self.modal = None;
    }

    pub fn set_user(&mut self, user: User) {
        self.user = Some(user);
    }

    pub fn logout(&mut self) {
        self.user = None;
    }
}
