use serde::{Deserialize, Serialize};

pub(super) mod errors;
pub(super) mod health;

mod count_emails;
mod delete_attachment;
mod delete_emails;
mod delete_inbox;
mod download_attachment;
mod get_domains;
mod get_email_attachments;
mod get_emails;
mod get_inbox;
mod get_inbox_attachments;

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
    /// Indicates whether the email has attachments or not
    pub has_attachments: bool,
    /// Message received date
    pub attachment_count: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Attachment {
    /// The MIME type of the attachment.
    pub content_type: String,
    /// The timestamp when the attachment was created (Unix epoch).
    pub created_at: i64,
    /// The original filename of the attachment.
    pub filename: String,
    /// The unique identifier for the attachment.
    pub id: String,
    /// The size of the attachment in bytes.
    pub size: u64,
}

#[derive(Debug, Deserialize, Clone)]
struct Error {
    /// Error name
    name: String,
    /// Error description
    #[serde(rename = "message")]
    description: String,
}
