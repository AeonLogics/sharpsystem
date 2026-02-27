// Protected routes - Requires authentication
// Dashboard, Settings, etc.

mod dashboard;
mod layout;

pub use dashboard::DashboardPage;
pub use layout::LayoutPage;
