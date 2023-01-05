mod command_handler;
mod commands;
mod github;
mod playground;
mod utils;

use std::{collections::HashSet, env, sync::Arc};

use eludrs::HttpClient;
use futures::{future::join_all, stream::StreamExt};
use lazy_static::lazy_static;
use regex::Regex;

use command_handler::CommandRunner;
use github::*;

const PREFIX: &str = "uwu ";
const NAME: &str = "Uwuki";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let mut client = HttpClient::new().name(NAME.to_string());
    let gateway = client.create_gateway().await?;
    let client = Arc::new(client);
    let gh = Github::new(env::var("GITHUB_TOKEN").ok());

    let commands = CommandRunner::new(PREFIX.to_string()).commands(&[]);

    let mut events = gateway.get_events().await?;

    while let Some(mut msg) = events.next().await {
        if msg.author == NAME {
            continue;
        } else if msg.content.trim().to_lowercase() == "uwu" {
            client.send("UwU").await?;
            continue;
        }

        if let Err(err) = commands.run_command(Arc::clone(&client), msg.clone()).await {
            client
                .send(format!("You're bad, you broke me :( ({:?})", err))
                .await
                .ok();
        }

        lazy_static! {
            static ref REPO_REGEX: Regex = Regex::new(r"(?P<ignore>!)?(?P<user>[a-zA-Z0-9_-]+)/(?P<repo>[a-zA-Z0-9_.-]+)").unwrap();
            static ref ISSUE_REGEX: Regex = Regex::new(r"(:?(?P<user>[a-zA-Z0-9_-]+)/)?(?P<repo>[a-zA-Z0-9_.-]+)#(?P<num>\d+)").unwrap();
            static ref SNIPPET_REGEX: Regex = Regex::new(r"https://github\.com/(?P<user>[a-zA-Z0-9_-]+)/(?P<repo>[a-zA-Z0-9_.-]+)/blob/(?P<file>[a-zA-Z0-9./_ -]+)#L(?P<start>\d+)(?:-L(?P<end>\d+))?").unwrap();
        }
        let mut repos = join_all(
            REPO_REGEX
                .captures_iter(&msg.content)
                .filter(|c| c.name("ignore").is_none())
                .map(|c| {
                    (
                        c.name("user").unwrap().as_str(),
                        c.name("repo").unwrap().as_str(),
                    )
                })
                .collect::<HashSet<(&str, &str)>>()
                .into_iter()
                .map(|(user, repo)| gh.get_repo(user, repo)),
        )
        .await
        .into_iter()
        .filter(|i| i.is_ok())
        .map(|i| i.unwrap().to_string())
        .collect::<Vec<String>>();

        let mut issues = join_all(
            ISSUE_REGEX
                .captures_iter(&msg.content)
                .flat_map(|c| match c.name("user") {
                    Some(name) => vec![(
                        name.as_str(),
                        c.name("repo").unwrap().as_str(),
                        c.name("num").unwrap().as_str().parse().unwrap(),
                    )],
                    None => vec![
                        (
                            "eludris",
                            c.name("repo").unwrap().as_str(),
                            c.name("num").unwrap().as_str().parse().unwrap(),
                        ),
                        (
                            "eludris-community",
                            c.name("repo").unwrap().as_str(),
                            c.name("num").unwrap().as_str().parse().unwrap(),
                        ),
                    ],
                })
                .collect::<HashSet<(&str, &str, u32)>>()
                .into_iter()
                .map(|(user, repo, num)| gh.get_issue(user, repo, num)),
        )
        .await
        .into_iter()
        .filter(|i| i.is_ok())
        .map(|i| i.unwrap().to_string())
        .collect::<Vec<String>>();

        let mut snippets: Vec<String> = join_all(
            SNIPPET_REGEX
                .captures_iter(&msg.content)
                .map(|c| {
                    (
                        c.name("user").unwrap().as_str(),
                        c.name("repo").unwrap().as_str(),
                        c.name("file").unwrap().as_str(),
                        c.name("start").unwrap().as_str().parse().unwrap(),
                        c.name("end").map(|c| c.as_str().parse().unwrap()),
                    )
                })
                .collect::<HashSet<(&str, &str, &str, u32, Option<u32>)>>()
                .into_iter()
                .map(|(user, repo, file, start, end)| gh.get_snippet(user, repo, file, start, end)),
        )
        .await
        .into_iter()
        .filter_map(|s| s.ok())
        .collect();

        let mut blocks = Vec::new();
        blocks.append(&mut repos);
        blocks.append(&mut issues);
        blocks.append(&mut snippets);

        if !blocks.is_empty() {
            let content = blocks.join("\n");
            if content.len() > 2000 {
                client.send("Content too long uwu but sad").await?;
            } else {
                client.send(blocks.join("\n")).await?;
            }
        } else if msg.content.starts_with(PREFIX) {
            msg.content.drain(..PREFIX.len());
            match msg
                .content
                .split_once([' ', '\n'])
                .map(|(cmd, args)| (cmd, Some(args)))
                .unwrap_or((msg.content.trim(), None))
            {
                ("waa", _) => {
                    client.send("desuwa!").await?;
                }
                ("info", _) => {
                    client.send("<https://eludris.pages.dev>").await?;
                }
                ("blog", _) => {
                    client.send("<https://eludris.pages.dev/blog>").await?;
                }
                ("docs", _) => {
                    client.send("https://eludris.github.io/docs").await?;
                }
                ("awesome" | "awe", _) => {
                    client.send("<https://github.com/eludris/awesome>").await?;
                }
                ("community", _) => {
                    client
                        .send("<https://github.com/eludris-community>")
                        .await?;
                }
                ("org", _) => {
                    client.send("<https://github.com/eludris>").await?;
                }
                ("github" | "gh" | "repo", repo) => {
                    client
                        .send(format!(
                            "<https://github.com/eludris/{}>",
                            repo.unwrap_or("eludris").split(' ').next().unwrap()
                        ))
                        .await?;
                }
                ("stellar", _) => {
                    client
                        .send("<https://www.youtube.com/watch?v=a51VH9BYzZA>")
                        .await?;
                }
                _ => {}
            }
        } else if msg.content.trim() == "I am the fastest." {
            client.send("I am the even fasterer.").await?;
        }
    }

    Ok(())
}
