use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

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
