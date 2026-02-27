use leptos::prelude::*;
use leptos_router::components::Outlet;
use models::system_state::{AuthState, SystemState};

#[component]
pub fn LayoutPage() -> impl IntoView {
    let state = use_context::<RwSignal<SystemState>>().expect("SystemState context not found");

    Effect::new(move |_| {
        if let AuthState::Unauthenticated = state.get().auth_state {
            let navigate = leptos_router::hooks::use_navigate();
            navigate("/auth/login", Default::default());
        }
    });

    view! {
        <Outlet />
    }
}
