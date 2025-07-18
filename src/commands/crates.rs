use eludrs::models::Message;
use uwuki_macros::command;

use crate::{command_handler::CommandResult, crates::Crates, state::State};

#[command]
#[uwuki(name = "crate")]
#[uwuki(description = "Gives you info on a crate from crates.io")]
#[uwuki(usage = "crate <crate name>")]
pub async fn crates(state: State, msg: Message, args: Option<String>) -> CommandResult {
    if let Some(args) = args {
        state
            .send(
                msg.channel.get_id(),
                state.get_crate(args).await?.to_string(),
            )
            .await?;
    }

    Ok(())
}
