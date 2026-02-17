use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub workspace_handle: String,
    pub system_name: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub preferred_theme: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(
    feature = "ssr",
    sqlx(type_name = "public.handler_role", rename_all = "snake_case")
)]
pub enum HandlerRole {
    SystemAdmin,
    SystemManager,
    SystemSalesman,
}
