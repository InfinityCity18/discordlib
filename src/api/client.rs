use crate::error::BoxErr;

use super::error::ApiClientError;
use super::links::*;
use reqwest::header::{HeaderValue, InvalidHeaderValue};
use reqwest::{
    header::{self, HeaderMap},
    Client, Url,
};
use serde_json::Value;

#[derive(Debug)]
pub struct ApiClient {
    client: Client,
}

impl ApiClient {
    pub fn new(token: &str) -> Result<Self, ApiClientError> {
        let mut headers = HeaderMap::new();

        let mut auth: HeaderValue = header::HeaderValue::from_str(token).bx()?;
        auth.set_sensitive(true);

        headers.insert(header::AUTHORIZATION, auth);

        let client = Client::builder().default_headers(headers).build().bx()?;

        Ok(Self { client })
    }
    pub async fn get_gateway(&self, bot: bool) -> Result<Url, ApiClientError> {
        let endpoint = if bot {
            GET_GATEWAY_BOT_ENDPOINT
        } else {
            GET_GATEWAY_ENDPOINT
        };

        let response = self
            .client
            .get(format!("{}{}", API_LINK, endpoint))
            .send()
            .await
            .bx()?;

        let mut json: Value = response.json().await.bx()?;
        let url: String = serde_json::from_value(json["url"].take()).bx()?;

        Ok(Url::parse(&url).bx()?)
    }
}

#[cfg(test)]
mod tests {
    use super::ApiClient;

    #[tokio::test]
    async fn gateway_test() {
        let client = ApiClient::new("").unwrap();
        let url = client.get_gateway(false).await.unwrap();
        assert_eq!("wss", &url.as_str()[..=2]);
    }

    #[tokio::test]
    #[should_panic]
    async fn gateway_bot_test() {
        let client = ApiClient::new("").unwrap();
        client.get_gateway(true).await.unwrap(); // invalid token provided -> panic
    }
}
