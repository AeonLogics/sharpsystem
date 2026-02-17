use actions::auth::Signup;
use actions::is_handle_available;
use leptos::prelude::*;
use leptos_router::components::A;
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
    let navigate = leptos_router::hooks::use_navigate();

    // Form field signals
    let (user_name, set_user_name) = signal(String::new());
    let (system_name, set_system_name) = signal(String::new());
    let (workspace_handle, set_workspace_handle) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (confirm_password, set_confirm_password) = signal(String::new());
    let (show_password, set_show_password) = signal(false);

    // Handle availability action
    let handle_check = Action::new(|h: &String| {
        let h = h.clone();
        async move { is_handle_available(h).await }
    });
    let handle_available = handle_check.value();

    // Industrial-grade slug synchronization
    Effect::new(move |_| {
        let name = system_name.get();
        let mut enforced_handle = String::with_capacity(name.len());
        let mut last_was_hyphen = false;

        for c in name.to_lowercase().chars() {
            if c.is_alphanumeric() {
                enforced_handle.push(c);
                last_was_hyphen = false;
            } else if !last_was_hyphen && !enforced_handle.is_empty() {
                enforced_handle.push('-');
                last_was_hyphen = true;
            }
        }

        // Final trim of trailing hyphen
        if enforced_handle.ends_with('-') {
            enforced_handle.pop();
        }

        set_workspace_handle.set(enforced_handle);
    });

    // Check availability on synchronized change
    Effect::new(move |_| {
        let h = workspace_handle.get();
        if h.len() >= 3 {
            handle_check.dispatch(h);
        }
    });

    // Server action
    let signup_action = ServerAction::<Signup>::new();
    let signup_loading = signup_action.pending();
    let signup_value = signup_action.value();

    let next_step = move |_| {
        let current_step = step.get();

        let result = (move || {
            match current_step {
                0 => Ok(()), // Welcome step - no validation
                1 => {
                    SignupPayload::validate_system_name(&system_name.get())?;
                    SignupPayload::validate_workspace_handle(&workspace_handle.get())?;
                    if let Some(Ok(available)) = handle_available.get_untracked() {
                        if !available {
                            return Err(models::auth::AuthError::InvalidInput(
                                "This workspace handle is already taken.".to_string(),
                            ));
                        }
                    }
                    Ok(())
                }
                2 => SignupPayload::validate_email(&email.get()),
                3 => {
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
            }
        })();

        match result {
            Ok(_) => {
                if current_step < 4 {
                    set_step.update(|s| *s += 1);
                } else {
                    // Final submission
                    signup_action.dispatch(Signup {
                        payload: SignupPayload {
                            user_name: user_name.get(),
                            system_name: system_name.get(),
                            workspace_handle: workspace_handle.get(),
                            email: email.get(),
                            password: password.get(),
                            confirm_password: confirm_password.get(),
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
                s.add_toast(Arc::new(e));
            });
        }
    });

    // Watch for success
    Effect::new(move |_| {
        if let Some(Ok(user)) = signup_value.get() {
            state.update(|s| {
                s.set_user(user);
                s.add_toast(Arc::new(Notification::new(
                    "System Initialized",
                    "Security protocols active. Welcome to the grid.",
                    NotificationLevel::Success,
                )));
            });
            navigate("/dashboard", Default::default());
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
                            <span class="step-number">"0" {move || step.get() + 1} " // 05"</span>
                            <div class="progress-bar-track">
                                <div
                                    class="progress-bar-fill"
                                    style=move || {
                                        let p = (step.get() as f32 + 1.0) / 5.0 * 100.0;
                                        format!("width: {}%", p)
                                    }
                                ></div>
                            </div>
                        </div>
                        <h1 class="gradient-text glow-text">
                            {move || match step.get() {
                                0 => "Bootstrap",
                                1 => "Organization",
                                2 => "Protocol",
                                3 => "Security",
                                _ => "Verification",
                            }}
                        </h1>
                        <p>
                            {move || match step.get() {
                                0 => "System authentication required to access the network.",
                                1 => "Initialize your system workspace.",
                                2 => "Configure your neural communication link.",
                                3 => "Encrypt your private access keys.",
                                _ => "Final system entry authorization.",
                            }}
                        </p>
                    </header>

                    <div class="auth-form wizard-form">
                        <form method="post" on:submit=move |ev| ev.prevent_default()>
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
                                            <label for="user_name">"Identity // Your Full Name"</label>
                                            <input
                                                type="text"
                                                id="user_name"
                                                placeholder="e.g. John Doe"
                                                on:input=move |ev| set_user_name.set(event_target_value(&ev))
                                                prop:value=user_name
                                                required
                                            />

                                            <label for="system_name" class="mt-4">"Organization // Display Name"</label>
                                            <input
                                                type="text"
                                                id="system_name"
                                                placeholder="e.g. Sharp System Inc."
                                                on:input=move |ev| set_system_name.set(event_target_value(&ev))
                                                prop:value=system_name
                                                required
                                            />

                                            <label for="workspace_handle" class="mt-4">"Subdomain // Workspace Handle"</label>
                                            <div class="handle-input-wrapper flex items-center gap-2">
                                                <input
                                                    type="text"
                                                    id="workspace_handle"
                                                    placeholder="Automated..."
                                                    readonly
                                                    prop:value=workspace_handle
                                                    class=move || {
                                                        let base = "bg-slate-900/10 cursor-not-allowed opacity-70 ";
                                                        let status = match handle_available.get() {
                                                            Some(Ok(true)) => "border-green-500/50",
                                                            Some(Ok(false)) => "border-red-500/50",
                                                            _ => ""
                                                        };
                                                        format!("{}{}", base, status)
                                                    }
                                                />
                                                <span class="text-xs font-mono opacity-50">".sharpsystem.com"</span>
                                            </div>
                                            <div class="mt-1 text-[10px] font-mono uppercase tracking-tighter">
                                                {move || {
                                                    let handle = workspace_handle.get();
                                                    if handle.len() < 3 {
                                                         view! { <span class="opacity-30">"Minimum 3 characters"</span> }.into_any()
                                                    } else {
                                                        match SignupPayload::validate_workspace_handle(&handle) {
                                                            Err(models::auth::AuthError::InvalidInput(e)) if e.contains("reserved") => {
                                                                view! { <span class="text-red-500">"✗ Reserved Handle"</span> }.into_any()
                                                            },
                                                            _ => match handle_available.get() {
                                                                Some(Ok(true)) => view! { <span class="text-green-500">"✓ Available"</span> }.into_any(),
                                                                Some(Ok(false)) => view! { <span class="text-red-500">"✗ Taken"</span> }.into_any(),
                                                                Some(Err(_)) => view! { <span class="text-yellow-500">"Error checking availability"</span> }.into_any(),
                                                                None => view! { <span class="animate-pulse">"Checking..."</span> }.into_any(),
                                                            }
                                                        }
                                                    }
                                                }}
                                            </div>
                                        </div>
                                    }.into_any(),
                                    2 => view! {
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
                                    3 => view! {
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
                                                <span class="label">"USER"</span>
                                                <span class="value">{user_name}</span>
                                            </div>
                                            <div class="review-item">
                                                <span class="label">"ORG"</span>
                                                <span class="value">{system_name}</span>
                                            </div>
                                            <div class="review-item">
                                                <span class="label">"HANDLE"</span>
                                                <span class="value">{workspace_handle}</span>
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
                                    <button
                                        type="button"
                                        class="btn btn-ghost glass-btn flex-1"
                                        on:click=prev_step
                                    >
                                        "PREVIOUS"
                                    </button>
                                })}
                                <button
                                    type="button"
                                    class="btn btn-primary glow-primary flex-[2]"
                                    on:click=next_step
                                    disabled=move || signup_loading.get() || (step.get() == 1 && (workspace_handle.get().len() < 3 || matches!(handle_available.get(), Some(Ok(false)))))
                                >
                                    {move || {
                                        if signup_loading.get() {
                                            "INITIALIZING..."
                                        } else if step.get() == 0 {
                                            "START INITIALIZATION"
                                        } else if step.get() < 4 {
                                            "CONTINUE"
                                        } else {
                                            "AUTHORIZE SYSTEM"
                                        }
                                    }}
                                </button>
                            </div>
                        </form>
                    </div>

                    <footer class="auth-footer scale-in mt-12">
                        <p>
                            "Already authorized? "
                            <A href="/auth/login">"Enter System"</A>
                        </p>
                        <A href="/" attr:class="back-to-home">
                            "← Back to Terminal"
                        </A>
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
