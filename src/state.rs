use std::{error::Error, fmt::Display, sync::Arc};

use eludrs::{todel::Message, HttpClient};
use reqwest::Client;

pub type State = Arc<UwukiState>;

#[derive(Clone, Debug)]
pub struct UwukiState {
    pub http: HttpClient,
    pub client: Client,
    pub github_token: Option<String>,
}

impl UwukiState {
    pub async fn send(
        &self,
        content: impl Display,
    ) -> Result<Message, Box<dyn Error + Send + Sync>> {
        self.http.send(content).await
    }
}
