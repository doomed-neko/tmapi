use crate::types::Error;
use crate::{client::API_URL, types::Attachment};
use serde::Deserialize;

use crate::{client::Client, types::errors::EmailsError};
#[derive(Debug, Deserialize, Clone)]
struct GetEmailAttachmentsResponse {
    success: bool,
    result: Option<Vec<Attachment>>,
    error: Option<Error>,
    note: Option<SupportedDomainsNote>,
}
#[derive(Debug, Deserialize, Clone)]
struct SupportedDomainsNote {
    #[serde(rename = "supportedDomains")]
    supported_domains: Vec<String>,
}

impl Client {
    /// Get all attachments for an email.
    /// You can specify a limit and/or an offset for easier pagination.
    /// maximum limit is 100, minimum is 1
    /// minimum offset is 0
    ///
    /// ## Example
    /// ```no_run
    /// use tmapi::Client;
    ///
    /// # async {
    /// let client = Client::new("y@iusearch.lol").unwrap();
    /// let emails = client.get_email_attachments(10, 0).await.unwrap();
    /// # };
    /// ```
    pub async fn get_email_attachments(
        &self,
        limit: u8,
        offset: u32,
    ) -> Result<Vec<Attachment>, crate::ErrorType> {
        let url = format!(
            "{API_URL}/emails/{}/attachments?limit={limit}&offset={offset}",
            self.email
        );
        let response = self.client.get(url).send().await?;
        let response = response.json::<GetEmailAttachmentsResponse>().await?;
        if response.success {
            Ok(response.result.unwrap())
        } else {
            Err(Box::new(EmailsError::from_get_email_attachments(response)))
        }
    }
}
impl EmailsError {
    fn from_get_email_attachments(response: GetEmailAttachmentsResponse) -> Self {
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
        let response = GetEmailAttachmentsResponse {
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
        let error = EmailsError::from_get_email_attachments(response);
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
        let response = GetEmailAttachmentsResponse {
            success: false,
            error: Some(Error {
                name: "ValidationError".into(),
                description: "Invalid input".into(),
            }),
            note: None,
            result: None,
        };
        let error = EmailsError::from_get_email_attachments(response);
        assert_eq!(
            error,
            EmailsError::ValidationError {
                name: "ValidationError".into(),
                message: "Invalid input".into(),
            }
        )
    }
}
