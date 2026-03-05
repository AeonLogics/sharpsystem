use crate::components::InlineLoader;
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

            <div class="dashboard-grid">
                <Suspense fallback=move || view! {
                    <div class="card glass-card p-6 border border-white/5 flex items-center justify-center">
                        <InlineLoader size_class="size-base" loading_text="Syncing Core...".to_string() />
                    </div>
                }>
                    {move || Suspend::new(async move {
                        let health = status.await;
                        view! {
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
                        }
                    })}
                </Suspense>

                // Active Users Card
                <div class="card glass-card p-6 border border-white/5 flex flex-col justify-between">
                    <div>
                        <h3 class="text-lg font-bold text-white mb-2">"Active Sessions"</h3>
                        <p class="text-sm text-gray-400">"1,204 online now"</p>
                    </div>
                    <div class="mt-4 pt-4 border-t border-[var(--border-default)]">
                        <span class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-medium bg-green-500/10 text-green-400">
                            <span class="w-1.5 h-1.5 rounded-full bg-green-400"></span>
                            "System Nominal"
                        </span>
                    </div>
                </div>

                // Compute Load Card
                <div class="card glass-card p-6 border border-white/5 flex flex-col justify-between">
                    <div>
                        <h3 class="text-lg font-bold text-white mb-2">"Inventory Value"</h3>
                        <p class="text-sm text-gray-400">"Total MSRP Locked"</p>
                    </div>
                    <div class="mt-4">
                        <span class="text-3xl font-bold tracking-tight text-white">"$0.00"</span>
                    </div>
                </div>
            </div>

            // NEW: Quick Actions Section
            <div class="mt-8">
                <h2 class="text-xl font-bold text-[var(--text-primary)] mb-4">"Quick Actions"</h2>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                    // POS Button (Primary Action)
                    <a href="/system/pos" class="group relative overflow-hidden rounded-[var(--radius-lg)] p-6 bg-gradient-to-br from-[var(--color-primary)] to-[var(--color-primary-hover)] border border-white/10 shadow-premium transition-all hover:-translate-y-1 hover:shadow-[0_8px_30px_rgb(139,92,246,0.3)]">
                        <div class="absolute top-0 right-0 -mt-4 -mr-4 w-24 h-24 bg-white/10 rounded-full blur-xl group-hover:bg-white/20 transition-all"></div>
                        <div class="relative z-10 flex flex-col gap-3">
                            <div class="w-10 h-10 rounded-full bg-white/20 flex items-center justify-center text-white backdrop-blur-sm">
                                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="10" cy="20.5" r="1"/><circle cx="18" cy="20.5" r="1"/><path d="M2.5 2.5h3l2.7 12.4a2 2 0 0 0 2 1.6h7.7a2 2 0 0 0 2-1.6l1.6-8.4H7.1"/></svg>
                            </div>
                            <h3 class="text-lg font-bold text-white">"Point of Sale"</h3>
                            <p class="text-sm text-white/80 font-medium">"Launch scanner & checkout"</p>
                        </div>
                    </a>

                    // Receive Stock Button
                    <a href="/system/inventory" class="group relative overflow-hidden rounded-[var(--radius-lg)] p-6 bg-[var(--bg-elevated)] border border-[var(--border-default)] hover:border-[var(--color-primary)]/50 transition-all shadow-sm hover:-translate-y-1 hover:shadow-md">
                        <div class="relative z-10 flex flex-col gap-3">
                            <div class="w-10 h-10 rounded-full bg-[var(--bg-subtle)] flex items-center justify-center text-[var(--color-primary)] group-hover:bg-[var(--color-primary)]/10 transition-colors">
                                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"></path><polyline points="3.27 6.96 12 12.01 20.73 6.96"></polyline><line x1="12" y1="22.08" x2="12" y2="12"></line></svg>
                            </div>
                            <h3 class="text-lg font-bold text-[var(--text-primary)] group-hover:text-[var(--color-primary)] transition-colors">"Receive Stock"</h3>
                            <p class="text-sm text-[var(--text-secondary)]">"Scan or bulk add items"</p>
                        </div>
                    </a>

                    // Catalog Button
                    <a href="/system/catalog" class="group relative overflow-hidden rounded-[var(--radius-lg)] p-6 bg-[var(--bg-elevated)] border border-[var(--border-default)] hover:border-[var(--color-primary)]/50 transition-all shadow-sm hover:-translate-y-1 hover:shadow-md">
                        <div class="relative z-10 flex flex-col gap-3">
                            <div class="w-10 h-10 rounded-full bg-[var(--bg-subtle)] flex items-center justify-center text-[var(--text-primary)] group-hover:text-[var(--color-primary)] group-hover:bg-[var(--color-primary)]/10 transition-colors">
                                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="7" height="7"></rect><rect x="14" y="3" width="7" height="7"></rect><rect x="14" y="14" width="7" height="7"></rect><rect x="3" y="14" width="7" height="7"></rect></svg>
                            </div>
                            <h3 class="text-lg font-bold text-[var(--text-primary)] group-hover:text-[var(--color-primary)] transition-colors">"Product Catalog"</h3>
                            <p class="text-sm text-[var(--text-secondary)]">"Manage products & SKUs"</p>
                        </div>
                    </a>
                </div>
            </div>
        </div>
    }
}
