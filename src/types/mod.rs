use serde::{Deserialize, Serialize};
pub mod errors;

mod delete_emails;
mod delete_inbox;
mod get_domains;
mod get_emails;
mod get_inbox;
mod health;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Email {
    /// Message's ID
    pub id: String,
    /// Sender's address
    pub from_address: String,
    /// Receiver's address
    pub to_address: String,
    /// Message subject
    pub subject: String,
    /// Message received date
    pub received_at: i64,
    /// Message content formatted as html
    pub html_content: Option<String>,
    /// Message content formatted as text
    pub text_content: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct Error {
    /// Error name
    name: String,
    /// Error description
    #[serde(rename = "message")]
    description: String,
}
