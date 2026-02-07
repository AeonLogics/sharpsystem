use leptos::prelude::*;

pub trait NotificationTrait {
    fn message(&self) -> String;
    fn level(&self) -> NotificationLevel;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NotificationLevel {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Notification {
    pub message: String,
    pub level: NotificationLevel,
}

impl NotificationTrait for Notification {
    fn message(&self) -> String {
        self.message.clone()
    }
    fn level(&self) -> NotificationLevel {
        self.level
    }
}

#[component]
pub fn Notifier(#[prop(into)] notifications: Signal<Vec<Notification>>) -> impl IntoView {
    view! {
        <div class="notifier-container">
            <For
                each=move || notifications.get()
                key=|n| format!("{:?}-{}", n.level, n.message)
                children=|n| {
                    let level_class = n.level.to_string().to_lowercase();
                    view! {
                        <div class={format!("notification-toast {}", level_class)}>
                            <div class="notification-icon"></div>
                            <div class="notification-content">
                                {n.message}
                            </div>
                        </div>
                    }
                }
            />
        </div>
    }
}

impl std::fmt::Display for NotificationLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Info => write!(f, "info"),
            Self::Success => write!(f, "success"),
            Self::Warning => write!(f, "warning"),
            Self::Error => write!(f, "error"),
        }
    }
}
