use eludrs::todel::Message;
use uwuki_macros::command;

use crate::{command_handler::CommandResult, state::State};

#[command]
#[uwuki(description = "Does the waa")]
#[uwuki(usage = "waa")]
pub async fn waa(state: State, _: Message, _: Option<String>) -> CommandResult {
    state.send("desuwa!").await?;

    Ok(())
}

#[command]
#[uwuki(description = "For when you're feeling a bit STELLAR STELLAR")]
#[uwuki(usage = "stellar")]
pub async fn stellar(state: State, _: Message, _: Option<String>) -> CommandResult {
    state
        .send("<https://www.youtube.com/watch?v=a51VH9BYzZA>")
        .await?;

    Ok(())
}
