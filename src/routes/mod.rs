// Routes module
// Contains all page components organized by access level

pub mod auth;
pub mod home;
pub mod not_found;
pub mod protected;

// Re-export commonly used components
pub use auth::*;
pub use home::*;
pub use not_found::*;
pub use protected::*;
