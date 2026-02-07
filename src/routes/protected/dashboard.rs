use leptos::prelude::*;

/// Dashboard page component (protected)
#[component]
pub fn DashboardPage() -> impl IntoView {
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
            </div>
        </div>
    }
}
