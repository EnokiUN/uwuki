#![allow(clippy::uninlined_format_args)]

mod command_handler;
mod commands;
mod crates;
mod github;
mod playground;
mod state;
mod urban_dictionary;
mod utils;

use std::{collections::HashSet, env, sync::Arc};

use anyhow::Context;
use commands::commands;
use eludrs::{models::Event, HttpClient};
use futures::{future::join_all, stream::StreamExt};
use lazy_static::lazy_static;
use rand::SeedableRng;
use regex::Regex;

use command_handler::CommandRunner;
use github::GitHub;
use reqwest::Client;
use state::UwukiState;
use tokio::sync::Mutex;

const PREFIX: &str = "uwu ";
const HELP_INVOCATION: &str = "uwu help";
const NAME: &str = "uwuki";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let mut client = HttpClient::new(
        &env::var("TOKEN").context("Couldn't find the TOKEN environment variable")?,
    );
    if let Ok(url) = env::var("INSTANCE_URL") {
        client = client.rest_url(url);
    }
    let gateway = client.create_gateway().await?;

    let state = Arc::new(UwukiState {
        http: client,
        client: Client::new(),
        github_token: env::var("GITHUB_TOKEN").ok(),
        rng: Mutex::new(SeedableRng::from_entropy()),
    });

    let commands =
        CommandRunner::new(PREFIX.to_string(), Arc::clone(&state)).commands(&commands()[..]);

    let mut events = gateway.get_events().await?;

    while let Some(event) = events.next().await {
        if let Event::Message(msg) = event {
            let mut content = match msg.content {
                Some(ref content) => content.trim().to_string(),
                None => continue,
            };
            if msg.author.username == NAME {
                continue;
            } else if content.to_lowercase() == "uwu" {
                state.send(msg.channel.get_id(), "UwU").await?;
                continue;
            } else if content.to_lowercase() == "!speed" {
                state.send(msg.channel.get_id(), "I am the faster.").await?;
                continue;
            } else if content.to_lowercase() == "kys" {
                state
                    .send(msg.channel.get_id(), "Keep Yourself Safe*")
                    .await?;
                continue;
            } else if content.to_lowercase() == "rtfrb" {
                state
                    .send(
                        msg.channel.get_id(),
                        "Read The Fucking [Rust Book](https://doc.rust-lang.org/stable/book/)",
                    )
                    .await?;
                continue;
            }

            if let Err(err) = commands.run_command(msg.clone()).await {
                state
                    .send(
                        msg.channel.get_id(),
                        format!("You're bad, you broke me :( ({:?})", err),
                    )
                    .await
                    .ok();
            }

            lazy_static! {
                static ref REPO_REGEX: Regex = Regex::new(r"(?P<ignore>(:?!|/))?(?P<user>[a-zA-Z0-9_-]+)/(?P<repo>[a-zA-Z0-9_.-]+)").unwrap();
                static ref ISSUE_REGEX: Regex = Regex::new(r"(:?(?P<user>[a-zA-Z0-9_-]+)/)?(?P<repo>[a-zA-Z0-9_.-]+)(?:/(:?issues|pull))?(#|/)(?P<num>\d+)").unwrap();
                static ref COMMENT_REGEX: Regex = Regex::new(r"https://github\.com/(?P<user>[a-zA-Z0-9_-]+)/(?P<repo>[a-zA-Z0-9_.-]+)/(?:issues|pull)/(?:\d+)#(?P<type>issuecomment-|discussion_r)(?P<id>\d+)").unwrap();
                static ref SNIPPET_REGEX: Regex = Regex::new(r"https://github\.com/(?P<user>[a-zA-Z0-9_-]+)/(?P<repo>[a-zA-Z0-9_.-]+)/blob/(?P<file>[a-zA-Z0-9./_+() -]+)#L(?P<start>\d+)(?:-L(?P<end>\d+))?").unwrap();
            }
            let mut repos = join_all(
                REPO_REGEX
                    .captures_iter(&content)
                    .filter(|c| c.name("ignore").is_none())
                    .map(|c| {
                        (
                            c.name("user").unwrap().as_str(),
                            c.name("repo").unwrap().as_str(),
                        )
                    })
                    .collect::<HashSet<(&str, &str)>>()
                    .into_iter()
                    .map(|(user, repo)| state.get_repo(user, repo)),
            )
            .await
            .into_iter()
            .filter(|i| i.is_ok())
            .map(|i| i.unwrap().to_string())
            .collect::<Vec<String>>();

            let mut issues = join_all(
                ISSUE_REGEX
                    .captures_iter(&content)
                    .flat_map(|c| match c.name("user") {
                        Some(name) => vec![(
                            name.as_str(),
                            c.name("repo").unwrap().as_str(),
                            c.name("num").unwrap().as_str().parse().unwrap_or(0),
                        )],
                        None => vec![
                            (
                                "eludris",
                                c.name("repo").unwrap().as_str(),
                                c.name("num").unwrap().as_str().parse().unwrap_or(0),
                            ),
                            (
                                "eludris-community",
                                c.name("repo").unwrap().as_str(),
                                c.name("num").unwrap().as_str().parse().unwrap_or(0),
                            ),
                        ],
                    })
                    .collect::<HashSet<(&str, &str, u32)>>()
                    .into_iter()
                    .map(|(user, repo, num)| state.get_issue(user, repo, num)),
            )
            .await
            .into_iter()
            .filter(|i| i.is_ok())
            .map(|i| i.unwrap().to_string())
            .collect::<Vec<String>>();

            let mut comments: Vec<String> = join_all(
                COMMENT_REGEX
                    .captures_iter(&content)
                    .map(|c| {
                        (
                            c.name("user").unwrap().as_str(),
                            c.name("repo").unwrap().as_str(),
                            if c.name("type").unwrap().as_str() == "issuecomment-" {
                                "issues"
                            } else {
                                "pulls"
                            },
                            c.name("id").unwrap().as_str().parse().unwrap_or(0),
                        )
                    })
                    .collect::<HashSet<(&str, &str, &str, u32)>>()
                    .into_iter()
                    .map(|(user, repo, comment_type, id)| {
                        state.get_comment(user, repo, comment_type, id)
                    }),
            )
            .await
            .into_iter()
            .filter(|i| i.is_ok())
            .map(|i| i.unwrap().to_string())
            .collect();

            let mut snippets: Vec<String> = join_all(
                SNIPPET_REGEX
                    .captures_iter(&content)
                    .map(|c| {
                        (
                            c.name("user").unwrap().as_str(),
                            c.name("repo").unwrap().as_str(),
                            c.name("file").unwrap().as_str(),
                            c.name("start").unwrap().as_str().parse().unwrap_or(0),
                            c.name("end").map(|c| c.as_str().parse().unwrap_or(0)),
                        )
                    })
                    .collect::<HashSet<(&str, &str, &str, u32, Option<u32>)>>()
                    .into_iter()
                    .map(|(user, repo, file, start, end)| {
                        state.get_snippet(user, repo, file, start, end)
                    }),
            )
            .await
            .into_iter()
            .filter_map(|s| s.ok())
            .collect();

            let mut blocks = Vec::new();
            blocks.append(&mut repos);
            blocks.append(&mut issues);
            blocks.append(&mut comments);
            blocks.append(&mut snippets);

            if !blocks.is_empty() {
                let content = blocks.join("\n");
                if content.len() > 2000 {
                    state
                        .send(msg.channel.get_id(), "Content too long uwu but sad")
                        .await?;
                } else {
                    state.send(msg.channel.get_id(), content).await?;
                }
            } else if content.starts_with(HELP_INVOCATION) {
                content.drain(..HELP_INVOCATION.len());
                if content.is_empty() {
                    state
                        .send(
                            msg.channel.get_id(),
                            format!(
                                "```\nHelp:\n{}\n\nuwu > owo\n```",
                                commands
                                    .get_commands()
                                    .iter()
                                    .map(|c| format!(
                                        "{:<15} {}",
                                        format!("{}:", c.names[0]),
                                        c.description
                                    ))
                                    .collect::<Vec<String>>()
                                    .join("\n")
                            ),
                        )
                        .await?;
                } else {
                    match commands.get_command(content.trim()) {
                        Some(command) => {
                            state
                                .send(
                                    msg.channel.get_id(),
                                    format!(
                                        "```\n__{}__\n{}\n\n{}\n```",
                                        command.names[0], command.description, command.usage
                                    ),
                                )
                                .await?;
                        }
                        None => {
                            state
                                .send(msg.channel.get_id(), "Could not find that command, L? >~<")
                                .await?;
                        }
                    }
                };
            }
        }
    }

    Ok(())
}
