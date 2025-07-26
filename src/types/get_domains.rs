use crate::client::API_URL;
use serde::Deserialize;

use crate::client::Client;
#[derive(Debug, Deserialize, Clone)]
struct DomainsResponse {
    result: Vec<String>,
}

impl Client {
    /// Get the available domains for emails
    ///
    /// ## Example
    /// ```no_run
    /// use tmapi::Client;
    ///
    /// # async {
    /// let client = Client::new("y@iusearch.lol").unwrap();
    /// let domains = client.get_domains().await.unwrap();
    /// # };
    /// ```
    pub async fn get_domains(&self) -> Result<Vec<String>, crate::ErrorType> {
        let url = format!("{API_URL}/domains");
        let response = self.client.get(url).send().await?;
        let response = response.json::<DomainsResponse>().await?;
        Ok(response.result)
    }
}
