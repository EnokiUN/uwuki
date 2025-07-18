use std::collections::HashMap;

use eludrs::models::Message;
use tokio::fs;
use uwuki_macros::command;

use crate::{command_handler::CommandResult, state::State};

#[command]
#[uwuki(description = "Provides you with the literal form of an Eludris amendment")]
#[uwuki(usage = "amendment <number>")]
pub async fn amendment(state: State, msg: Message, args: Option<String>) -> CommandResult {
    if let Some(number) = args {
        // this doesn't have to be a number, just don't tell em :D
        let amendments = fs::read_to_string("./assets/amendments.json").await?;
        let amendments: HashMap<String, String> = serde_json::from_str(&amendments)?;
        if let Some(amendment) = amendments.get(&number) {
            state
                .send(
                    msg.channel.get_id(),
                    format!("The {}th Eludris amendment:\n{}", number, amendment),
                )
                .await?;
        } else {
            state
                .send(msg.channel.get_id(), "Unknown amendment")
                .await?;
        }
    }
    Ok(())
}
