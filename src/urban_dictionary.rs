use std::fmt::Display;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::state::UwukiState;

pub const API_URL: &str = "https://api.urbandictionary.com/v0";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Definition {
    definition: String,
    thumbs_up: u32,
    author: String,
    word: String,
    example: String,
    thumbs_down: u32,
}

impl Display for Definition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "__{}__", self.word)?;
        writeln!(f, "{}\n", self.definition)?;
        writeln!(f, "Example:\n{}\n", self.example)?;
        write!(
            f,
            "By {} - {} points",
            self.author,
            self.thumbs_up - self.thumbs_down
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefineResponse {
    list: Vec<Definition>,
}

#[async_trait]
pub trait UrbanDictionary {
    async fn define(&self, term: &str) -> anyhow::Result<Vec<Definition>>;
}

#[async_trait]
impl UrbanDictionary for UwukiState {
    async fn define(&self, term: &str) -> anyhow::Result<Vec<Definition>> {
        Ok(self
            .client
            .get(format!("{}/define?term={}", API_URL, term))
            .send()
            .await?
            .json::<DefineResponse>()
            .await?
            .list)
    }
}
