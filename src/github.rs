use std::fmt::Display;

use reqwest::{header::USER_AGENT, Client};
use serde::{Deserialize, Serialize};

pub const API_URL: &str = "https://api.github.com";
pub type Error<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug)]
pub struct Github {
    client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub login: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    pub number: u32,
    pub html_url: String,
    pub state: String,
    pub title: String,
    pub body: Option<String>,
    pub user: User,
    pub comments: u32,
}

impl Display for Issue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "```\n")?;
        writeln!(f, "Number:     #{}", self.number)?;
        writeln!(f, "Url:        {}", self.html_url)?;
        writeln!(f, "State:      {}", self.state)?;
        writeln!(f, "Title:      {}", self.title)?;
        writeln!(f, "Author:     {}", self.user.login)?;
        writeln!(f, "Comments:   {}", self.comments)?;
        if let Some(body) = &self.body {
            writeln!(f, "Body:\n{}", body)?;
        }
        write!(f, "```")
    }
}

impl Default for Github {
    fn default() -> Self {
        Github {
            client: Client::new(),
        }
    }
}

impl Github {
    pub fn new() -> Github {
        Github::default()
    }

    pub async fn get_issue(&self, user: &str, repository: &str, issue: u32) -> Error<Issue> {
        Ok(self
            .client
            .get(format!(
                "{}/repos/{}/{}/issues/{}",
                API_URL, user, repository, issue,
            ))
            .header(USER_AGENT, "*The* Uwuki")
            .send()
            .await?
            .json()
            .await?)
    }

    // After some thought, this is *perfect*
    pub async fn get_snippet(
        &self,
        user: &str,
        repo: &str,
        file: &str,
        start: usize,
        end: Option<usize>,
    ) -> Error<String> {
        let content = self
            .client
            .get(format!(
                "https://raw.githubusercontent.com/{}/{}/{}",
                user, repo, file
            ))
            .send()
            .await?
            .text()
            .await?
            .lines()
            .skip(start - 1)
            .take(end.map(|e| e - start + 1).unwrap_or(1))
            .collect::<Vec<&str>>()
            .join("\n");
        let language = file.rsplit_once('.').map(|s| s.1).unwrap_or("");
        Ok(format!("```{}\n{}\n```", language, content))
    }
}
