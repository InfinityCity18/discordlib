use serde_repr::{Deserialize_repr, Serialize_repr};

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
