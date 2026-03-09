#[cfg(feature = "ssr")]
use argon2::{Argon2, PasswordHasher};
#[cfg(feature = "ssr")]
use models::{HandlerRole, RegisterWorkspacePayload, SystemError};
#[cfg(feature = "ssr")]
use password_hash::SaltString;
#[cfg(feature = "ssr")]
use rand_core::OsRng;
#[cfg(feature = "ssr")]
use sqlx::PgConnection;
#[cfg(feature = "ssr")]
use uuid::Uuid;

#[cfg(feature = "ssr")]
use tracing::instrument;
#[cfg(feature = "ssr")]
pub async fn create_system(
    tx: &mut PgConnection,
    payload: &RegisterWorkspacePayload,
    owner_id: &Uuid,
) -> Result<Uuid, SystemError> {
    let avatar_url = format!(
        "https://api.dicebear.com/7.x/initials/svg?seed={}",
        payload.system_name
    );
    let id = sqlx::query_scalar!(
        "INSERT INTO systems (owner_id, system_handle, system_name, avatar_url) VALUES ($1, $2, $3, $4) RETURNING id",
        *owner_id,
        payload.workspace_handle,
        payload.system_name,
        avatar_url
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| SystemError::database(e.to_string()))?;

    Ok(id)
}

#[cfg(feature = "ssr")]
pub async fn create_handler(
    tx: &mut PgConnection,
    system_id: &Uuid,
    payload: &RegisterWorkspacePayload,
    password_hash: &str,
    role: HandlerRole,
) -> Result<Uuid, SystemError> {
    let id = sqlx::query_scalar!(
        "INSERT INTO handlers 
        (system_id, email, password_hash, user_name, handler_role)
         VALUES ($1, $2, $3, $4, $5::public.handler_role) 
         RETURNING id",
        system_id,
        payload.email,
        password_hash,
        payload.user_name,
        role as _
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| SystemError::database(e.to_string()))?;

    Ok(id)
}

#[cfg(feature = "ssr")]
use models::entities::User;

#[cfg(feature = "ssr")]
pub struct SessionBundle {
    pub token: String,
    pub user: User,
}

#[cfg(feature = "ssr")]
pub async fn create_session(
    tx: &mut PgConnection,
    data: &crate::db_ops::handler::HandlerAuthData,
    token: &str,
) -> Result<SessionBundle, SystemError> {
    struct SessionRecord {
        token: String,
        handler_id: Uuid,
        system_id: Uuid,
        email: String,
        user_name: String,
        handler_role: HandlerRole,
        avatar_url: Option<String>,
        bio: Option<String>,
        preferred_theme: Option<String>,
        system_handle: String,
        system_name: String,
    }

    let record = sqlx::query_as!(
        SessionRecord,
        r#"INSERT INTO sessions 
        (handler_id, system_id, token, handler_role, user_name, email, avatar_url, bio, preferred_theme, system_handle, system_name) 
        VALUES ($1, $2, $3, $4::public.handler_role, $5, $6, $7, $8, $9, $10, $11) 
        RETURNING token as "token!", handler_id as "handler_id!", system_id as "system_id!", email as "email!", user_name as "user_name!", handler_role as "handler_role!: HandlerRole", avatar_url, bio, preferred_theme, system_handle as "system_handle!", system_name as "system_name!""#,
        data.handler_id,
        data.system_id,
        token,
        data.handler_role as _,
        data.user_name,
        data.email,
        data.avatar_url,
        data.bio,
        data.preferred_theme,
        data.workspace_handle,
        data.system_name
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| SystemError::database(e.to_string()))?;

    Ok(SessionBundle {
        token: record.token,
        user: User {
            id: record.handler_id,
            system_id: record.system_id,
            email: record.email,
            user_name: record.user_name,
            handler_role: record.handler_role,
            workspace_handle: record.system_handle,
            system_name: record.system_name,
            avatar_url: record.avatar_url,
            bio: record.bio,
            preferred_theme: record.preferred_theme,
        },
    })
}

