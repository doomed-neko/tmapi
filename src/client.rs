use reqwest::Client as ReqClient;

pub(crate) const API_URL: &str = "https://api.barid.site";

#[derive(Debug, Clone)]
pub struct Client {
    /// The email which the client uses
    pub email: String,
    /// A [reqwest] client for communicating with the API
    ///
    /// [reqwest]: <https://docs.rs/reqwest>
    pub client: ReqClient,
}
impl Client {
    /// Create a new instance of `Client`
    /// Returns None when the email is invalid.
    /// Your email must end with one of the supported domains returned by [`Client::get_domains`].
    ///
    /// ## Example
    /// ```
    /// use tmapi::Client;
    ///
    /// let client = Client::new("y@iusearch.lol").unwrap();
    /// ```
    /// [`Client::get_domains`]: crate::Client::get_domains
    pub fn new<S>(email: S) -> Option<Self>
    where
        S: Into<String>,
    {
        let email = email.into();
        if !email_address::EmailAddress::is_valid(&email) {
            return None;
        }
        let client = ReqClient::new();
        Some(Self { email, client })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_email() {
        let correct_email = "y@iusearch.lol";
        let client = Client::new(correct_email);
        assert!(client.is_some())
    }
    #[test]
    fn invalid_email() {
        let incorrect_email = "y";
        let client = Client::new(incorrect_email);
        assert!(client.is_none())
    }
}
