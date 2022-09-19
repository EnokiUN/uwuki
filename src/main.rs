use futures::stream::StreamExt;

use uwuki::{GatewayClient, HttpClient};

const PREFIX: &'static str = "uwu ";
const NAME: &'static str = "Uwuki";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv::dotenv().ok();
    env_logger::init();

    let client = HttpClient::new().name(NAME.to_string());
    let gateway = GatewayClient::new();
    let mut events = gateway.get_events().await?;

    while let Some(mut msg) = events.next().await {
        if msg.content.to_lowercase() == "uwu" && msg.author != NAME {
            client.send("UwU").await?;
        } else {
            if msg.content.starts_with(PREFIX) {
                msg.content.drain(..PREFIX.len());
                match msg.content.split_once(' ') {
                    Some((cmd, args)) => match cmd {
                        "say" => {
                            client.send(args).await?;
                        }
                        "imposter" => match args.split_once(' ') {
                            Some((author, content)) => {
                                client.send_message(author, content).await?;
                            }
                            None => {
                                client.send_message(args, "I am sus").await?;
                            }
                        },
                        _ => {}
                    },
                    None => match msg.content.as_ref() {
                        "waa" => {
                            client.send("desuwa!").await?;
                        }
                        _ => {}
                    },
                }
            }
        }
    }

    Ok(())
}
