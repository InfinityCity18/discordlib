use std::default;

use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    Heartbeat(u32),
    Hello {
        heartbeat_interval: u64,
    },
    Ready {
        v: u8,
        user: User,
        guilds: Vec<UnavailableGuild>,
        session_id: String,
        resume_gateway_url: String,
    },
    Other(Value),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectionProperties {
    pub os: String,
    pub browser: String,
    pub device: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnavailableGuild {
    pub id: String,
    pub unavailable: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    id: String,
    username: String,
    discriminator: String,
    avatar: String,
}
