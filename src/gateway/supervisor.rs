use crate::error::BoxErr;

use super::client::GatewayClient;
use super::client::{get_msg, send_msg};
use super::client::{WsRx, WsTx};
use super::event::GatewayEvent;
use errors::SupervisorError;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;
use tokio_tungstenite::tungstenite::Message;

pub async fn supervisor(
    client: Arc<GatewayClient>,
    api_tx: UnboundedSender<GatewayEvent>,
    api_rx: UnboundedReceiver<GatewayEvent>,
    wstx: WsTx,
    wsrx: WsRx,
    hb_interval: u64,
    mut seq: u32,
) -> Result<(), errors::SupervisorError> {
    let (mut msg_sender, mut msg_receiver) = tokio::sync::mpsc::unbounded_channel::<Message>();

    let mut hb_timer = spawn_hb_sleeper(hb_interval);
    tokio::pin!(hb_timer);

    loop {
        tokio::select! {
            _ = &mut hb_timer => {
                let hb = GatewayEvent::heartbeat(seq);
                tokio::spawn(send_msg(hb.try_into().bx()?, wstx.clone()));
                *hb_timer = spawn_hb_sleeper(hb_interval);
            }
            msg = msg_receiver.recv() => {
                    let w = msg.unwrap();
                }
        }
    }
}

fn spawn_hb_sleeper(hb_interval: u64) -> JoinHandle<()> {
    tokio::spawn(tokio::time::sleep(Duration::from_millis(hb_interval)))
}

mod errors {
    use crate::error::error_template;
    error_template!(SupervisorError);
}
