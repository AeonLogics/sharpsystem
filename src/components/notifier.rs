use leptos::prelude::*;
use models::system_state::SystemState;
use std::time::Duration;

#[component]
pub fn Notifier() -> impl IntoView {
    let state = use_context::<RwSignal<SystemState>>().expect("SystemState context not found");

    view! {
        <div class="notifier-container">
            <For
                each=move || state.get().toasts
                key=|toast| toast.id()
                children=move |toast| {
                    let id = toast.id();
                    let level_class = format!("{:?}", toast.level()).to_lowercase();

                    // Auto-clear logic for each individual toast
                    Effect::new(move |_| {
                        let id_clone = id.clone();
                        set_timeout(
                            move || {
                                state.update(|s| s.remove_toast(&id_clone));
                            },
                            Duration::from_secs(5),
                        );
                    });

                    view! {
                        <div class={format!("notification-toast {}", level_class)}>
                            <div class="notification-icon"></div>
                            <div class="notification-content">
                                <span class="notification-title">{toast.title()}</span>
                                <span class="notification-message">{toast.message()}</span>
                            </div>
                        </div>
                    }
                }
            />
        </div>
    }
}
