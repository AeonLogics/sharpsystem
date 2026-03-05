#[cfg(feature = "ssr")]
use models::entities::Product;
#[cfg(feature = "ssr")]
use models::errors::SystemError;
#[cfg(feature = "ssr")]
use models::payloads::AddProductPayload;
#[cfg(feature = "ssr")]
use uuid::Uuid;

#[cfg(feature = "ssr")]
use sqlx::{Postgres, Transaction};

#[cfg(feature = "ssr")]
pub async fn insert_product(
    conn: &mut Transaction<'_, Postgres>,
    payload: &AddProductPayload,
    system_id: &Uuid,
    added_by: &Uuid,
) -> Result<Product, SystemError> {
    let product_id = Uuid::new_v4();

    let product = sqlx::query_as::<_, Product>(
        r#"
        INSERT INTO products (
            id, system_id, name, sku, category, is_tracked, added_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, system_id, name, sku, category, is_tracked, added_by, last_edited_by, created_at, updated_at
        "#
    )
    .bind(product_id)
    .bind(system_id)
    .bind(&payload.name)
    .bind(&payload.sku)
    .bind(&payload.category)
    .bind(payload.is_tracked)
    .bind(added_by)
    .fetch_one(&mut **conn)
    .await
    .map_err(|e| {
        tracing::error!("Failed to insert product: {}", e);
        if let Some(db_err) = e.as_database_error() {
            if db_err.constraint() == Some("products_sku_key") {
                return SystemError::validation("A product with this SKU already exists.".to_string());
            }
        }
        SystemError::database(e.to_string())
    })?;

    Ok(product)
}
