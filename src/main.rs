#![recursion_limit = "512"]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use sharp_system::app::*;
    use tower_sessions::{Expiry, SessionManagerLayer};
    use tower_sessions_sqlx_store::PostgresStore;

    // Initialize tracing with env-filter to catch sqlx errors
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                tracing_subscriber::EnvFilter::new(
                    "info,sqlx=debug,hyper=warn,tower=warn,axum=warn",
                )
            }),
        )
        .init();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);
    println!(
        "GENERATED ROUTES: {:?}",
        routes.iter().map(|r| r.path()).collect::<Vec<_>>()
    );

    #[cfg(feature = "ssr")]
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/sharp_system".to_string());

    let pool_result = sqlx::postgres::PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(&database_url)
        .await;

    let pool = match pool_result {
        Ok(p) => {
            sqlx::migrate!("./migrations")
                .run(&p)
                .await
                .expect("Failed to apply database migrations");
            Some(p)
        }
        Err(e) => {
            log!("Warning: Could not connect to database: {}. System functions depending on the database will fail.", e);
            None
        }
    };

    let session_store = PostgresStore::new(pool.clone().unwrap())
        .with_table_name("tower_sessions")
        .expect("Invalid table name");
    session_store.migrate().await.expect("Migration failed");

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_http_only(true)
        .with_expiry(Expiry::OnInactivity(time::Duration::days(7)));
    use axum::http::header::CACHE_CONTROL;
    use tower_http::{compression::CompressionLayer, set_header::SetResponseHeaderLayer};

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            {
                let pool = pool.clone();
                move || {
                    if let Some(pool) = pool.clone() {
                        provide_context(pool);
                    }
                }
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .layer(SetResponseHeaderLayer::if_not_present(
            CACHE_CONTROL,
            axum::http::HeaderValue::from_static("no-cache, no-store, must-revalidate"),
        ))
        .fallback(leptos_axum::file_and_error_handler_with_context(
            {
                let pool = pool.clone();
                move || {
                    if let Some(pool) = pool.clone() {
                        provide_context(pool);
                    }
                }
            },
            shell,
        ))
        .layer(CompressionLayer::new())
        .layer(session_layer)
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
