use serde::Deserialize;

use crate::client::{API_URL, Client};

#[derive(Debug, Deserialize, Clone)]
struct HealthResponse {
    result: ServerHealth,
}
#[derive(Debug, Deserialize, Clone)]
pub struct ServerHealth {
    pub worker: ServerHealthStatus,
    pub database: ServerHealthStatus,
    pub kv: ServerHealthStatus,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ServerHealthStatus {
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "disconnected")]
    Disconnected,
}
impl Client {
    /// Check server health
    ///
    /// ## Example
    /// ```no_run
    /// use tmapi::Client;
    /// use tmapi::ServerHealthStatus;
    ///
    /// # async {
    /// let client = Client::new("y@iusearch.lol").unwrap();
    /// let status = client.server_health().await.unwrap();
    /// assert_eq!(status.database, ServerHealthStatus::Connected);
    /// # };
    /// ```
    ///
    pub async fn server_health(self) -> Result<ServerHealth, crate::ErrorType> {
        let url = format!("{API_URL}/health");
        #[cfg(not(feature = "blocking"))]
        let response = self.client.get(url).send().await?;
        #[cfg(not(feature = "blocking"))]
        let response = response.json::<HealthResponse>().await?;
        #[cfg(feature = "blocking")]
        let response = self.client.get(url).send()?;
        #[cfg(feature = "blocking")]
        let response = response.json::<HealthResponse>()?;
        Ok(response.result)
    }
}
