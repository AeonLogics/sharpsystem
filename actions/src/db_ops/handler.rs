use models::HandlerRole;
#[cfg(feature = "ssr")]
use models::SystemError;
#[cfg(feature = "ssr")]
use sqlx::PgConnection;
#[cfg(feature = "ssr")]
use uuid::Uuid;

#[cfg(feature = "ssr")]
#[derive(Debug, Clone)]
pub struct HandlerAuthData {
    pub handler_id: Uuid,
    pub password_hash: String,
    pub user_name: String,
    pub email: String,
    pub handler_role: HandlerRole,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub preferred_theme: Option<String>,
    pub system_id: Uuid,
    pub workspace_handle: String,
    pub system_name: String,
}

#[cfg(feature = "ssr")]
pub async fn get_handler_auth_data(
    tx: &mut PgConnection,
    email: &str,
) -> Result<HandlerAuthData, SystemError> {
    let record = sqlx::query!(
        r#"
        SELECT 
            h.id as handler_id,
            h.password_hash,
            h.user_name,
            h.email,
            h.handler_role as "handler_role!: HandlerRole",
            h.avatar_url,
            h.bio,
            h.preferred_theme as "preferred_theme!",
            s.id as system_id,
            s.system_handle as "workspace_handle!",
            s.system_name as "system_name!"
        FROM handlers h
        JOIN systems s ON h.system_id = s.id
        WHERE h.email = $1
        "#,
        email
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| SystemError::database(e.to_string()))?
    .ok_or_else(|| SystemError::not_found("User not found"))?;

    Ok(HandlerAuthData {
        handler_id: record.handler_id,
        password_hash: record.password_hash,
        user_name: record.user_name,
        email: record.email,
        handler_role: record.handler_role,
        avatar_url: record.avatar_url,
        bio: record.bio,
        preferred_theme: Some(record.preferred_theme),
        system_id: record.system_id,
        workspace_handle: record.workspace_handle,
        system_name: record.system_name,
    })
}
