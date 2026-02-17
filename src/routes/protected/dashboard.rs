use actions::check_system_health;
use leptos::prelude::*;
use models::system_state::SystemState;

/// Dashboard page component (protected)
#[component]
pub fn DashboardPage() -> impl IntoView {
    let state =
        use_context::<RwSignal<SystemState>>().expect("SystemState context should be provided");
    let navigate = leptos_router::hooks::use_navigate();

    let status = Resource::new(|| {}, async |_| check_system_health().await);

    view! {
        {move || {
            let s = state.get();
            if !s.auth_initialized {
                view! { <div class="p-10 text-center opacity-50 font-mono">"AUTHENTICATING..."</div> }.into_any()
            } else if s.user.is_none() {
                navigate("/auth/login", Default::default());
                view! { <div class="p-10 text-center text-red-400 font-mono">"UNAUTHORIZED // REDIRECTING"</div> }.into_any()
            } else {
                view! {
                    <div class="dashboard-container">
                        <header class="dashboard-header">
                            <h1 class="gradient-text">"Dashboard"</h1>
                            <p class="text-secondary">"Welcome back!"</p>
                        </header>

                        <div class="dashboard-grid">
                            <div class="card">
                                <h3>"Overview"</h3>
                                <p class="text-muted">"Your activity summary"</p>
                            </div>

                            <div class="card">
                                <h3>"Statistics"</h3>
                                <p class="text-muted">"Performance metrics"</p>
                            </div>

                            <div class="card">
                                <h3>"Recent Activity"</h3>
                                <p class="text-muted">"Latest updates"</p>
                            </div>

                            <div class="card">
                                <h3>"System Status"</h3>
                                    {move || {
                                        status.get().map(|res| match res {
                                            Ok(status) => view! { <p>{status}</p> }.into_any(),
                                            Err(e) => view! { <p class="text-danger">{e.to_string()}</p> }.into_any(),
                                        })
                                    }}
                            </div>
                        </div>
                    </div>
                }.into_any()
            }
        }}
    }
}
