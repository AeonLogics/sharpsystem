use leptos::prelude::*;

/// Register page component
#[component]
pub fn RegisterPage() -> impl IntoView {
    view! {
        <main class="auth-page">
            <div class="grain-bg"></div>

            // Decorative background elements
            <div class="gradient-orb auth-orb orb-1"></div>
            <div class="gradient-orb auth-orb orb-2"></div>

            <div class="auth-container">
                <div class="glass-card-premium auth-card">
                    <header class="auth-header scale-in">
                        <h1 class="gradient-text glow-text">"Initialize"</h1>
                        <p>"Secure your entry into the Sharp System"</p>
                    </header>

                    <form class="auth-form fade-in">
                        <div class="form-group">
                            <label for="full_name">"Identity // Full Name"</label>
                            <input
                                type="text"
                                id="full_name"
                                name="full_name"
                                placeholder="e.g. Alexander Hamilton"
                                required
                            />
                        </div>

                        <div class="form-group">
                            <label for="email">"Protocol // Email Address"</label>
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

                        <div class="form-group">
                            <label for="confirm_password">"Verification // Confirm Key"</label>
                            <input
                                type="password"
                                id="confirm_password"
                                name="confirm_password"
                                placeholder="••••••••"
                                required
                            />
                        </div>

                        <button type="submit" class="btn btn-primary btn-full glow-primary">
                            "CREATE ACCOUNT"
                        </button>
                    </form>

                    <footer class="auth-footer scale-in">
                        <p>
                            "Already authorized? "
                            <a href="/login">"Enter System"</a>
                        </p>
                        <a href="/" class="back-to-home">
                            "← Back to Terminal"
                        </a>
                    </footer>
                </div>

                // Extra UX Micro-detail
                <div class="mt-8 flex justify-between items-center px-4 opacity-50 font-mono text-[10px] uppercase tracking-widest text-slate-500">
                    <span>"Sharp // Auth-Service"</span>
                    <span>"ENC // RSA-4096"</span>
                </div>
            </div>
        </main>
    }
}
