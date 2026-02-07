use leptos::prelude::*;
use models::system_state::SystemState;

#[component]
pub fn Notifier() -> impl IntoView {
    let state = use_context::<RwSignal<SystemState>>().expect("SystemState context not found");

    view! {
        <div class="notifier-container">
            {move || {
                let state = state.get();
                state.notification.as_ref().map(|n| {
                    let level_class = format!("{:?}", n.level()).to_lowercase();
                    view! {
                        <div class={format!("notification-toast {}", level_class)}>
                            <div class="notification-icon"></div>
                            <div class="notification-content">
                                <span class="notification-title">{n.title()}</span>
                                <span class="notification-message">{n.message()}</span>
                            </div>
                        </div>
                    }
                })
            }}
        </div>
    }
}
