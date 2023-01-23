use eludrs::todel::Message;
use uwuki_macros::command;

use crate::{
    command_handler::CommandResult,
    playground::{Playground, PlaygroundRequest},
    state::State,
};

#[command]
#[uwuki(description = "Says what you need to say")]
#[uwuki(usage = "say <shit here>")]
pub async fn exec(state: State, _: Message, args: Option<String>) -> CommandResult {
    if let Some(code) = args {
        state
            .send(
                state
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
