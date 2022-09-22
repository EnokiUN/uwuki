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
        write!(f, "```\n")?;
        write!(f, "Number:     #{}\n", self.number)?;
        write!(f, "Url:        {}\n", self.html_url)?;
        write!(f, "State:      {}\n", self.state)?;
        write!(f, "Title:      {}\n", self.title)?;
        write!(f, "Author:     {}\n", self.user.login)?;
        write!(f, "Comments:   {}\n", self.comments)?;
        if let Some(body) = &self.body {
            write!(f, "Body:\n{}\n", body)?;
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

    pub async fn get_issue(&self, repository: String, issue: u32) -> Error<Issue> {
        Ok(self
            .client
            .get(format!(
                "{}/repos/eludris/{}/issues/{}",
                API_URL, repository, issue,
            ))
            .header(USER_AGENT, "*The* Uwuki")
            .send()
            .await?
            .json()
            .await?)
    }
}
