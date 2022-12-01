use std::fmt::Display;

use reqwest::{header::USER_AGENT, Client};
use serde::{Deserialize, Serialize};

pub const API_URL: &str = "https://api.github.com";
pub type Error<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug)]
pub struct Github {
    client: Client,
    token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub login: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    pub number: u32,
    #[serde(rename = "html_url")]
    pub url: String,
    pub state: String,
    pub title: String,
    pub body: Option<String>,
    pub user: User,
    pub comments: u32,
}

impl Display for Issue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "```")?;
        writeln!(f, "Number:         #{}", self.number)?;
        writeln!(f, "Url:            {}", self.url)?;
        writeln!(f, "State:          {}", self.state)?;
        writeln!(f, "Title:          {}", self.title)?;
        writeln!(f, "Author:         {}", self.user.login)?;
        writeln!(f, "Comments:       {}", self.comments)?;
        if let Some(body) = &self.body {
            writeln!(f, "Body:\n{}", body)?;
        }
        write!(f, "```")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct License {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    #[serde(rename = "full_name")]
    pub name: String,
    #[serde(rename = "html_url")]
    pub url: String,
    pub description: Option<String>,
    pub homepage: Option<String>,
    #[serde(rename = "stargazers_count")]
    pub stars: u32,
    pub language: Option<String>,
    #[serde(rename = "open_issues_count")]
    pub issues: u32,
    pub license: Option<License>,
}

impl Display for Repository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "```")?;
        writeln!(f, "{}", self.name)?;
        writeln!(f, "Url:            {}", self.url)?;
        if let Some(homepage) = &self.homepage {
            if !homepage.is_empty() {
                writeln!(f, "Homepage:       {}", homepage)?;
            }
        }
        if let Some(description) = &self.description {
            writeln!(f, "Description:    {}", description)?;
        }
        writeln!(f, "Stars:          {}", self.stars)?;
        writeln!(f, "Open issues:    {}", self.issues)?;
        if let Some(language) = &self.language {
            writeln!(f, "Language:       {}", language)?;
        }
        if let Some(license) = &self.license {
            writeln!(f, "License:        {}", license.name)?;
        }
        write!(f, "```")
    }
}

impl Github {
    pub fn new(token: Option<String>) -> Self {
        Self {
            client: Client::new(),
            token,
        }
    }

    pub async fn get_issue(&self, user: &str, repository: &str, issue: u32) -> Error<Issue> {
        log::debug!("Fetching issue {} at {}/{}", issue, user, repository);
        let builder = self
            .client
            .get(format!(
                "{}/repos/{}/{}/issues/{}",
                API_URL, user, repository, issue,
            ))
            .header(USER_AGENT, "*The* Uwuki");
        let builder = if let Some(token) = &self.token {
            builder.bearer_auth(token)
        } else {
            builder
        };
        Ok(builder.send().await?.json().await?)
    }

    pub async fn get_repo(&self, user: &str, repository: &str) -> Error<Repository> {
        log::debug!("Fetching repository {}/{}", user, repository);
        let builder = self
            .client
            .get(format!("{}/repos/{}/{}", API_URL, user, repository))
            .header(USER_AGENT, "*The* Uwuki");
        let builder = if let Some(token) = &self.token {
            builder.bearer_auth(token)
        } else {
            builder
        };
        Ok(builder.send().await?.json().await?)
    }

    // After some thought, this is *perfect*
    pub async fn get_snippet(
        &self,
        user: &str,
        repo: &str,
        file: &str,
        start: u32,
        end: Option<u32>,
    ) -> Error<String> {
        log::info!(
            "Fetching code snippet at {}/{} in {} from lines {} to {:?}",
            user,
            repo,
            file,
            start,
            end
        );
        let builder = self
            .client
            .get(format!(
                "https://raw.githubusercontent.com/{}/{}/{}",
                user, repo, file
            ))
            .header(USER_AGENT, "*The* Uwuki");
        let builder = if let Some(token) = &self.token {
            builder.bearer_auth(token)
        } else {
            builder
        };
        let content = builder
            .send()
            .await?
            .text()
            .await?
            .lines()
            .skip(start as usize - 1)
            .take(end.map(|e| e as usize - start as usize + 1).unwrap_or(1))
            .collect::<Vec<&str>>()
            .join("\n");
        let language = file.rsplit_once('.').map(|s| s.1).unwrap_or("");
        Ok(format!("```{}\n{}\n```", language, content))
    }
}
