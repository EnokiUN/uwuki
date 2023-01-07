use std::sync::Arc;

use eludrs::{todel::Message, HttpClient};
use uwuki_macros::command;

use crate::{
    command_handler::CommandResult,
    playground::{Playground, PlaygroundRequest},
};

#[command]
#[uwuki(description = "Says what you need to say")]
#[uwuki(usage = "say <shit here>")]
pub async fn exec(client: Arc<HttpClient>, _: Message, args: Option<String>) -> CommandResult {
    let playground = Playground::new();
    if let Some(code) = args {
        client
            .send(
                playground
                    .execute(PlaygroundRequest::new(
                        code.replace("```rs", "").replace("```", "").to_string(),
                    ))
                    .await?
                    .to_string(),
            )
            .await?;
    }

    Ok(())
}
