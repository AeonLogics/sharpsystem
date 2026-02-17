use crate::components::{AlertModal, Notifier};
use crate::routes::HomePage;
use crate::routes::NotFoundPage;
use crate::routes::{AuthLayout, DashboardPage, LoginPage, RegisterPage};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    StaticSegment,
};
use models::system_state::SystemState;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" data-theme="light">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // Create global system state and provide it as context
    let state = RwSignal::new(SystemState::default());
    provide_context(state);

    // Session Hydration: Fetch user on mount
    let user_resource = Resource::new(|| (), |_| async move { actions::get_user().await });

    Effect::new(move |_| {
        if let Some(res) = user_resource.get() {
            state.update(|s| {
                s.auth_initialized = true;
                if let Ok(Some(user)) = res {
                    s.set_user(user);
                }
            });
        }
    });

    // Logout Action
    let logout_action = Action::new(|_| async move { actions::logout().await });

    view! {
        <Stylesheet id="leptos" href="/pkg/sharp-system.css"/>
        <Title text="Sharp System"/>

        <Router>
            <div class="app-layout">
                <Notifier />
                <AlertModal />
                <header class="main-header flex justify-between items-center px-6 py-4">
                    <div class="logo font-bold italic tracking-tighter">"SHARP // SYSTEM"</div>
                    <div class="user-status text-xs font-mono opacity-60">
                        <Suspense fallback=move || view! { <div class="guest">"STATUS // GUEST_PROBE"</div> }>
                            {move || Suspend::new(async move {
                                let res = user_resource.await;
                                if let Ok(Some(user)) = res {
                                    let avatar = user.avatar_url.clone().unwrap_or_else(|| {
                                        format!("https://ui-avatars.com/api/?name={}&background=8B5CF6&color=fff", user.system_name)
                                    });

                                    view! {
                                        <div class="logged-in flex items-center gap-3">
                                            <img src=avatar alt="User Avatar" class="size-8 rounded-lg border border-white/10 shadow-lg"/>
                                            <span>"CONNECTED // " {user.system_name.clone()}</span>
                                            <button class="text-red-400 hover:text-red-300" on:click=move |_| {
                                                logout_action.dispatch(());
                                                state.update(|s| s.logout());
                                            }>
                                                "[ DISCONNECT ]"
                                            </button>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="guest">"STATUS // GUEST_PROBE"</div>
                                    }.into_any()
                                }
                            })}
                        </Suspense>
                    </div>
                </header>
                <main>
                    <Routes fallback=|| view! { <NotFoundPage /> }>
                        <Route path=StaticSegment("") view=HomePage/>
                        <ParentRoute path=StaticSegment("auth") view=AuthLayout>
                            <Route path=StaticSegment("login") view=LoginPage/>
                            <Route path=StaticSegment("register") view=RegisterPage/>
                        </ParentRoute>
                        <Route path=StaticSegment("dashboard") view=DashboardPage/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
