#[cfg(feature = "ssr")]
pub mod auth;
#[cfg(feature = "ssr")]
pub mod handler;
#[cfg(feature = "ssr")]
pub mod inventory;

#[cfg(feature = "ssr")]
pub use auth::*;
#[cfg(feature = "ssr")]
pub use handler::*;
#[cfg(feature = "ssr")]
pub use inventory::*;
