use eludrs::todel::Message;
use uwuki_macros::command;

use crate::{
    command_handler::CommandResult, state::State, urban_dictionary::UrbanDictionary, utils::get_arg,
};

#[command]
#[uwuki(description = "")]
#[uwuki(alias = "ud")]
#[uwuki(usage = "define <term> [page=1]")]
pub async fn define(state: State, _: Message, args: Option<String>) -> CommandResult {
    if let Some(mut args) = args {
        let term = get_arg(&mut args);
        let definitions = state.define(&term).await?;
        let page = args.split(' ').next().unwrap_or("1").parse::<usize>();
        if let Ok(page) = page {
            state
                .send(
                    definitions
                        .get(page - 1)
                        .map(|d| d.to_string())
                        .unwrap_or_else(|| "Definition not found".to_string()),
                )
                .await?;
        } else {
            state
                .send(
                    definitions
                        .first()
                        .map(|d| d.to_string())
                        .unwrap_or_else(|| "Unknown term".to_string()),
                )
                .await?;
        };
    };
    Ok(())
}
