use leptos::prelude::*;
#[cfg(feature = "ssr")]
use sqlx::PgPool;

#[server]
pub async fn check_system_health() -> Result<String, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    Ok("I am working ok".to_string())
}

#[server]
pub async fn is_handle_available(handle: String) -> Result<bool, ServerFnError> {
    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Database connection pool not found in context."))?;

    let exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM systems WHERE system_handle = $1)")
            .bind(handle)
            .fetch_one(&pool)
            .await
            .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    Ok(!exists)
}
#[server]
pub async fn is_email_available(email: String) -> Result<bool, ServerFnError> {
    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Database connection pool not found in context."))?;

    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM handlers WHERE email = $1)")
        .bind(email)
        .fetch_one(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    Ok(!exists)
}
