use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total_count: i64,
    pub page: i64,
    pub page_size: i64,
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, total_count: i64, page: i64, page_size: i64) -> Self {
        Self {
            items,
            total_count,
            page,
            page_size,
        }
    }
}
