use leptos::prelude::*;
use leptos_router::components::Outlet;
use leptos_router::hooks::use_navigate;
use models::system_state::SystemState;

/// Industrial Auth Nexus - Centralized layout for authentication flows.
#[component]
pub fn AuthLayout() -> impl IntoView {
    let state = use_context::<RwSignal<SystemState>>().expect("SystemState context not found");
    if state.get().user.is_some() {
        let navigate = use_navigate();
        navigate("/dashboard", Default::default());
    }
    view! {
        <Outlet />
    }
}
