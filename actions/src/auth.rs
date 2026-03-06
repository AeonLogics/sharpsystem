#[cfg(feature = "ssr")]
use crate::db_ops::auth::{
    create_session, delete_session, get_session_user, hash_password, refresh_session_expiry,
    verify_password,
};
#[cfg(feature = "ssr")]
use crate::db_ops::{create_handler, create_system, get_handler_auth_data};
#[cfg(feature = "ssr")]
use models::HandlerRole;
#[cfg(feature = "ssr")]
use sqlx::PgPool;
#[cfg(feature = "ssr")]
use tower_sessions::Session;

use leptos::prelude::*;
use models::entities::User;
use models::errors::SystemError;
use models::payloads::{AuthenticateUserPayload, RegisterWorkspacePayload};
use tracing::instrument;

#[instrument(ret, err, skip_all, fields(email = %payload.email))]
#[server(RegisterWorkspace)]
pub async fn register_workspace(payload: RegisterWorkspacePayload) -> Result<User, SystemError> {
    #[cfg(feature = "ssr")]
    {
        payload
            .validate()
            .map_err(|e| SystemError::validation(e.to_string()))?;

        let pool = use_context::<PgPool>().ok_or_else(|| {
            SystemError::database("Database connection pool not found in context.")
        })?;

        let password_hash = hash_password(&payload.password).await;
        let owner_id = uuid::Uuid::new_v4();

        let mut tx = pool
            .begin()
            .await
            .map_err(|e| SystemError::database(e.to_string()))?;

        let system_id = create_system(&mut tx, &payload, &owner_id).await?;

        let handler_id = create_handler(
            &mut tx,
            &system_id,
            &payload,
            &password_hash,
            HandlerRole::SystemAdmin,
        )
        .await?;

        let session: Session = leptos_axum::extract()
            .await
            .map_err(|e| SystemError::general(e.to_string()))?;

        let avatar_url = format!(
            "https://api.dicebear.com/7.x/initials/svg?seed={}",
            payload.system_name
        );

        let auth_data = crate::db_ops::handler::HandlerAuthData {
            handler_id,
            password_hash: password_hash.clone(),
            user_name: payload.user_name.clone(),
            email: payload.email.clone(),
            handler_role: HandlerRole::SystemAdmin,
            avatar_url: Some(avatar_url.clone()),
            bio: None,
            preferred_theme: Some("light".to_string()),
            system_id,
            workspace_handle: payload.workspace_handle.clone(),
            system_name: payload.system_name.clone(),
        };

        let token = uuid::Uuid::new_v4().to_string();
        let bundle = create_session(&mut tx, &auth_data, &token).await?;

        tx.commit()
            .await
            .map_err(|e| SystemError::database(e.to_string()))?;

        session
            .insert("session_token", bundle.token)
            .await
            .map_err(|e| SystemError::general(e.to_string()))?;

        Ok(bundle.user)
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = payload;
        unreachable!()
    }
}

#[instrument(err, skip_all, fields(email = %payload.email))]
#[server(AuthenticateUser)]
pub async fn authenticate_user(payload: AuthenticateUserPayload) -> Result<User, SystemError> {
    #[cfg(feature = "ssr")]
    {
        payload
            .validate()
            .map_err(|e| SystemError::validation(e.to_string()))?;

        let pool = use_context::<PgPool>().ok_or_else(|| {
            SystemError::database("Database connection pool not found in context.")
        })?;

        let mut tx = pool
            .begin()
            .await
            .map_err(|e| SystemError::database(e.to_string()))?;

        let data = get_handler_auth_data(&mut tx, &payload.email).await?;

        if !verify_password(&payload.password, &data.password_hash)? {
            return Err(SystemError::unauthorized("Invalid credentials"));
        }

        let session: Session = leptos_axum::extract()
            .await
            .map_err(|e| SystemError::general(e.to_string()))?;

        let token = uuid::Uuid::new_v4().to_string();
        let bundle = create_session(&mut tx, &data, &token).await?;

        tx.commit()
            .await
            .map_err(|e| SystemError::database(e.to_string()))?;

        session
            .insert("session_token", bundle.token)
            .await
            .map_err(|e| SystemError::general(e.to_string()))?;

        Ok(bundle.user)
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = payload;
        unreachable!()
    }
}

#[instrument(skip_all)]
#[server(ValidateSession)]
pub async fn validate_session() -> Result<Option<User>, SystemError> {
    #[cfg(feature = "ssr")]
    {
        tracing::debug!("Executing ValidateSession server function");

        let pool = use_context::<PgPool>().ok_or_else(|| {
            tracing::error!(
                "CRITICAL: use_context::<PgPool>() returned None in validate_session()"
            );
            SystemError::database("Database connection pool not found in context.")
        })?;

        let session: Session = leptos_axum::extract()
            .await
            .map_err(|e| SystemError::general(e.to_string()))?;

        let token: Option<String> = session
            .get("session_token")
            .await
            .map_err(|e| SystemError::general(e.to_string()))?;

        tracing::debug!(
            token_resolved = token.is_some(),
            "Session token resolution check"
        );

        match token {
            Some(t) => {
                let mut conn = pool
                    .acquire()
                    .await
                    .map_err(|e| SystemError::database(e.to_string()))?;

                let user = get_session_user(&mut conn, &t).await?;
                if let Some(ref u) = user {
                    tracing::info!(email = %u.email, "Session successfully authenticated for user");
                    let _ = refresh_session_expiry(&mut conn, &t).await;
                }
                tracing::debug!(user_fetched = user.is_some(), "Database user fetch check");
                Ok(user)
            }
            None => {
                tracing::debug!("No active session token found; returning no user.");
                Ok(None)
            }
        }
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[instrument(skip_all)]
#[server(Logout)]
pub async fn logout() -> Result<(), SystemError> {
    #[cfg(feature = "ssr")]
    {
        let pool = use_context::<PgPool>().ok_or_else(|| {
            SystemError::database("Database connection pool not found in context.")
        })?;

        let session: Session = leptos_axum::extract()
            .await
            .map_err(|e| SystemError::general(e.to_string()))?;

        let token: Option<String> = session
            .get("session_token")
            .await
            .map_err(|e| SystemError::general(e.to_string()))?;

        if let Some(t) = token {
            let mut conn = pool
                .acquire()
                .await
                .map_err(|e| SystemError::database(e.to_string()))?;

            delete_session(&mut conn, &t).await?;
        }

        session
            .delete()
            .await
            .map_err(|e| SystemError::general(e.to_string()))?;

        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}
