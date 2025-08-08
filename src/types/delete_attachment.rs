use serde::Deserialize;

use crate::{
    client::{API_URL, Client},
    types::{Error, errors::EmailsError},
};

#[derive(Debug, Deserialize, Clone)]
struct DeleteAttachmentResponse {
    success: bool,
    error: Option<Error>,
}

impl Client {
    /// Delete a specific attachment by its ID.
    ///
    /// ## Example
    /// ```no_run
    /// use tmapi::Client;
    /// # async {
    /// let client = Client::new("y@iusearch.lol").unwrap();
    /// let deleted_count = client.delete_attachment("att_usm2sw0qfv9a5ku9z4xmh8og").await.unwrap();
    /// # };
    /// ```
    pub async fn delete_attachment<S>(&self, attachment_id: S) -> Result<(), crate::ErrorType>
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
        let response = self.client.delete(url).send().await?;
        let response = response.json::<DeleteAttachmentResponse>().await?;
        if response.success {
            Ok(())
        } else {
            Err(Box::new(EmailsError::from_delete_attachment(response)))
        }
    }
}
impl EmailsError {
    fn from_delete_attachment(response: DeleteAttachmentResponse) -> Self {
        let error = response.error.unwrap();

        Self::ValidationError {
            name: error.name,
            message: error.description,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_conversion() {
        let response = DeleteAttachmentResponse {
            success: false,
            error: Some(Error {
                name: "ValidationError".into(),
                description: "Invalid input".into(),
            }),
        };
        let error = EmailsError::from_delete_attachment(response);
        assert_eq!(
            error,
            EmailsError::ValidationError {
                name: "ValidationError".into(),
                message: "Invalid input".into()
            }
        )
    }
}
