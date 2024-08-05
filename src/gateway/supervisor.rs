use crate::error::BoxErr;

use super::client::GatewayClient;
use super::client::{get_msg, send_msg};
use super::client::{WsRx, WsTx};
use super::event::EventData;
use super::event::GatewayEvent;
use super::event::OpCode;
use errors::{ConnectionClosed, SupervisorError};
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

    let hb_timer = spawn_hb_sleeper(hb_interval);
    tokio::pin!(hb_timer);

    loop {
        tokio::select! {
        _ = &mut hb_timer => {
            let hb = GatewayEvent::heartbeat(seq);
            tokio::spawn(send_msg(hb.try_into().bx()?, wstx.clone()));
            *hb_timer = spawn_hb_sleeper(hb_interval);
        }
        msg = msg_receiver.recv() => {
                let msg = msg.unwrap();
                let event: GatewayEvent = msg.try_into()?;
                if let Some(sequence) = event.seq {
                    seq = sequence;
                }

                match event.op {
                    OpCode::HeartbeatACK => (),
                    OpCode::Dispatch => {
                        api_tx.send(event);
                    },
                    OpCode::Heartbeat => {
                        let hb = GatewayEvent::heartbeat(seq);
                        tokio::spawn(send_msg(hb.try_into().bx()?, wstx.clone()));
                        *hb_timer = spawn_hb_sleeper(hb_interval);
                    },
                    OpCode::InvalidSession => {
                        if let Some(EventData::InvalidSession(resume)) = event.event_data {
                            if resume {
                                api_tx.send(GatewayEvent { op: OpCode::Resume, event_data: None, seq: None, event_name: None });
                                return Ok(());
                            } else {
                                api_tx.send(GatewayEvent { op: OpCode::NonDiscordClosed, event_data: None, seq: None, event_name: None });
                                return Err(ConnectionClosed).bx()?;
                            }
                        }
                    },
                    OpCode::Reconnect => {
                        api_tx.send(GatewayEvent { op: OpCode::Resume, event_data: None, seq: None, event_name: None });
                        return Ok(());
                    }
                }
            }
        }
    }
}

fn spawn_hb_sleeper(hb_interval: u64) -> JoinHandle<()> {
    tokio::spawn(tokio::time::sleep(Duration::from_millis(hb_interval)))
}

mod errors {
    use crate::error::{error_template, error_unit};
    error_template!(SupervisorError);
    error_unit!(ConnectionClosed);
}
