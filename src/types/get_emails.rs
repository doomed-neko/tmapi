use crate::client::API_URL;
use crate::types::Error;
use serde::Deserialize;

use crate::{
    client::Client,
    types::{Email, errors::EmailsError},
};
#[derive(Debug, Deserialize, Clone)]
struct GetResponse {
    success: bool,
    result: Option<Vec<Email>>,
    error: Option<Error>,
    note: Option<SupportedDomainsNote>,
}
#[derive(Debug, Deserialize, Clone)]
struct SupportedDomainsNote {
    #[serde(rename = "supportedDomains")]
    supported_domains: Vec<String>,
}

impl Client {
    /// Get all messages for an email.
    /// You can specify a limit and/or an offest for easier pagination.
    /// maximum limit is 100, minimum is 1
    /// minimum offset is 0
    ///
    /// ## Example
    /// ```no_run
    /// use tmapi::Client;
    ///
    /// # async {
    /// let client = Client::new("y@iusearch.lol").unwrap();
    /// let emails = client.get_emails(10, 0).await.unwrap();
    /// # };
    /// ```
    pub async fn get_emails(&self, limit: u8, offset: u32) -> Result<Vec<Email>, crate::ErrorType> {
        let url = format!(
            "{API_URL}/emails/{}?limit={limit}&offset={offset}",
            self.email
        );
        let response = self.client.get(url).send().await?;
        let response = response.json::<GetResponse>().await?;
        if response.success {
            Ok(response.result.unwrap())
        } else {
            Err(Box::new(EmailsError::from_get_emails(response)))
        }
    }
}
impl EmailsError {
    fn from_get_emails(response: GetResponse) -> Self {
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
