#[cfg(feature = "ssr")]
use argon2::{Argon2, PasswordHasher};
#[cfg(feature = "ssr")]
use models::{HandlerRole, SignupPayload, SystemError};
#[cfg(feature = "ssr")]
use password_hash::SaltString;
#[cfg(feature = "ssr")]
use rand_core::OsRng;
#[cfg(feature = "ssr")]
use sqlx::PgConnection;
#[cfg(feature = "ssr")]
use uuid::Uuid;

#[cfg(feature = "ssr")]
pub async fn create_system(
    tx: &mut PgConnection,
    payload: &SignupPayload,
    owner_id: &Uuid,
) -> Result<Uuid, SystemError> {
    let avatar_url = format!(
        "https://api.dicebear.com/7.x/initials/svg?seed={}",
        payload.system_name
    );
    let record = sqlx::query!(
        "INSERT INTO systems (owner_id, system_handle, system_name, avatar_url) VALUES ($1, $2, $3, $4) RETURNING id",
        *owner_id,
        payload.workspace_handle,
        payload.system_name,
        avatar_url
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| SystemError::database(e.to_string()))?;

    Ok(record.id)
}

#[cfg(feature = "ssr")]
pub async fn create_handler(
    tx: &mut PgConnection,
    system_id: &Uuid,
    payload: &SignupPayload,
    password_hash: &str,
    role: HandlerRole,
) -> Result<Uuid, SystemError> {
    let record = sqlx::query!(
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

    Ok(record.id)
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
    let record = sqlx::query!(
        "INSERT INTO sessions 
        (handler_id, system_id, token, handler_role, user_name, email, avatar_url, bio, preferred_theme, system_handle, system_name) 
        VALUES ($1, $2, $3, $4::public.handler_role, $5, $6, $7, $8, $9, $10, $11) 
        RETURNING token, handler_id, user_name, email, avatar_url, bio, preferred_theme, system_handle, system_name",
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
            id: data.handler_id,
            email: data.email.clone(),
            workspace_handle: data.workspace_handle.clone(),
            system_name: data.system_name.clone(),
            avatar_url: data.avatar_url.clone(),
            bio: data.bio.clone(),
            preferred_theme: data.preferred_theme.clone(),
        },
    })
}

#[cfg(feature = "ssr")]
pub async fn get_session_user(
    tx: &mut PgConnection,
    token: &str,
) -> Result<Option<User>, SystemError> {
    let record = sqlx::query!(
        "SELECT handler_id as \"handler_id!\", user_name as \"user_name!\", email as \"email!\", avatar_url, bio, preferred_theme, system_handle as \"system_handle!\", system_name as \"system_name!\" 
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
            email: r.email,
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
pub fn verify_password(password_candidate: &str, password_hash: &str) -> Result<bool, SystemError> {
    use argon2::{Argon2, PasswordHash, PasswordVerifier};

    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|e| SystemError::general(format!("Invalid hash format: {}", e)))?;

    let is_valid = Argon2::default()
        .verify_password(password_candidate.as_bytes(), &parsed_hash)
        .is_ok();

    Ok(is_valid)
}

#[cfg(feature = "ssr")]
pub async fn hash_password(raw_pass: &str) -> String {
    let engine = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let hash = engine.hash_password(raw_pass.as_bytes(), &salt).unwrap();

    hash.to_string()
}
