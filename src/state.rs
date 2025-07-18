use std::{fmt::Display, sync::Arc};

use anyhow::Result;
use eludrs::{todel::models::Message, HttpClient};
use rand::rngs::StdRng;
use reqwest::Client;
use tokio::sync::Mutex;

pub type State = Arc<UwukiState>;

#[derive(Debug)]
pub struct UwukiState {
    pub http: HttpClient,
    pub client: Client,
    pub github_token: Option<String>,
    pub rng: Mutex<StdRng>,
}

impl UwukiState {
    pub async fn send(&self, channel_id: u64, content: impl Display) -> Result<Message> {
        self.http.send_message(channel_id, content).await
    }
}
