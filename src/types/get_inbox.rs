use crate::client::API_URL;
use crate::types::Error;
use serde::Deserialize;

use crate::{
    client::Client,
    types::{Email, errors::EmailsError},
};
#[derive(Debug, Deserialize, Clone)]
struct GetInboxResponse {
    success: bool,
    result: Option<Email>,
    error: Option<Error>,
}

impl Client {
    /// Get a specific message by its ID.
    ///
    /// ## Example
    /// ```no_run
    /// use tmapi::Client;
    ///
    /// # async {
    /// let client = Client::new("y@iusearch.lol").unwrap();
    /// let emails = client.get_inbox("usm2sw0qfv9a5ku9z4xmh8og").await.unwrap();
    /// # };
    /// ```
    pub async fn get_inbox<S>(&self, email_id: S) -> Result<Email, crate::ErrorType>
    where
        S: Into<String>,
    {
        let email_id: String = email_id.into();
        if email_id.is_empty() {
            return Err(Box::new(EmailsError::NotFoundError {
                name: "NotFound".to_owned(),
                message: "Email not found".to_owned(),
            }));
        }
        let url = format!("{API_URL}/inbox/{email_id}");
        let response = self.client.get(url).send().await?;
        let response = response.json::<GetInboxResponse>().await?;
        if response.success {
            Ok(response.result.unwrap())
        } else {
            Err(Box::new(EmailsError::from_get_inbox(response)))
        }
    }
}
impl EmailsError {
    fn from_get_inbox(response: GetInboxResponse) -> Self {
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
        let response = GetInboxResponse {
            success: false,
            error: Some(Error {
                name: "NotFound".into(),
                description: "Inbox not found".into(),
            }),
            result: None,
        };
        let error = EmailsError::from_get_inbox(response);
        assert_eq!(
            error,
            EmailsError::NotFoundError {
                name: "NotFound".into(),
                message: "Inbox not found".into(),
            }
        )
    }
    #[test]
    fn validation_error_conversion() {
        let response = GetInboxResponse {
            success: false,
            error: Some(Error {
                name: "ValidationError".into(),
                description: "Invalid input".into(),
            }),
            result: None,
        };
        let error = EmailsError::from_get_inbox(response);
        assert_eq!(
            error,
            EmailsError::ValidationError {
                name: "ValidationError".into(),
                message: "Invalid input".into(),
            }
        )
    }
}
