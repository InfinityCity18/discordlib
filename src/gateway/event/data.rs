use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventData {
    Identify {
        token: String,
        properties: ConnectionProperties,
        intents: u32,
    },
    Resume {
        token: String,
        session_id: String,
        seq: u32,
    },
    Other(Value),
}

#[derive(Serialize, Deserialize)]
pub struct ConnectionProperties {
    os: String,
    browser: String,
    device: String,
}
