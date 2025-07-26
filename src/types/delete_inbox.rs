use serde::Deserialize;

use crate::{
    client::{API_URL, Client},
    types::{Error, errors::EmailsError},
};

#[derive(Debug, Deserialize, Clone)]
struct DeleteResponse {
    success: bool,
    error: Option<Error>,
}
impl Client {
    /// Delete a specific email from your inbox.
    ///
    /// ## Example
    /// ```no_run
    /// use tmapi::Client;
    ///
    /// # async {
    /// let client = Client::new("y@iusearch.lol").unwrap();
    /// client.delete_inbox("usm2sw0qfv9a5ku9z4xmh8og").await.unwrap();
    /// # };
    /// ```
    pub async fn delete_inbox<S>(&self, email_id: S) -> Result<(), crate::ErrorType>
    where
        S: Into<String>,
    {
        let url = format!("{API_URL}/inbox/{}", email_id.into());
        let response = self.client.delete(url).send().await?;
        let response = response.json::<DeleteResponse>().await?;
        if response.success {
            Ok(())
        } else {
            Err(Box::new(EmailsError::from_delete_inbox(response)))
        }
    }
}
impl EmailsError {
    fn from_delete_inbox(response: DeleteResponse) -> Self {
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
        let response = DeleteResponse {
            success: false,
            error: Some(Error {
                name: "Validation Error".into(),
                description: "Invalid input".into(),
            }),
        };
        let error = EmailsError::from_delete_inbox(response);
        assert_eq!(
            error,
            EmailsError::ValidationError {
                name: "Validation Error".into(),
                message: "Invalid input".into()
            }
        )
    }
}
