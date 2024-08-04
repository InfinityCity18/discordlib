use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};
use tokio_tungstenite::tungstenite::Message;

use crate::error::BoxErr;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayEvent {
    #[serde(rename = "op")]
    pub op: OpCode,
    #[serde(rename = "d")]
    pub event_data: Option<Value>,
    #[serde(rename = "s")]
    pub seq: Option<u32>,
    #[serde(rename = "t")]
    pub event_name: Option<String>,
}

impl GatewayEvent {
    pub fn heartbeat(seq: u32) -> Self {
        Self {
            op: OpCode::Heartbeat,
            seq: Some(seq),
            ..Default::default()
        }
    }
}

impl From<GatewayEvent> for Message {
    fn from(value: GatewayEvent) -> Self {
        Message::text(serde_json::to_string(&value).unwrap())
    }
}

#[derive(PartialEq, Clone, Debug, Deserialize_repr, Serialize_repr, Default)]
#[repr(u8)]
pub enum OpCode {
    #[default]
    Dispatch = 0,
    Heartbeat = 1,
    Identify = 2,
    PresenceUpdate = 3,
    VoiceStateUpdate = 4,
    Resume = 6,
    Reconnect = 7,
    RequestGuildMembers = 8,
    InvalidSession = 9,
    Hello = 10,
    HeartbeatACK = 11,
}
