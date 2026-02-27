use leptos::prelude::*;
use leptos_router::components::Outlet;
use leptos_router::hooks::use_navigate;
use models::system_state::{AuthState, SystemState};

/// Industrial Auth Nexus - Centralized layout for authentication flows.
#[component]
pub fn AuthLayout() -> impl IntoView {
    let state = use_context::<RwSignal<SystemState>>().expect("SystemState context not found");

    Effect::new(move |_| {
        if let AuthState::Authenticated(_) = state.get().auth_state {
            let navigate = use_navigate();
            navigate("/system/dashboard", Default::default());
        }
    });
    view! {
        <Outlet />
    }
}
