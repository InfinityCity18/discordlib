use reqwest::Url;

use super::error::{GatewayClientError, IntoBox};
use crate::apiclient::ApiClient;
use std::sync::Arc;

struct GatewayClient {
    gateway_url: Url,
}

impl GatewayClient {
    async fn new(api_client: Arc<ApiClient>, bot: bool) -> Result<Self, GatewayClientError> {
        let gateway_url = api_client.get_gateway(bot).await.intobox()?;
        Ok(GatewayClient { gateway_url })
    }
}
