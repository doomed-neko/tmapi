use serde::Deserialize;

use crate::{
    client::{API_URL, Client},
    types::{Error, errors::EmailsError},
};

#[derive(Debug, Deserialize, Clone)]
struct DeleteResponse {
    success: bool,
    error: Option<Error>,
    result: Option<DeleteResult>,
}
#[derive(Debug, Deserialize, Clone)]
struct DeleteResult {
    deleted_count: u32,
}
impl Client {
    /// Delete all emails in your inbox.
    /// Returns how many emails where deleted in the operation.
    ///
    /// ## Example
    /// ```no_run
    /// use tmapi::Client;
    /// # async {
    /// let client = Client::new("y@iusearch.lol").unwrap();
    /// let deleted_count = client.delete_all_emails().await.unwrap();
    /// # };
    /// ```
    pub async fn delete_all_emails(&self) -> Result<u32, crate::ErrorType> {
        let url = format!("{API_URL}/emails/{}", self.email);
        let response = self.client.delete(url).send().await?;
        let response = response.json::<DeleteResponse>().await?;
        if response.success {
            Ok(response.result.unwrap().deleted_count)
        } else {
            Err(Box::new(EmailsError::from_delete_emails(response)))
        }
    }
}
impl EmailsError {
    fn from_delete_emails(response: DeleteResponse) -> Self {
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
            result: None,
        };
        let error = EmailsError::from_delete_emails(response);
        assert_eq!(
            error,
            EmailsError::ValidationError {
                name: "Validation Error".into(),
                message: "Invalid input".into()
            }
        )
    }
}
