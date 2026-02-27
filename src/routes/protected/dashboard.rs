use crate::components::{InlineLoader, Loading};
use actions::check_system_health;
use leptos::prelude::*;

/// Dashboard page component (protected)
#[component]
pub fn DashboardPage() -> impl IntoView {
    let status = Resource::new(|| {}, async |_| check_system_health().await);

    view! {
        <div class="dashboard-container">
            <header class="dashboard-header">
                <h1 class="gradient-text">"Dashboard"</h1>
                <p class="text-secondary">"Welcome back!"</p>
            </header>

             <Suspense fallback=move || view! {
                <div class="p-6">
                    <InlineLoader size_class="size-base" loading_text="Syncing Core...".to_string() />
                </div>
            }>
                {move || Suspend::new(async move {
                    let health = status.await;
                    view! {
                        <div class="dashboard-grid">
                            // System Status Card
                            <div class="card glass-card p-6 border border-white/5">
                                <h3 class="text-lg font-bold text-white mb-2">"System Health"</h3>
                                {match health {
                                    Ok(msg) => view! {
                                        <p class="text-sm text-gray-400 font-mono">
                                            <span class="text-green-400">"OK "</span>
                                            "// DB_LATENCY: " {msg}
                                        </p>
                                    }.into_any(),
                                    Err(e) => view! {
                                        <p class="text-sm text-red-400 font-mono">
                                            <span class="text-red-500">"ERR "</span>
                                            "// " {e.to_string()}
                                        </p>
                                    }.into_any(),
                                }}
                            </div>
                            </div>
                        }
                    })}
                    // Active Users Card
                    </Suspense>
                    <div class="card glass-card p-6 border border-white/5">
                        <h3 class="text-lg font-bold text-white mb-2">"Active Sessions"</h3>
                        <p class="text-sm text-gray-400">"1,204 online now"</p>
                    </div>

                    // Revenue Card
                    <div class="card glass-card p-6 border border-white/5">
                        <h3 class="text-lg font-bold text-white mb-2">"Compute Load"</h3>
                        <p class="text-sm text-gray-400">"24% Capacity"</p>
                    </div>
        </div>
    }
}
