use const_format::formatcp;

pub const API_LINK: &str = formatcp!("https://discord.com/api/v{}", crate::API_VERSION);
pub const GET_GATEWAY_BOT_ENDPOINT: &str = "/gateway/bot";
pub const GET_GATEWAY_ENDPOINT: &str = "/gateway";
