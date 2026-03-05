use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Product {
    pub id: Uuid,
    pub system_id: Uuid,
    pub name: String,
    pub sku: Option<String>,
    pub category: Option<String>,
    pub is_tracked: bool,
    pub added_by: Uuid,
    pub last_edited_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
