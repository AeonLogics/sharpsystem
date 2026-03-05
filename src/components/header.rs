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

                // Middle: Main Navigation
                {move || match state.get().auth_state {
                    AuthState::Authenticated(_) => view! {
                        <nav class="header-nav flex gap-6 text-sm font-medium">
                            <a href="/system/dashboard" class="text-[var(--text-secondary)] hover:text-[var(--text-primary)] transition-colors">"Dashboard"</a>
                            <a href="/system/pos" class="text-[var(--color-primary)] hover:text-[var(--color-primary-hover)] font-bold transition-colors flex items-center gap-1">
                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="10" cy="20.5" r="1"/><circle cx="18" cy="20.5" r="1"/><path d="M2.5 2.5h3l2.7 12.4a2 2 0 0 0 2 1.6h7.7a2 2 0 0 0 2-1.6l1.6-8.4H7.1"/></svg>
                                "POS"
                            </a>
                            <a href="/system/inventory" class="text-[var(--text-secondary)] hover:text-[var(--text-primary)] transition-colors">"Inventory"</a>
                            <a href="/system/catalog" class="text-[var(--text-secondary)] hover:text-[var(--text-primary)] transition-colors">"Catalog"</a>
                        </nav>
                    }.into_any(),
                    _ => view! { <div></div> }.into_any(),
                }}

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
                                format!("https://api.dicebear.com/7.x/initials/svg?seed={}", user.email)
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
