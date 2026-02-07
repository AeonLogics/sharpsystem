use crate::routes::HomePage;
use crate::routes::NotFoundPage;
use crate::routes::{DashboardPage, LoginPage, RegisterPage};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

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
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/sharp-system.css"/>

        // sets the document title
        <Title text="Sharp System"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| view! { <NotFoundPage /> }>
                    <Route path=StaticSegment("") view=HomePage/>
                    // Auth routes (public)
                    <Route path=StaticSegment("login") view=LoginPage/>
                    <Route path=StaticSegment("register") view=RegisterPage/>
                    // Protected routes
                    <Route path=StaticSegment("dashboard") view=DashboardPage/>

                </Routes>
            </main>
        </Router>
    }
}
