//! A simple library to help with using [vwh]'s TempMail service on <https://barid.site>
//!
//! ```no_run
//! use tmapi::Client;
//! # async{
//! let client = Client::new("y@iusearch.lol").unwrap();
//! //                              limit, offset
//! let emails = client.get_emails(  10  ,   0   ).await.unwrap();
//! let first_email = emails.iter().next().unwrap();
//! let id = &first_email.id;
//! client.delete_inbox(id).await.unwrap();
//! # };
//! ```
//! [vwh]: <https://vwh.sh>
mod client;
mod types;

pub(crate) use types::errors::ErrorType;

pub use client::Client;
pub use types::Email;
pub use types::errors::EmailsError;
pub use types::health::{ServerHealth, ServerHealthStatus};
