use crate::{error::BoxErr, API_VERSION};
use errors::{EmptyEventDataError, MessageError, NoHeartbeatACKError, NotHelloError};
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use reqwest::{get, Url};
use std::error::Error;
use std::ops::Not;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::WebSocket;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use super::error::GatewayClientError;
use super::event::{EventData, GatewayEvent, OpCode};
use crate::api::ApiClient;
use std::sync::Arc;

type WsTx = Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>;
type WsRx = Arc<Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>;
type BoxError = Box<dyn std::error::Error + Send + Sync>;

const JITTER: u64 = 2;

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

        let (wsstream, _) = connect_async(gateway_url.as_str()).await.bx()?;
        let (wstx, wsrx) = wsstream.split();
        let (wstx, wsrx) = (Arc::new(Mutex::new(wstx)), Arc::new(Mutex::new(wsrx)));

        let (sender, mut receiver) = mpsc::unbounded_channel::<Message>();

        get_msg(sender.clone(), wsrx.clone()).await?;
        let init_msg = receiver.recv().await.unwrap();
        let init_msg: GatewayEvent = init_msg.try_into()?;

        if init_msg.op != OpCode::Hello {
            Err(NotHelloError).bx()?;
        }

        let event_data = init_msg.event_data.ok_or(EmptyEventDataError).bx()?;
        let hb_interval = if let EventData::Hello { heartbeat_interval } = event_data {
            heartbeat_interval
        } else {
            return Err(NotHelloError).bx()?;
        };

        let first_hb = GatewayEvent::heartbeat(0u32);
        let first_hb = Message::try_from(first_hb).bx()?;

        tokio::time::sleep(Duration::from_millis(hb_interval / JITTER)).await;

        send_msg(first_hb, wstx).await?;
        get_msg(sender.clone(), wsrx.clone()).await?;

        let ack: GatewayEvent = receiver.recv().await.unwrap().try_into()?;
        if OpCode::HeartbeatACK != ack.op {
            return Err(NoHeartbeatACKError).bx()?;
        }

        Ok(GatewayClient { gateway_url })
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

async fn supervisor() {}

async fn send_msg(msg: Message, tx: WsTx) -> Result<(), BoxError> {
    let mut lock = tx.lock().await;
    lock.send(msg).await.bx()?;
    Ok(())
}

async fn get_msg(sender: mpsc::UnboundedSender<Message>, rx: WsRx) -> Result<(), BoxError> {
    let mut lock = rx.lock().await;
    let msg = lock.next().await.unwrap()?;
    sender.send(msg)?;
    Ok(())
}

mod errors {
    use crate::error::error_unit;
    error_unit!(MessageError);
    error_unit!(EmptyEventDataError);
    error_unit!(NotHelloError);
    error_unit!(NoHeartbeatACKError);
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
