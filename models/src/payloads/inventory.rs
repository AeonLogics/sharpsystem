use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AddProductPayload {
    #[validate(length(min = 2, message = "Product name must be at least 2 characters long."))]
    pub name: String,

    pub sku: Option<String>,
    pub category: Option<String>,
    pub is_tracked: bool,
}
