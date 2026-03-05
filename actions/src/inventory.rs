use leptos::prelude::*;
use models::entities::Product;
use models::errors::SystemError;
use models::payloads::AddProductPayload;
use tracing::instrument;
use validator::Validate;

#[cfg(feature = "ssr")]
use sqlx::PgPool;
#[cfg(feature = "ssr")]
use tower_sessions::Session;

#[instrument(ret, err, skip_all, fields(name = %payload.name))]
#[server(AddProduct)]
pub async fn add_product(payload: AddProductPayload) -> Result<Product, SystemError> {
    #[cfg(feature = "ssr")]
    {
        // 1. Validate the payload (e.g., name is not empty)
        payload
            .validate()
            .map_err(|e| SystemError::validation(e.to_string()))?;

        // 2. Get the database pool
        let pool = use_context::<PgPool>().ok_or_else(|| {
            SystemError::database("Database connection pool not found in context.")
        })?;

        // 3. Get the active user's session
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

            // Look up who is making this request
            let user = crate::db_ops::auth::get_session_user(&mut conn, &t).await?;

            if let Some(user) = user {
                let mut tx = pool
                    .begin()
                    .await
                    .map_err(|e| SystemError::database(e.to_string()))?;

                // 4. Insert the actual product!
                let product = crate::db_ops::inventory::insert_product(
                    &mut tx,
                    &payload,
                    &user.system_id,
                    &user.id,
                )
                .await?;

                tx.commit()
                    .await
                    .map_err(|e| SystemError::database(e.to_string()))?;

                return Ok(product);
            }
        }

        Err(SystemError::unauthorized(
            "You must be logged in to modify the catalog.",
        ))
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = payload;
        unreachable!()
    }
}
