use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};
use tokio_tungstenite::tungstenite::Message;

use crate::error::BoxErr;

pub use data::EventData;
pub use data::*;
pub use opcode::OpCode;

mod data;
mod opcode;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayEvent {
    #[serde(rename = "op")]
    pub op: OpCode,
    #[serde(rename = "d")]
    pub event_data: Option<EventData>,
    #[serde(rename = "s")]
    pub seq: Option<u32>,
    #[serde(rename = "t")]
    pub event_name: Option<String>,
}

impl GatewayEvent {
    pub fn heartbeat(seq: u32) -> Self {
        Self {
            op: OpCode::Heartbeat,
            event_data: Some(EventData::Heartbeat(seq)),
            ..Default::default()
        }
    }
}

impl From<GatewayEvent> for Message {
    fn from(value: GatewayEvent) -> Self {
        Message::text(serde_json::to_string(&value).unwrap())
    }
}

impl From<Message> for GatewayEvent {
    fn from(value: Message) -> Self {
        serde_json::from_str(value.to_text().unwrap()).unwrap()
    }
}
