use actions::auth::Signup;
use leptos::prelude::*;
use models::shared::notifications::{Notification, NotificationLevel};
use models::system_state::SystemState;
use models::SignupPayload;
use std::sync::Arc;

/// Register page component
#[allow(non_snake_case)]
#[component]
pub fn RegisterPage() -> impl IntoView {
    let (step, set_step) = signal(0);
    let state = use_context::<RwSignal<SystemState>>().expect("SystemState context not found");

    // Form field signals
    let (username, set_username) = signal(String::new());
    let (company_name, set_company_name) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (confirm_password, set_confirm_password) = signal(String::new());
    let (show_password, set_show_password) = signal(false);

    // Server action
    let signup_action = ServerAction::<Signup>::new();
    let signup_loading = signup_action.pending();
    let signup_value = signup_action.value();

    let next_step = move |_| {
        let current_step = step.get();

        let result = match current_step {
            0 => Ok(()), // Welcome step - no validation
            1 => SignupPayload::validate_company_name(&company_name.get()),
            2 => SignupPayload::validate_username(&username.get()),
            3 => SignupPayload::validate_email(&email.get()),
            4 => {
                let p = password.get();
                let cp = confirm_password.get();
                if p != cp {
                    Err(models::auth::AuthError::InvalidInput(
                        "Passwords do not match.".to_string(),
                    ))
                } else {
                    SignupPayload::validate_password(&p)
                }
            }
            _ => Ok(()),
        };

        match result {
            Ok(_) => {
                if current_step < 5 {
                    set_step.update(|s| *s += 1);
                } else {
                    // Final submission
                    signup_action.dispatch(Signup {
                        payload: SignupPayload {
                            username: username.get(),
                            company_name: company_name.get(),
                            email: email.get(),
                            password: password.get(),
                        },
                    });
                }
            }
            Err(e) => {
                state.update(|s| {
                    s.add_toast(Arc::new(Notification::new(
                        "Data Requirement",
                        e.to_string(),
                        NotificationLevel::Error,
                    )));
                });
            }
        }
    };

    let prev_step = move |_| {
        if step.get() > 0 {
            set_step.update(|s| *s -= 1);
        }
    };

    // Watch for server errors and trigger toasts
    Effect::new(move |_| {
        if let Some(Err(e)) = signup_value.get() {
            state.update(|s| {
                s.add_toast(Arc::new(Notification::new(
                    "System Conflict",
                    e.to_string(),
                    NotificationLevel::Error,
                )));
            });
        }
    });

    // Watch for success
    Effect::new(move |_| {
        if let Some(Ok(_)) = signup_value.get() {
            state.update(|s| {
                s.add_toast(Arc::new(Notification::new(
                    "System Initialized",
                    "Security protocols active. Welcome to the grid.",
                    NotificationLevel::Success,
                )));
            });
        }
    });

    view! {
        <main class="auth-page wizard-mode">
            <div class="grain-bg"></div>

            // Decorative background elements
            <div class="gradient-orb auth-orb orb-1"></div>
            <div class="gradient-orb auth-orb orb-2"></div>

            <div class="auth-container">
                <div class="glass-card-premium auth-card">
                    <header class="auth-header scale-in">
                        <div class="step-indicator">
                            <span class="step-number">"0" {move || step.get() + 1} " // 06"</span>
                            <div class="progress-bar-track">
                                <div
                                    class="progress-bar-fill"
                                    style=move || {
                                        let p = (step.get() as f32 + 1.0) / 6.0 * 100.0;
                                        format!("width: {}%", p)
                                    }
                                ></div>
                            </div>
                        </div>
                        <h1 class="gradient-text glow-text">
                            {move || match step.get() {
                                0 => "Bootstrap",
                                1 => "Organization",
                                2 => "Identity",
                                3 => "Protocol",
                                4 => "Security",
                                _ => "Verification",
                            }}
                        </h1>
                        <p>
                            {move || match step.get() {
                                0 => "System authentication required to access the network.",
                                1 => "Initialize your system workspace.",
                                2 => "Establish your individual system alias.",
                                3 => "Configure your neural communication link.",
                                4 => "Encrypt your private access keys.",
                                _ => "Final system entry authorization.",
                            }}
                        </p>
                    </header>

                    <div class="auth-form wizard-form">


                        <div class="wizard-step-container">
                            {move || match step.get() {
                                0 => view! {
                                    <div class="welcome-step slide-in text-center py-6">
                                        <div class="welcome-icon mb-6 scale-in">
                                            <div class="pulsing-shield">
                                                <span class="icon">"◈"</span>
                                            </div>
                                        </div>
                                        <h2 class="text-xl font-bold mb-4">"Ready to Initialize?"</h2>
                                        <p class="text-muted mb-8 italic">
                                            "You are about to establish a new node in the Sharp System network. Ensure your credentials are ready for encryption."
                                        </p>
                                        <div class="system-stats flex justify-center gap-8 text-xs font-mono opacity-60">
                                            <div class="stat">"STATE: STANDBY"</div>
                                            <div class="stat">"LATENCY: 0.4ms"</div>
                                            <div class="stat">"NODE: PHOENIX"</div>
                                        </div>
                                    </div>
                                }.into_any(),
                                1 => view! {
                                    <div class="form-group slide-in">
                                        <label for="company_name">"Organization // Company Name"</label>
                                        <input
                                            type="text"
                                            id="company_name"
                                            placeholder="e.g. Sharp System Inc."
                                            on:input=move |ev| set_company_name.set(event_target_value(&ev))
                                            prop:value=company_name
                                            required
                                        />
                                    </div>
                                }.into_any(),
                                2 => view! {
                                    <div class="form-group slide-in">
                                        <label for="username">"Identity // Full Name"</label>
                                        <input
                                            type="text"
                                            id="username"
                                            placeholder="e.g. Alexander Hamilton"
                                            on:input=move |ev| set_username.set(event_target_value(&ev))
                                            prop:value=username
                                            required
                                        />
                                    </div>
                                }.into_any(),
                                3 => view! {
                                    <div class="form-group slide-in">
                                        <label for="email">"Protocol // Email Address"</label>
                                        <input
                                            type="email"
                                            id="email"
                                            placeholder="you@example.com"
                                            on:input=move |ev| set_email.set(event_target_value(&ev))
                                            prop:value=email
                                            required
                                        />
                                    </div>
                                }.into_any(),
                                4 => view! {
                                    <div class="form-group slide-in">
                                        <label for="password">"Security // Private Key"</label>
                                        <div class="password-input-wrapper">
                                            <input
                                                type=move || if show_password.get() { "text" } else { "password" }
                                                id="password"
                                                placeholder="••••••••"
                                                on:input=move |ev| set_password.set(event_target_value(&ev))
                                                prop:value=password
                                                required
                                            />
                                            <button
                                                type="button"
                                                class="password-toggle"
                                                on:click=move |_| set_show_password.update(|v| *v = !*v)
                                                title="Toggle Visibility"
                                            >
                                                {move || if show_password.get() { "" } else { "" }}
                                            </button>
                                        </div>

                                        <label for="confirm_password" class="mt-4">"Verification // Confirm Key"</label>
                                        <div class="password-input-wrapper">
                                            <input
                                                type=move || if show_password.get() { "text" } else { "password" }
                                                id="confirm_password"
                                                placeholder="••••••••"
                                                on:input=move |ev| set_confirm_password.set(event_target_value(&ev))
                                                prop:value=confirm_password
                                                required
                                            />
                                        </div>
                                    </div>
                                }.into_any(),
                                _ => view! {
                                    <div class="review-step slide-in">
                                        <div class="review-item">
                                            <span class="label">"ORG"</span>
                                            <span class="value">{company_name}</span>
                                        </div>
                                        <div class="review-item">
                                            <span class="label">"ID"</span>
                                            <span class="value">{username}</span>
                                        </div>
                                        <div class="review-item">
                                            <span class="label">"COMMS"</span>
                                            <span class="value">{email}</span>
                                        </div>
                                    </div>
                                }.into_any(),
                            }}
                        </div>

                        <div class="wizard-actions mt-10 flex gap-4">
                            {move || (step.get() > 0).then(|| view! {
                                <button class="btn btn-ghost glass-btn flex-1" on:click=prev_step>
                                    "PREVIOUS"
                                </button>
                            })}
                            <button
                                class="btn btn-primary glow-primary flex-[2]"
                                on:click=next_step
                                disabled=signup_loading
                            >
                                {move || {
                                    if signup_loading.get() {
                                        "INITIALIZING..."
                                    } else if step.get() == 0 {
                                        "START INITIALIZATION"
                                    } else if step.get() < 5 {
                                        "CONTINUE"
                                    } else {
                                        "AUTHORIZE SYSTEM"
                                    }
                                }}
                            </button>
                        </div>
                    </div>

                    <footer class="auth-footer scale-in mt-12">
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
                    <span>"Sharp // Initialization-Wizard"</span>
                    <span>"ENC // AES-256-GCM"</span>
                </div>
            </div>
        </main>
    }
}
