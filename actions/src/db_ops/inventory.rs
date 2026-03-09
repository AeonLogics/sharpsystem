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

    let sku = payload.sku.as_ref().filter(|s| !s.trim().is_empty());
    let category = payload.category.as_ref().filter(|s| !s.trim().is_empty());

    let product = sqlx::query_as!(
        Product,
        r#"
        INSERT INTO products (
            id, system_id, name, sku, category, is_tracked, added_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, system_id, name, sku, category, is_tracked, added_by, last_edited_by as "last_edited_by?"
        "#
        ,
        product_id,
        system_id,
        payload.name,
        sku,
        category,
        payload.is_tracked,
        added_by
    )
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

#[cfg(feature = "ssr")]
pub async fn initialize_untracked_inventory(
    conn: &mut Transaction<'_, Postgres>,
    product_id: &Uuid,
) -> Result<(), SystemError> {
    sqlx::query!(
        r#"
        INSERT INTO untracked_inventory (product_id, quantity)
        VALUES ($1, 0)
        ON CONFLICT (product_id) DO NOTHING
        "#,
        product_id
    )
    .execute(&mut **conn)
    .await
    .map_err(|e| {
        tracing::error!("Failed to initialize untracked inventory: {}", e);
        SystemError::database(e.to_string())
    })?;

    Ok(())
}
#[cfg(feature = "ssr")]
pub async fn get_products_for_system(
    conn: &mut sqlx::PgConnection,
    system_id: &Uuid,
) -> Result<Vec<Product>, SystemError> {
    let products = sqlx::query_as!(
        Product,
        r#"
        SELECT id, system_id, name, sku, category, is_tracked, added_by, last_edited_by as "last_edited_by?"
        FROM products
        WHERE system_id = $1
        ORDER BY name ASC
        "#,
        system_id
    )
    .fetch_all(conn)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch products: {}", e);
        SystemError::database(e.to_string())
    })?;

    Ok(products)
}
