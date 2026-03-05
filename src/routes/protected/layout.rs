use leptos::prelude::*;
use leptos_router::components::Outlet;
use models::system_state::{AuthState, SystemState};

#[component]
pub fn LayoutPage() -> impl IntoView {
    let state = use_context::<RwSignal<SystemState>>().expect("SystemState context not found");

    Effect::new(move |_| {
        let current_state = state.get();
        match current_state.auth_state {
            AuthState::Unauthenticated => {
                let navigate = leptos_router::hooks::use_navigate();
                navigate("/auth/login", Default::default());
            }
            AuthState::Authenticated(user) => {
                // If there's a subdomain in the URL, verify it belongs to this user!
                if let Some(active_handle) = &current_state.active_workspace_handle {
                    if active_handle != &user.workspace_handle {
                        // The user tried to access a workspace they don't own!
                        // Redirect them to the absolute base domain safely.
                        // In local dev, we check window location to strip the subdomain.
                        #[cfg(feature = "hydrate")]
                        if let Some(window) = web_sys::window() {
                            let hostname = window.location().hostname().unwrap_or_default();

                            // If we are on `fake.localhost`, redirect to `localhost`
                            if hostname.ends_with(".localhost") {
                                let _ = window
                                    .location()
                                    .set_href("http://localhost:3000/auth/login");
                            }
                            // If we are on `fake.sharpsystem.app`, redirect to `sharpsystem.app`
                            else if hostname.ends_with(".sharpsystem.app") {
                                let _ = window
                                    .location()
                                    .set_href("https://sharpsystem.app/auth/login");
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    });

    view! {
        <Outlet />
    }
}
