use leptos::prelude::*;
use models::entities::Product;
use models::errors::SystemError;
use models::payloads::AddProductPayload;
use tracing::instrument;

#[cfg(feature = "ssr")]
use sqlx::PgPool;
#[cfg(feature = "ssr")]
use validator::Validate;

#[instrument(ret, err, skip_all, fields(name = %payload.name))]
#[server(AddProduct)]
pub async fn add_product(payload: AddProductPayload) -> Result<Product, SystemError> {
    #[cfg(feature = "ssr")]
    {
        // 1. Validate the payload
        payload
            .validate()
            .map_err(|e| SystemError::validation(e.to_string()))?;

        // 2. Get the database pool
        let pool = use_context::<PgPool>().ok_or_else(|| {
            SystemError::database("Database connection pool not found in context.")
        })?;

        // 3. Get the active user's session
        use crate::helper::get_session_token;
        let token = get_session_token().await;

        if let Some(t) = token {
            let mut conn = pool
                .acquire()
                .await
                .map_err(|e| SystemError::database(e.to_string()))?;

            // Look up who is making this request
            let user = crate::db_ops::get_session_user(&mut conn, &t).await?;

            if let Some(user) = user {
                let mut tx = pool
                    .begin()
                    .await
                    .map_err(|e| SystemError::database(e.to_string()))?;

                // 4. Insert the actual product!
                let product =
                    crate::db_ops::insert_product(&mut tx, &payload, &user.system_id, &user.id)
                        .await?;

                // 5. If it's an untracked (bulk) item, initialize its bucket
                if !product.is_tracked {
                    crate::db_ops::initialize_untracked_inventory(&mut tx, &product.id).await?;
                }

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

#[server(GetProducts)]
pub async fn get_products() -> Result<Vec<Product>, SystemError> {
    #[cfg(feature = "ssr")]
    {
        // 1. Get the database pool
        let pool = use_context::<PgPool>().ok_or_else(|| {
            SystemError::database("Database connection pool not found in context.")
        })?;

        // 2. Get the session token
        use crate::helper::get_session_token;
        let token = get_session_token().await;

        if let Some(t) = token {
            let mut conn = pool
                .acquire()
                .await
                .map_err(|e| SystemError::database(e.to_string()))?;

            // 3. Get the user
            let user = crate::db_ops::get_session_user(&mut conn, &t).await?;

            if let Some(user) = user {
                // 4. Fetch the products!
                let products =
                    crate::db_ops::get_products_for_system(&mut conn, &user.system_id).await?;
                return Ok(products);
            }
        }

        Err(SystemError::unauthorized(
            "You must be logged in to view the catalog.",
        ))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}
