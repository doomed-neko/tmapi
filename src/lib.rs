mod client;
mod types;

pub(crate) use types::errors::ErrorType;

pub use client::Client;
pub use types::errors::EmailsError;
pub use types::health::{ServerHealth, ServerHealthStatus};
