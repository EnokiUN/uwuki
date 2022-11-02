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
    let gh = Github::new();

    loop {
        let mut events = gateway.get_events().await?;

        while let Some(mut msg) = events.next().await {
            lazy_static! {
                static ref ISSUE_REGEX: Regex = Regex::new(r"(:?(?P<user>[a-zA-Z0-9_-]+)/)?(?P<repo>[a-zA-Z0-9_.-]+)#(?P<num>\d+)").unwrap();
                static ref SNIPPET_REGEX: Regex = Regex::new(r"https://github\.com/(?P<user>[a-zA-Z0-9_-]+)/(?P<repo>[a-zA-Z0-9_.-]+)/blob/(?P<file>[a-zA-Z0-9./_ -]+)#L(?P<start>\d+)(?:-L(?P<end>\d+))?").unwrap();
            }
            let issues = join_all(ISSUE_REGEX.captures_iter(&msg.content).map(|c| {
                gh.get_issue(
                    c.name("user").map(|c| c.as_str()).unwrap_or("eludris"),
                    c.name("repo").unwrap().as_str(),
                    c.name("num").unwrap().as_str().parse().unwrap(),
                )
            }))
            .await
            .into_iter()
            .filter(|i| i.is_ok())
            .map(|i| i.unwrap().to_string())
            .collect::<Vec<String>>();
            let snippets: Vec<String> =
                join_all(SNIPPET_REGEX.captures_iter(&msg.content).map(|c| {
                    gh.get_snippet(
                        c.name("user").unwrap().as_str(),
                        c.name("repo").unwrap().as_str(),
                        c.name("file").unwrap().as_str(),
                        c.name("start").unwrap().as_str().parse().unwrap(),
                        c.name("end").map(|c| c.as_str().parse().unwrap()),
                    )
                }))
                .await
                .into_iter()
                .filter_map(|s| s.ok())
                .collect();
            if msg.content.trim().to_lowercase() == "uwu" && msg.author != NAME {
                client.send("UwU").await?;
            } else if !issues.is_empty() {
                client.send(issues.join("\n")).await?;
            } else if !snippets.is_empty() {
                client.send(snippets.join("\n")).await?;
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
                            } else if content.is_empty() {
                                client.send_message(args, "I am sus").await?;
                            } else {
                                client.send_message(author, content).await?;
                            }
                        }
                        "ban" => {
                            client.send(format!("Banned {} :hammer:", args)).await?;
                        }
                        "unban" => {
                            client.send(format!("unBanned {} un:hammer:", args)).await?;
                        }
                        "bonk" => {
                            client.send(format!("Bonkned {} :hammer:", args)).await?;
                        }
                        "unbonk" => {
                            client
                                .send(format!("unBonkned {} un:hammer:", args))
                                .await?;
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
    }
}
