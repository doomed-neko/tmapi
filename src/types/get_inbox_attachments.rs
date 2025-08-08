use crate::types::Error;
use crate::{client::API_URL, types::Attachment};
use serde::Deserialize;

use crate::{client::Client, types::errors::EmailsError};
#[derive(Debug, Deserialize, Clone)]
struct GetAttachmentsResponse {
    success: bool,
    result: Option<Vec<Attachment>>,
    error: Option<Error>,
}

impl Client {
    /// Get attachments for an inbox by its ID.
    ///
    /// ## Example
    /// ```no_run
    /// use tmapi::Client;
    ///
    /// # async {
    /// let client = Client::new("y@iusearch.lol").unwrap();
    /// let emails = client.get_attachments("usm2sw0qfv9a5ku9z4xmh8og").await.unwrap();
    /// # };
    /// ```
    pub async fn get_attachments<S>(&self, email_id: S) -> Result<Vec<Attachment>, crate::ErrorType>
    where
        S: Into<String>,
    {
        let email_id: String = email_id.into();
        if email_id.is_empty() {
            return Err(Box::new(EmailsError::NotFoundError {
                name: "NotFound".to_owned(),
                message: "Attachment not found".to_owned(),
            }));
        }
        let url = format!("{API_URL}/inbox/{email_id}/attachments");
        let response = self.client.get(url).send().await?;
        let response = response.json::<GetAttachmentsResponse>().await?;
        if response.success {
            Ok(response.result.unwrap())
        } else {
            Err(Box::new(EmailsError::from_get_attachment(response)))
        }
    }
}
impl EmailsError {
    fn from_get_attachment(response: GetAttachmentsResponse) -> Self {
        let error = response.error.unwrap();
        match error.name.as_str() {
            "NotFound" => Self::NotFoundError {
                name: error.name,
                message: error.description,
            },
            _ => Self::ValidationError {
                name: error.name,
                message: error.description,
            },
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notfound_error_conversion() {
        let response = GetAttachmentsResponse {
            success: false,
            error: Some(Error {
                name: "NotFound".into(),
                description: "Attachment not found".into(),
            }),
            result: None,
        };
        let error = EmailsError::from_get_attachment(response);
        assert_eq!(
            error,
            EmailsError::NotFoundError {
                name: "NotFound".into(),
                message: "Attachment not found".into(),
            }
        )
    }
    #[test]
    fn validation_error_conversion() {
        let response = GetAttachmentsResponse {
            success: false,
            error: Some(Error {
                name: "ValidationError".into(),
                description: "Invalid input".into(),
            }),
            result: None,
        };
        let error = EmailsError::from_get_attachment(response);
        assert_eq!(
            error,
            EmailsError::ValidationError {
                name: "ValidationError".into(),
                message: "Invalid input".into(),
            }
        )
    }
}
