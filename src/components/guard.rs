use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use models::system_state::{AuthState, SystemState};

#[component]
pub fn Guard(children: ChildrenFn) -> impl IntoView {
    let state = use_context::<RwSignal<SystemState>>().expect("SystemState context not found");
    let navigate = use_navigate();

    Effect::new(move |_| {
        if !state.get().auth_state.is_loading() && !state.get().auth_state.is_authenticated() {
            navigate("/auth/login", Default::default());
        }
    });

    move || match state.get().auth_state {
        AuthState::Authenticated(_) => children().into_any(),
        AuthState::Loading => {
            view! { <div class="p-20 text-center opacity-50">"VERIFYING SESSION..."</div> }
                .into_any()
        }
        _ => view! { <div class="p-20 text-center">"REDIRECTING..."</div> }.into_any(),
    }
}
