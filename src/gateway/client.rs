use crate::{error::BoxErr, API_VERSION};
use errors::{EmptyEventDataError, NotHelloError};
use futures_util::{SinkExt, StreamExt};
use reqwest::Url;
use std::error::Error;
use std::ops::Not;
use std::time::Duration;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use super::error::GatewayClientError;
use super::event::{GatewayEvent, OpCode};
use crate::api::ApiClient;
use std::sync::Arc;

const JITTER: f64 = 0.5;

struct GatewayClient {
    gateway_url: Url,
}

impl GatewayClient {
    async fn new(
        api_client: Arc<ApiClient>,
        token: &str,
        bot: bool,
    ) -> Result<Self, GatewayClientError> {
        let mut gateway_url = api_client.get_gateway(bot).await.bx()?;
        gateway_url.set_query(Some(format!("v={}", API_VERSION).as_str()));
        gateway_url.set_query(Some("encoding=json"));

        let (mut wsstream, _) = connect_async(gateway_url.as_str()).await.bx()?;
        let init_msg: GatewayEvent =
            serde_json::from_str(wsstream.next().await.unwrap().bx()?.to_text().bx()?).bx()?;

        if init_msg.op != OpCode::Hello {
            Err(NotHelloError).bx()?;
        }

        let mut hb_interval = init_msg.event_data.ok_or(EmptyEventDataError).bx()?.take();
        let hb_interval = hb_interval
            .get_mut("heartbeat_interval")
            .ok_or(EmptyEventDataError)
            .bx()?
            .take();
        let hb_interval: f64 = serde_json::from_value(hb_interval).bx()?;
        let hb_interval = hb_interval / 1000.0;

        dbg!(hb_interval);

        let first_hb = GatewayEvent::heartbeat(0u32);
        let first_hb = Message::from(first_hb);

        tokio::time::sleep(Duration::from_secs_f64(hb_interval * JITTER)).await;

        wsstream.send(first_hb).await.bx()?;

        dbg!(wsstream.next().await);

        Ok(GatewayClient { gateway_url })
    }
}

async fn supervisor() {}

mod errors {
    use crate::error::error_unit;
    error_unit!(EmptyEventDataError);
    error_unit!(NotHelloError);
}

#[cfg(test)]
mod tests {
    use super::GatewayClient;
    use crate::api::ApiClient;

    #[tokio::test]
    async fn placeholder() {
        let apiclient = crate::api::ApiClient::new("").unwrap();
        let _gatewayclient = GatewayClient::new(std::sync::Arc::new(apiclient), "", false)
            .await
            .unwrap();
    }
}
