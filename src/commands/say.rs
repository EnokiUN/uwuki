use std::sync::Arc;

use eludrs::{todel::Message, HttpClient};
use uwuki_macros::command;

use crate::command_handler::CommandResult;

#[command]
#[uwuki(description = "Says what you need to say")]
#[uwuki(usage = "say <shit here>")]
pub async fn say(client: Arc<HttpClient>, _: Message, args: Option<String>) -> CommandResult {
    if let Some(args) = args {
        client.send(args).await?;
    }

    Ok(())
}
