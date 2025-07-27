use crate::client::API_URL;
use crate::types::Error;
use serde::Deserialize;

use crate::{client::Client, types::errors::EmailsError};
#[derive(Debug, Deserialize, Clone)]
struct CountResponse {
    success: bool,
    result: Option<CountResult>,
    error: Option<Error>,
    note: Option<SupportedDomainsNote>,
}
#[derive(Debug, Deserialize, Clone, Copy)]
struct CountResult {
    count: u32,
}
#[derive(Debug, Deserialize, Clone)]
struct SupportedDomainsNote {
    #[serde(rename = "supportedDomains")]
    supported_domains: Vec<String>,
}

impl Client {
    /// Get message count for an email.
    ///
    /// ## Example
    /// ```no_run
    /// use tmapi::Client;
    ///
    /// # async {
    /// let client = Client::new("y@iusearch.lol").unwrap();
    /// let emails = client.email_count().await.unwrap();
    /// # };
    /// ```
    pub async fn email_count(&self) -> Result<u32, crate::ErrorType> {
        let url = format!("{API_URL}/emails/count/{}", self.email);
        let response = self.client.get(url).send().await?;
        let response = response.json::<CountResponse>().await?;
        if response.success {
            Ok(response.result.unwrap().count)
        } else {
            Err(Box::new(EmailsError::from_email_count(response)))
        }
    }
}
impl EmailsError {
    fn from_email_count(response: CountResponse) -> Self {
        let error = response.error.unwrap();

        match response.note {
            Some(note) => Self::DomainError {
                name: error.name,
                message: error.description,
                supported_domains: note.supported_domains,
            },
            None => Self::ValidationError {
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
    fn domain_error_conversion() {
        let response = CountResponse {
            success: false,
            error: Some(Error {
                name: "DomainError".into(),
                description: "Invalid domain".into(),
            }),
            note: Some(SupportedDomainsNote {
                supported_domains: vec![],
            }),
            result: None,
        };
        let error = EmailsError::from_email_count(response);
        assert_eq!(
            error,
            EmailsError::DomainError {
                name: "DomainError".into(),
                message: "Invalid domain".into(),
                supported_domains: vec![]
            }
        )
    }
    #[test]
    fn validation_error_conversion() {
        let response = CountResponse {
            success: false,
            error: Some(Error {
                name: "ValidationError".into(),
                description: "Invalid input".into(),
            }),
            note: None,
            result: None,
        };
        let error = EmailsError::from_email_count(response);
        assert_eq!(
            error,
            EmailsError::ValidationError {
                name: "ValidationError".into(),
                message: "Invalid input".into(),
            }
        )
    }
}