#[cfg(feature = "ssr")]
#[instrument(skip_all)]
pub async fn get_session_user(
    tx: &mut PgConnection,
    token: &str,
) -> Result<Option<User>, SystemError> {
    struct UserRecord {
        handler_id: Uuid,
        system_id: Uuid,
        email: String,
        user_name: String,
        handler_role: HandlerRole,
        avatar_url: Option<String>,
        bio: Option<String>,
        preferred_theme: Option<String>,
        system_handle: String,
        system_name: String,
    }

    let record = sqlx::query_as!(
        UserRecord,
        "SELECT handler_id as \"handler_id!\", system_id as \"system_id!\", email as \"email!\", user_name as \"user_name!\", handler_role as \"handler_role!: HandlerRole\", avatar_url, bio, preferred_theme, system_handle as \"system_handle!\", system_name as \"system_name!\" 
         FROM sessions 
         WHERE token = $1 AND expires_at > NOW()",
        token
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| SystemError::database(e.to_string()))?;

    match record {
        Some(r) => Ok(Some(User {
            id: r.handler_id,
            system_id: r.system_id,
            email: r.email,
            user_name: r.user_name,
            handler_role: r.handler_role,
            workspace_handle: r.system_handle,
            system_name: r.system_name,
            avatar_url: r.avatar_url,
            bio: r.bio,
            preferred_theme: r.preferred_theme,
        })),
        None => Ok(None),
    }
}

#[cfg(feature = "ssr")]
pub async fn delete_session(tx: &mut PgConnection, token: &str) -> Result<(), SystemError> {
    sqlx::query!("DELETE FROM sessions WHERE token = $1", token)
        .execute(&mut *tx)
        .await
        .map_err(|e| SystemError::database(e.to_string()))?;

    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn refresh_session_expiry(tx: &mut PgConnection, token: &str) -> Result<(), SystemError> {
    // ◈ Optimization: Conditional Refresh (The "Lazier Refresh" pattern)
    // We only update the expires_at timestamp if it has been at least 24 hours since the last refresh.
    // This prevents row-locking contention during rapid-fire requests while keeping sessions alive.
    sqlx::query!(
        "UPDATE sessions 
         SET expires_at = NOW() + INTERVAL '7 days' 
         WHERE token = $1 
         AND expires_at < NOW() + INTERVAL '6 days'",
        token
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| SystemError::database(e.to_string()))?;

    Ok(())
}

// #[cfg(feature = "ssr")]
// pub async fn verify_password(
//     tx: &mut Transaction<'_, Postgres>,
//     email: &str,
//     user_pass: String,
// ) -> Result<bool, SystemError> {
//     let record = sqlx::query!("SELECT password_hash FROM handlers WHERE email = $1", email)
//         .fetch_one(&mut *tx)
//         .await
//         .map_err(|e| SystemError::database(e.to_string()))?;

//     Ok(record.password_hash)
// }

#[cfg(feature = "ssr")]
pub async fn verify_password(
    password_candidate: String,
    password_hash: String,
) -> Result<bool, SystemError> {
    use argon2::{Argon2, PasswordHash, PasswordVerifier};

    tokio::task::spawn_blocking(move || {
        let parsed_hash = PasswordHash::new(&password_hash)
            .map_err(|e| SystemError::general(format!("Invalid hash format: {}", e)))?;

        let is_valid = Argon2::default()
            .verify_password(password_candidate.as_bytes(), &parsed_hash)
            .is_ok();

        Ok(is_valid)
    })
    .await
    .map_err(|e| SystemError::general(format!("Task join error: {}", e)))?
}

#[cfg(feature = "ssr")]
pub async fn hash_password(raw_pass: String) -> Result<String, SystemError> {
    tokio::task::spawn_blocking(move || {
        let engine = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let hash = engine
            .hash_password(raw_pass.as_bytes(), &salt)
            .map_err(|e| SystemError::general(format!("Hashing error: {}", e)))?;

        Ok(hash.to_string())
    })
    .await
    .map_err(|e| SystemError::general(format!("Task join error: {}", e)))?
}
