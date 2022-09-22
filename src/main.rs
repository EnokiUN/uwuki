mod github;
mod utils;

use futures::{future::join_all, stream::StreamExt};
use lazy_static::lazy_static;
use regex::Regex;

use github::*;
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
    let gh = Github::new();

    while let Some(mut msg) = events.next().await {
        lazy_static! {
            static ref RE: Regex = Regex::new(r#"([a-zA-Z0-9_-]+)#(\d+)"#).unwrap();
        }
        let issues = join_all(RE.captures_iter(&msg.content).map(|c| {
            gh.get_issue(
                c.get(1).unwrap().as_str().to_string(),
                c.get(2).unwrap().as_str().parse().unwrap(),
            )
        }))
        .await
        .into_iter()
        .filter(|i| i.is_ok())
        .map(|i| i.unwrap().to_string())
        .collect::<Vec<String>>();
        if msg.content.to_lowercase().contains("uwu") && msg.author != NAME {
            client.send("UwU").await?;
        }
        if !issues.is_empty() {
            client.send(issues.join("\n")).await?;
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
                        } else {
                            if content.is_empty() {
                                client.send_message(args, "I am sus").await?;
                            } else {
                                client.send_message(author, content).await?;
                            }
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
