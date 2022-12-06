use std::fmt::Display;

use reqwest::Client;
use serde::{Deserialize, Serialize};

pub const API_URL: &str = "https://play.rust-lang.org/execute";
pub type Error<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug)]
pub struct Playground {
    client: Client,
}

//  {
//    "channel":"stable",
//    "mode":"debug",
//    "edition":"2021",
//    "crateType":"bin",
//    "tests":false,
//    "code":"[OMMITED]",
//    "backtrace":false
//  }
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaygroundRequest {
    channel: String,
    mode: String,
    edition: String,
    crate_type: String,
    tests: bool,
    code: String,
    backtrace: bool,
}

impl Default for PlaygroundRequest {
    fn default() -> Self {
        Self {
            channel: "stable".to_string(),
            mode: "debug".to_string(),
            edition: "2021".to_string(),
            crate_type: "bin".to_string(),
            tests: false,
            code: r#"fn main() { println!("Hello, World!"); }"#.to_string(),
            backtrace: false,
        }
    }
}

impl PlaygroundRequest {
    pub fn new(code: String) -> Self {
        Self {
            code,
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaygroundResponse {
    success: bool,
    stderr: String,
    stdout: String,
}

impl Display for PlaygroundResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Stderr:\n```\n{}```\n", self.stderr)?;
        write!(f, "Stdout:\n```\n{}```\n", self.stdout)
    }
}

impl Playground {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn execute(&self, request: PlaygroundRequest) -> Error<PlaygroundResponse> {
        Ok(self
            .client
            .post(API_URL)
            .json(&request)
            .send()
            .await?
            .json()
            .await?)
    }
}
