
# discordlib
Basic library for discord written in rust



Example usage code:
```
use discordlib::api::ApiClient;
use discordlib::gateway::client::{GatewayClient, GatewayInit};
use discordlib::gateway::event::{ConnectionProperties, GatewayEvent};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let apiclient = ApiClient::new("TOKEN HERE")?;

    let apiclient = Arc::new(apiclient);

    let init = GatewayInit {
        bot: true,
        conn_properties: ConnectionProperties {
            os: String::from("Linux"),
            browser: String::from("discordlib"),
            device: String::from("discordlib"),
        },
        intents: 0,
        token: String::from("TOKEN HERE"),
    };

    let (gatewayclient, supervisor) = GatewayClient::new(apiclient.clone(), init).await?;

    loop {
        // main loop here
        // e.g.
        let event = gatewayclient.get_event();

        do_something(event);
    }

    return Ok(());
}
```
