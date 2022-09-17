
use std::fmt::Display;

use reqwest::Client;
use todel::models::{Info, Message};

const REST_URL: &'static str = "https://eludris.tooty.xyz";

type Error<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug)]
pub struct HttpClient {
    client: Client,
    rest_url: String,
    pub user_name: Option<String>,
}

impl Default for HttpClient {
    fn default() -> Self {
        HttpClient {
            client: Client::new(),
            rest_url: REST_URL.to_string(),
            user_name: None,
        }
    }
}

impl HttpClient {
    pub fn new() -> HttpClient {
        HttpClient::default()
    }

    pub fn name(mut self, name: String) -> HttpClient {
        self.user_name = Some(name);
        self
    }

    pub fn rest_url(mut self, url: String) -> HttpClient {
        self.rest_url = url;
        self
    }

    pub async fn get_instance_info(&self) -> Error<Info> {
        Ok(self.client.get(&self.rest_url).send().await?.json().await?)
    }

    pub async fn send_message<T: Display, C: Display>(
        &self,
        author: T,
        content: C,
    ) -> Error<Message> {
        Ok(self
            .client
            .post(format!("{}/messages", self.rest_url))
            .json(&Message {
                author: author.to_string(),
                content: content.to_string(),
            })
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn send<T: Display>(&self, content: T) -> Error<Message> {
        self.send_message(
            &self
                .user_name
                .clone()
                .expect("You have to specifiy a name to run this function"),
            content,
        )
        .await
    }
}
