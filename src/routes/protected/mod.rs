// Protected routes - Requires authentication
// Dashboard, Settings, etc.

mod catalog;
mod components;
mod dashboard;
mod inventory;
mod layout;
mod pos;

pub use catalog::CatalogPage;
pub(super) use components::*;
pub use dashboard::DashboardPage;
pub use inventory::InventoryPage;
pub use layout::LayoutPage;
pub use pos::PosPage;
