use std::error::Error as ErrorTrait;
use thiserror::Error;

pub(crate) type ErrorType = Box<dyn ErrorTrait + Send + Sync>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EmailsError {
    /// This error occurs when the given input is in bad format, for both emails and inbox IDs.
    #[error("Invalid input")]
    ValidationError { name: String, message: String },
    /// This error occurs when using a domain that is not supported by the service.
    /// For supported domains see [`Client::get_domains`]
    ///
    /// [`Client::get_domains`]: crate::Client
    #[error("Domain not supported")]
    DomainError {
        name: String,
        message: String,
        supported_domains: Vec<String>,
    },
    /// This error occurs when requesting something that does not exist, like an invalid Inbox ID.
    #[error("Not found")]
    NotFoundError { name: String, message: String },
}
