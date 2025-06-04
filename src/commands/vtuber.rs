use eludrs::models::Message;
use uwuki_macros::command;

use crate::{command_handler::CommandResult, state::State};

#[command]
#[uwuki(description = "Does the waa")]
#[uwuki(usage = "waa")]
pub async fn waa(state: State, msg: Message, _: Option<String>) -> CommandResult {
    state.send(msg.channel.get_id(), "desuwa!").await?;

    Ok(())
}

#[command]
#[uwuki(description = "For when you're feeling a bit STELLAR STELLAR")]
#[uwuki(usage = "stellar")]
pub async fn stellar(state: State, msg: Message, _: Option<String>) -> CommandResult {
    state
        .send(
            msg.channel.get_id(),
            "<https://www.youtube.com/watch?v=a51VH9BYzZA>",
        )
        .await?;

    Ok(())
}
