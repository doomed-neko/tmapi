use crate::client::API_URL;
use crate::types::Error;
use serde::Deserialize;

use crate::{client::Client, types::errors::EmailsError};
#[derive(Debug, Deserialize, Clone)]
struct DownloadAttachmentResponse {
    error: Option<Error>,
}

impl Client {
    /// Download an attachment by its ID.
    ///
    /// ## Example
    /// ```no_run
    /// use tmapi::Client;
    ///
    /// # async {
    /// let client = Client::new("y@iusearch.lol").unwrap();
    /// let emails = client.download_attachment("usm2sw0qfv9a5ku9z4xmh8og").await.unwrap();
    /// # };
    /// ```
    pub async fn download_attachment<S>(
        &self,
        attachment_id: S,
    ) -> Result<Vec<u8>, crate::ErrorType>
    where
        S: Into<String>,
    {
        let attachment_id: String = attachment_id.into();
        if attachment_id.is_empty() {
            return Err(Box::new(EmailsError::NotFoundError {
                name: "NotFound".to_owned(),
                message: "Attachment not found".to_owned(),
            }));
        }
        let url = format!("{API_URL}/attachments/{attachment_id}");
        let response = self.client.get(url).send().await?;
        if response.status().is_success() {
            return Ok(response.bytes().await.unwrap().to_vec());
        }
        let response = response.json::<DownloadAttachmentResponse>().await?;
        Err(Box::new(EmailsError::from_download_attachment(response)))
    }
}
impl EmailsError {
    fn from_download_attachment(response: DownloadAttachmentResponse) -> Self {
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
        let response = DownloadAttachmentResponse {
            error: Some(Error {
                name: "NotFound".into(),
                description: "Attachment not found".into(),
            }),
        };
        let error = EmailsError::from_download_attachment(response);
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
        let response = DownloadAttachmentResponse {
            error: Some(Error {
                name: "ValidationError".into(),
                description: "Invalid input".into(),
            }),
        };
        let error = EmailsError::from_download_attachment(response);
        assert_eq!(
            error,
            EmailsError::ValidationError {
                name: "ValidationError".into(),
                message: "Invalid input".into(),
            }
        )
    }
}
