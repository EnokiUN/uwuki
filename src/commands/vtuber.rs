use std::sync::Arc;

use eludrs::{todel::Message, HttpClient};
use uwuki_macros::command;

use crate::command_handler::CommandResult;

#[command]
#[uwuki(description = "Does the waa")]
#[uwuki(usage = "waa")]
pub async fn waa(client: Arc<HttpClient>, _: Message, _: Option<String>) -> CommandResult {
    client.send("desuwa!").await?;

    Ok(())
}

#[command]
#[uwuki(description = "For when you're feeling a bit STELLAR STELLAR")]
#[uwuki(usage = "stellar")]
pub async fn stellar(client: Arc<HttpClient>, _: Message, _: Option<String>) -> CommandResult {
    client
        .send("<https://www.youtube.com/watch?v=a51VH9BYzZA>")
        .await?;

    Ok(())
}
