use serde::Deserialize;

use crate::client::{API_URL, Client};

#[derive(Debug, Deserialize, Clone)]
struct HealthResponse {
    result: ServerHealth,
}
#[derive(Debug, Deserialize, Clone)]
pub struct ServerHealth {
    pub worker: String,
    pub database: String,
    pub kv: String,
}
impl Client {
    /// Check server health
    ///
    /// ## Example
    /// ```no_run
    /// use tmapi::Client;
    ///
    /// # async {
    /// let client = Client::new("y@iusearch.lol").unwrap();
    /// let status = client.server_health().await.unwrap();
    /// # };
    /// ```
    pub async fn server_health(self) -> Result<ServerHealth, crate::ErrorType> {
        let url = format!("{API_URL}/health");
        let response = self.client.delete(url).send().await?;
        let response = response.json::<HealthResponse>().await?;
        Ok(response.result)
    }
}
