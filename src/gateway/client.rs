use crate::error::BoxErr;
use reqwest::Url;
use std::error::Error;

use super::error::GatewayClientError;
use crate::apiclient::ApiClient;
use std::sync::Arc;

struct GatewayClient {
    gateway_url: Url,
}

impl GatewayClient {
    async fn new<T: Error + Send + Sync>(
        api_client: &ApiClient,
        bot: bool,
    ) -> Result<Self, GatewayClientError> {
        let gateway_url = api_client.get_gateway(bot).await.bx()?;
        Ok(GatewayClient { gateway_url })
    }
}
