use actions::auth::Login;
use leptos::prelude::*;
use leptos_router::components::A;
use models::shared::notifications::{Notification, NotificationLevel};
use models::system_state::SystemState;
use models::LoginPayload;
use std::sync::Arc;

/// Login page component
#[component]
pub fn LoginPage() -> impl IntoView {
    let state =
        use_context::<RwSignal<SystemState>>().expect("SystemState context should be provided");
    let navigate = leptos_router::hooks::use_navigate();

    // Form field signals
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());

    // Server action
    let login_action = ServerAction::<Login>::new();
    let login_loading = login_action.pending();
    let login_value = login_action.value();

    // Handle form submission
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        login_action.dispatch(Login {
            payload: LoginPayload {
                email: email.get(),
                password: password.get(),
            },
        });
    };

    // Watch for server errors
    Effect::new(move |_| {
        if let Some(Err(e)) = login_value.get() {
            state.update(|s| {
                s.add_toast(Arc::new(e));
            });
        }
    });

    // Watch for success
    Effect::new(move |_| {
        if let Some(Ok(user)) = login_value.get() {
            state.update(|s| {
                s.set_user(user);
                s.add_toast(Arc::new(Notification::new(
                    "Access Authorized",
                    "Welcome back to the grid.",
                    NotificationLevel::Success,
                )));
            });
            navigate("/dashboard", Default::default());
        }
    });

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

                    <form class="auth-form fade-in" on:submit=on_submit>
                        <div class="form-group">
                            <label for="email">"Identity // Email Address"</label>
                            <input
                                type="email"
                                id="email"
                                name="email"
                                placeholder="you@example.com"
                                on:input=move |ev| set_email.set(event_target_value(&ev))
                                prop:value=email
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
                                on:input=move |ev| set_password.set(event_target_value(&ev))
                                prop:value=password
                                required
                            />
                        </div>

                        <button
                            type="submit"
                            class="btn btn-primary btn-full glow-primary"
                            disabled=login_loading
                        >
                            {move || if login_loading.get() { "AUTHORIZING..." } else { "SIGN IN" }}
                        </button>
                    </form>

                    <footer class="auth-footer scale-in">
                        <p>
                            "New to the system? "
                            <A href="/auth/register">"Initialize Account"</A>
                        </p>
                        <A href="/" attr:class="back-to-home">
                            "← Back to Terminal"
                        </A>
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
