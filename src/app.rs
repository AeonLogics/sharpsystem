use crate::components::*;
use crate::routes::*;
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
        <html lang="en" data-theme="dark">
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

    view! {
        <Stylesheet id="leptos" href="/pkg/sharp-system.css"/>
        <Title text="Sharp System"/>

        <Suspense fallback=|| view! { <div class="p-10 text-center font-mono opacity-50">"INITIALIZING SYSTEM..."</div> }>
            <AppRouter/>
        </Suspense>
    }
}

#[component]
fn AppRouter() -> impl IntoView {
    view! {
        <Router>
            <AuthWrapper />
        </Router>
    }
}

#[component]
fn AuthWrapper() -> impl IntoView {
    // 1. Kick off the database check IMMEDIATELY on the server.
    let auth_resource = Resource::new(|| (), |_| async move { actions::get_user().await });

    // 2. Initialize our global state empty for now (starts as AuthState::Loading)
    let state = RwSignal::new(SystemState::default());
    provide_context(state);

    // Give Leptos an asynchronous Effect that updates state safely after render
    Effect::new(move |_| {
        auth_resource.with(|res| {
            leptos::logging::log!(
                "==== BROWSER AUTH_RESOURCE RESOLVED: {:?} ====",
                res.is_some()
            );
            if let Some(outcome) = res {
                match outcome {
                    Ok(Some(user)) => {
                        leptos::logging::log!("==== BROWSER SETTING USER AUTHENTICATED ====");
                        state.update(|s| s.set_user(user.clone()))
                    }
                    Err(e) => {
                        leptos::logging::log!(
                            "==== BROWSER SETTING USER UNAUTHENTICATED. ERROR: {:?} ====",
                            e
                        );
                        state.update(|s| s.set_unauthenticated())
                    }
                    _ => {
                        leptos::logging::log!("==== BROWSER SETTING USER UNAUTHENTICATED (Unexpected resolution) ====");
                        state.update(|s| s.set_unauthenticated())
                    }
                }
            }
        });
    });

    view! {
        <div class="app-layout">
            <Notifier />
            <AlertModal />
            <Header />
            <main>
                <Routes fallback=|| view! { <NotFoundPage /> }>
                    <Route path=StaticSegment("") view=HomePage/>
                    <ParentRoute path=StaticSegment("auth") view=AuthLayout>
                        <Route path=StaticSegment("login") view=LoginPage/>
                        <Route path=StaticSegment("register") view=RegisterPage/>
                    </ParentRoute>
                    <ParentRoute path=StaticSegment("system") view=LayoutPage>
                        <Route path=StaticSegment("dashboard") view=DashboardPage/>
                    </ParentRoute>
                </Routes>
            </main>
        </div>
    }
}
