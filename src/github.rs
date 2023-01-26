use std::fmt::Display;

use async_trait::async_trait;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};

use crate::state::UwukiState;

pub const API_URL: &str = "https://api.github.com";

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "login")]
    pub name: String,
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
        writeln!(f, "<{}>", self.url)?;
        writeln!(f, "```")?;
        writeln!(f, "Number:         #{}", self.number)?;
        writeln!(f, "State:          {}", self.state)?;
        writeln!(f, "Title:          {}", self.title)?;
        writeln!(f, "Author:         {}", self.user.name)?;
        if self.comments > 0 {
            writeln!(f, "Comments:       {}", self.comments)?;
        }
        if let Some(body) = &self.body {
            writeln!(f, "Body:\n{}", body)?;
        }
        write!(f, "```")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    #[serde(rename = "html_url")]
    pub url: String,
    pub user: User,
    pub body: String,
}

impl Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "<{}>", self.url)?;
        writeln!(f, "```")?;
        writeln!(f, "[{}]:", self.user.name)?;
        writeln!(f, "{}", self.body)?;
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
        writeln!(f, "<{}>", self.url)?;
        writeln!(f, "```")?;
        writeln!(f, "{}", self.name)?;
        if let Some(homepage) = &self.homepage {
            if !homepage.is_empty() {
                writeln!(f, "Homepage:       {}", homepage)?;
            }
        }
        if let Some(description) = &self.description {
            writeln!(f, "Description:    {}", description)?;
        }
        writeln!(f, "Stars:          {}", self.stars)?;
        if self.issues > 0 {
            writeln!(f, "Open issues:    {}", self.issues)?;
        }
        if let Some(language) = &self.language {
            writeln!(f, "Language:       {}", language)?;
        }
        if let Some(license) = &self.license {
            writeln!(f, "License:        {}", license.name)?;
        }
        write!(f, "```")
    }
}

#[async_trait]
pub trait GitHub {
    async fn get_issue(&self, user: &str, repository: &str, issue: u32) -> anyhow::Result<Issue>;
    async fn get_repo(&self, user: &str, repository: &str) -> anyhow::Result<Repository>;
    async fn get_comment(
        &self,
        user: &str,
        repository: &str,
        comment_type: &str,
        id: u32,
    ) -> anyhow::Result<Comment>;
    async fn get_snippet(
        &self,
        user: &str,
        repo: &str,
        file: &str,
        start: u32,
        end: Option<u32>,
    ) -> anyhow::Result<String>;
}

#[async_trait]
impl GitHub for UwukiState {
    async fn get_issue(&self, user: &str, repository: &str, issue: u32) -> anyhow::Result<Issue> {
        log::debug!("Fetching issue {} at {}/{}", issue, user, repository);
        let builder = self
            .client
            .get(format!(
                "{}/repos/{}/{}/issues/{}",
                API_URL, user, repository, issue,
            ))
            .header(USER_AGENT, "Uwuki (github.com/Enokiun)");
        let builder = if let Some(token) = &self.github_token {
            builder.bearer_auth(token)
        } else {
            builder
        };
        Ok(builder.send().await?.json().await?)
    }

    async fn get_repo(&self, user: &str, repository: &str) -> anyhow::Result<Repository> {
        log::debug!("Fetching repository {}/{}", user, repository);
        let builder = self
            .client
            .get(format!("{}/repos/{}/{}", API_URL, user, repository))
            .header(USER_AGENT, "Uwuki (github.com/Enokiun)");
        let builder = if let Some(token) = &self.github_token {
            builder.bearer_auth(token)
        } else {
            builder
        };
        Ok(builder.send().await?.json().await?)
    }

    async fn get_comment(
        &self,
        user: &str,
        repository: &str,
        comment_type: &str,
        id: u32,
    ) -> anyhow::Result<Comment> {
        log::debug!(
            "Fetching {} comment {} from {}/{}",
            comment_type,
            id,
            user,
            repository
        );
        let builder = self
            .client
            .get(format!(
                "{}/repos/{}/{}/{}/comments/{}",
                API_URL, user, repository, comment_type, id
            ))
            .header(USER_AGENT, "Uwuki (github.com/Enokiun)");
        let builder = if let Some(token) = &self.github_token {
            builder.bearer_auth(token)
        } else {
            builder
        };
        Ok(builder.send().await?.json().await?)
    }

    // After some thought, this is *perfect*
    async fn get_snippet(
        &self,
        user: &str,
        repo: &str,
        file: &str,
        start: u32,
        end: Option<u32>,
    ) -> anyhow::Result<String> {
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
            .header(USER_AGENT, "Uwuki (github.com/Enokiun)");
        let builder = if let Some(token) = &self.github_token {
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
