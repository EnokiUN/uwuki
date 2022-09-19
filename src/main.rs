use futures::stream::StreamExt;

use uwuki::{GatewayClient, HttpClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = HttpClient::new().name("Uwuki".to_string());
    let gateway = GatewayClient::new();
    let mut events = gateway.get_events().await?;

    while let Some(msg) = events.next().await {
        if msg.content == "!waa".to_string() {
            client.send("Desuwa!").await?;
        }
    }

    Ok(())
}
