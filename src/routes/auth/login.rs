use leptos::prelude::*;

/// Login page component
#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <main class="auth-page">
            <div class="grain-bg"></div>

            // Decorative background elements
            <div class="gradient-orb auth-orb orb-1"></div>
            <div class="gradient-orb auth-orb orb-2"></div>

            <div class="auth-container">
                <div class="glass-card-premium auth-card">
                    <header class="auth-header scale-in">
                        <h1 class="gradient-text glow-text">"Welcome"</h1>
                        <p>"Sign in to your Sharp System terminal"</p>
                    </header>

                    <form class="auth-form fade-in">
                        <div class="form-group">
                            <label for="email">"Identity // Email Address"</label>
                            <input
                                type="email"
                                id="email"
                                name="email"
                                placeholder="you@example.com"
                                required
                            />
                        </div>

                        <div class="form-group">
                            <label for="password">"Security // Private Key"</label>
                            <input
                                type="password"
                                id="password"
                                name="password"
                                placeholder="••••••••"
                                required
                            />
                        </div>

                        <button type="submit" class="btn btn-primary btn-full glow-primary">
                            "SIGN IN"
                        </button>
                    </form>

                    <footer class="auth-footer scale-in">
                        <p>
                            "New to the system? "
                            <a href="/register">"Initialize Account"</a>
                        </p>
                        <a href="/" class="back-to-home">
                            "← Back to Terminal"
                        </a>
                    </footer>
                </div>

                // Extra UX Micro-detail
                <div class="mt-8 flex justify-between items-center px-4 opacity-50 font-mono text-[10px] uppercase tracking-widest text-slate-500">
                    <span>"Sharp // Auth-Service"</span>
                    <span>"STATUS // SECURE"</span>
                </div>
            </div>
        </main>
    }
}
