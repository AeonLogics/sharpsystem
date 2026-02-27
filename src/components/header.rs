use leptos::prelude::*;
use models::system_state::{AuthState, SystemState};

#[component]
pub fn Header() -> impl IntoView {
    let state =
        use_context::<RwSignal<SystemState>>().expect("SystemState context should be provided");
    let logout_action = Action::new(|_| async move { actions::logout().await });

    view! {
        <header class="app-header">
            <div class="header-container">
                // Left Side: Logo & Brand
                <div class="header-brand">
                    <div class="logo-mark"></div>
                    <span class="logo-text">"SHARP"<span class="logo-accent">"SYSTEM"</span></span>
                </div>

                // Right Side: User Controls
                <div class="header-controls">
                    {move || match state.get().auth_state {
                        AuthState::Loading => view! {
                            <div class="guest-badge badge opacity-50">
                                <span class="status-dot"></span>
                                "Loading..."
                            </div>
                        }
                        .into_any(),

                        AuthState::Authenticated(user) => {
                            let avatar = user.avatar_url.clone().unwrap_or_else(|| {
                                format!(
                                    "https://ui-avatars.com/api/?name={}&background=8B5CF6&color=fff",
                                    user.system_name
                                )
                            });

                            view! {
                                <div class="user-profile-widget">
                                    <div class="user-info">
                                        <span class="tenant-name">{user.system_name.clone()}</span>
                                        <span class="connection-status">
                                            <span class="status-dot online"></span>
                                            "Connected"
                                        </span>
                                    </div>
                                    <img src=avatar alt="User Avatar" class="user-avatar"/>

                                    <button class="disconnect-btn" on:click=move |_| {
                                        logout_action.dispatch(());
                                        state.update(|s| s.logout());
                                    }>
                                        <i class="icon-power"></i>
                                    </button>
                                </div>
                            }
                            .into_any()
                        }

                        AuthState::Unauthenticated => view! {
                            <div class="guest-badge badge">
                                <span class="status-dot offline"></span>
                                "Offline"
                            </div>
                        }
                        .into_any(),
                    }}
                </div>
            </div>
        </header>
    }
}
