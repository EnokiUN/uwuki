mod utils;

use futures::stream::StreamExt;

use utils::*;
use uwuki::{GatewayClient, HttpClient};

const PREFIX: &str = "uwu ";
const NAME: &str = "Uwuki";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let client = HttpClient::new().name(NAME.to_string());
    let gateway = GatewayClient::new();
    let mut events = gateway.get_events().await?;

    while let Some(mut msg) = events.next().await {
        if msg.content.trim().to_lowercase() == "uwu" && msg.author != NAME {
            client.send("UwU").await?;
        } else if msg.content.starts_with(PREFIX) {
            msg.content.drain(..PREFIX.len());
            match msg.content.split_once(' ') {
                Some((cmd, args)) => match cmd {
                    "say" => {
                        client.send(args).await?;
                    }
                    "imposter" => {
                        let mut content = args.to_string();
                        let author = get_arg(&mut content);
                        if author.len() < 2 || author.len() > 32 {
                            client
                                .send("The author name should be between 2-32 characters long *b..baka!* >//<")
                                .await?;
                        }
                        if content.is_empty() {
                            client.send_message(args, "I am sus").await?;
                        } else {
                            client.send_message(author, content).await?;
                        }
                    }
                    _ => {}
                },
                None => match msg.content.as_ref() {
                    "waa" => {
                        client.send("desuwa!").await?;
                    }
                    "info" => {
                        client.send("wot").await?;
                    }
                    _ => {}
                },
            }
        }
    }

    Ok(())
}
