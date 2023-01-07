use std::sync::Arc;

use eludrs::{todel::Message, HttpClient};
use uwuki_macros::command;

use crate::command_handler::CommandResult;

#[command]
#[uwuki(description = "Sends the link to the Eludris website")]
#[uwuki(usage = "info")]
pub async fn info(client: Arc<HttpClient>, _: Message, _: Option<String>) -> CommandResult {
    client.send("<https://eludris.pages.dev>").await?;

    Ok(())
}

#[command]
#[uwuki(description = "Sends the link to the Eludris blog")]
#[uwuki(usage = "blog")]
pub async fn blog(client: Arc<HttpClient>, _: Message, _: Option<String>) -> CommandResult {
    client.send("<https://eludris.pages.dev/blog>").await?;

    Ok(())
}

#[command]
#[uwuki(description = "Sends the link to the Eludris docs")]
#[uwuki(usage = "docs")]
pub async fn docs(client: Arc<HttpClient>, _: Message, _: Option<String>) -> CommandResult {
    client.send("https://eludris.github.io/docs").await?;

    Ok(())
}

#[command]
#[uwuki(description = "Sends the link to the Eludris awesome repository")]
#[uwuki(alias = "awe")]
#[uwuki(usage = "awesome|awe")]
pub async fn awesome(client: Arc<HttpClient>, _: Message, _: Option<String>) -> CommandResult {
    client.send("<https://github.com/eludris/awesome>").await?;

    Ok(())
}

#[command]
#[uwuki(description = "Sends the link to the Eludris community organisation")]
#[uwuki(usage = "community")]
pub async fn community(client: Arc<HttpClient>, _: Message, _: Option<String>) -> CommandResult {
    client
        .send("<https://github.com/eludris-community>")
        .await?;

    Ok(())
}

#[command]
#[uwuki(description = "Sends the link to the Eludris organisation")]
#[uwuki(alias = "gh")]
#[uwuki(alias = "org")]
#[uwuki(usage = "github|gh|org")]
pub async fn github(client: Arc<HttpClient>, _: Message, _: Option<String>) -> CommandResult {
    client.send("<https://github.com/eludris>").await?;

    Ok(())
}
