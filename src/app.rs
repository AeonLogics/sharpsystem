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
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let auth_resource = Resource::new(|| (), |_| async move { actions::validate_session().await });
    let state = RwSignal::new(SystemState::default());
    provide_context(state);

    Effect::new(move |_| {
        auth_resource.with(|res| {
            if let Some(outcome) = res {
                match outcome {
                    Ok(Some(user)) => state.update(|s| s.set_user(user.clone())),
                    Err(_e) => state.update(|s| s.set_unauthenticated()),
                    _ => state.update(|s| s.set_unauthenticated()),
                }
            }
        });
    });

    view! {
        <Stylesheet id="leptos" href="/pkg/sharp-system.css" />
        <Title text="Sharp System" />

        <Router>
            <Suspense fallback=|| {
                view! {
                    <div class="p-10 text-center font-mono opacity-50">
                        "INITIALIZING SYSTEM..."
                    </div>
                }
            }>
                <div class="app-layout">
                    <Notifier />
                    <AlertModal />
                    <Header />
                    <main>
                        <Routes fallback=|| view! { <NotFoundPage /> }>
                            <Route path=StaticSegment("") view=HomePage />
                            <ParentRoute path=StaticSegment("auth") view=AuthLayout>
                                <Route path=StaticSegment("login") view=LoginPage />
                                <Route path=StaticSegment("register") view=RegisterPage />
                            </ParentRoute>
                            <ParentRoute
                                path=StaticSegment("system")
                                view=|| {
                                    view! {
                                        <Guard>
                                            <LayoutPage />
                                        </Guard>
                                    }
                                }
                            >
                                <Route path=StaticSegment("dashboard") view=DashboardPage />
                                <Route path=StaticSegment("catalog") view=CatalogPage />
                                <Route path=StaticSegment("inventory") view=InventoryPage />
                                <Route path=StaticSegment("pos") view=PosPage />
                            </ParentRoute>
                        </Routes>
                    </main>
                </div>
            </Suspense>
        </Router>
    }
}
