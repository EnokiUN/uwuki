use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use futures::{stream::SplitStream, SinkExt, Stream, StreamExt};
use todel::models::Message;
use tokio::{net::TcpStream, spawn, time};
use tokio_tungstenite::{
    connect_async, tungstenite::Message as WSMessage, MaybeTlsStream, WebSocketStream,
};

use crate::models::Error;

/// The default gateway url
pub const GATEWAY_URL: &'static str = "wss://eludris.tooty.xyz/ws/";

/// A Stream of Pandemonium events
#[derive(Debug)]
pub struct Events(SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>);

/// Simple gateway client
#[derive(Debug)]
pub struct GatewayClient {
    pub gateway_url: String,
}

impl Default for GatewayClient {
    fn default() -> Self {
        GatewayClient {
            gateway_url: GATEWAY_URL.to_string(),
        }
    }
}

impl GatewayClient {
    /// Create a new GatewayClient
    pub fn new() -> GatewayClient {
        GatewayClient::default()
    }

    /// Change the url of the GatewayClient
    ///
    /// # Example:
    /// ```rust
    /// use uwuki::GatewayClient;
    ///
    /// let client = GatewayClient::new().gateway_url("http://0.0.0.0:7160".to_string());
    ///
    /// assert_eq!(client.gateway_url, "http://0.0.0.0:7160".to_string())
    /// ```
    pub fn gateway_url(mut self, url: String) -> GatewayClient {
        self.gateway_url = url;
        self
    }

    /// Start a connection to the Pandemonium and return [`Events`]
    pub async fn get_events(&self) -> Error<Events> {
        let (socket, _) = connect_async(&self.gateway_url).await?;
        let (mut tx, rx) = socket.split();
        spawn(async move {
            loop {
                tx.send(WSMessage::Ping(vec![]))
                    .await
                    .expect("Couldn't ping Pandemonium");
                time::sleep(Duration::from_secs(20)).await;
            }
        });
        Ok(Events(rx))
    }
}

impl Stream for Events {
    type Item = Message;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            match self.0.poll_next_unpin(cx) {
                Poll::Ready(msg) => {
                    if let Some(Ok(WSMessage::Text(msg))) = msg {
                        if let Ok(msg) = serde_json::from_str(&msg) {
                            break Poll::Ready(Some(msg));
                        }
                    }
                }
                Poll::Pending => break Poll::Pending,
            }
        }
    }
}
