use eludrs::todel::Message;
use uwuki_macros::command;

use crate::{command_handler::CommandResult, state::State};

#[command]
#[uwuki(description = "Sends the link to the Eludris website")]
#[uwuki(usage = "info")]
pub async fn info(state: State, _: Message, _: Option<String>) -> CommandResult {
    state.send("<https://eludris.pages.dev>").await?;

    Ok(())
}

#[command]
#[uwuki(description = "Sends the link to the Eludris blog")]
#[uwuki(usage = "blog")]
pub async fn blog(state: State, _: Message, _: Option<String>) -> CommandResult {
    state.send("<https://eludris.pages.dev/blog>").await?;

    Ok(())
}

#[command]
#[uwuki(description = "Sends the link to the Eludris docs")]
#[uwuki(usage = "docs")]
pub async fn docs(state: State, _: Message, _: Option<String>) -> CommandResult {
    state.send("https://eludris.github.io/docs").await?;

    Ok(())
}

#[command]
#[uwuki(description = "Sends the link to the Eludris awesome repository")]
#[uwuki(alias = "awe")]
#[uwuki(usage = "awesome|awe")]
pub async fn awesome(state: State, _: Message, _: Option<String>) -> CommandResult {
    state.send("<https://github.com/eludris/awesome>").await?;

    Ok(())
}

#[command]
#[uwuki(description = "Sends the link to the Eludris community organisation")]
#[uwuki(usage = "community")]
pub async fn community(state: State, _: Message, _: Option<String>) -> CommandResult {
    state.send("<https://github.com/eludris-community>").await?;

    Ok(())
}

#[command]
#[uwuki(description = "Sends the link to the Eludris organisation")]
#[uwuki(alias = "gh")]
#[uwuki(alias = "org")]
#[uwuki(usage = "github|gh|org")]
pub async fn github(state: State, _: Message, _: Option<String>) -> CommandResult {
    state.send("<https://github.com/eludris>").await?;

    Ok(())
}

#[command]
#[uwuki(description = "Sends the link to the Eludris subreddit")]
#[uwuki(usage = "reddit")]
pub async fn reddit(state: State, _: Message, _: Option<String>) -> CommandResult {
    state.send("<https://reddit.com/r/eludris>").await?;

    Ok(())
}

#[command]
#[uwuki(description = "Sends the link to the Eludris Twitter account")]
#[uwuki(usage = "twitter")]
pub async fn twitter(state: State, _: Message, _: Option<String>) -> CommandResult {
    state.send("<https://twitter.com/eludris>").await?;

    Ok(())
}

#[command]
#[uwuki(description = "Sends the link Pengin, the official Eludris web client")]
#[uwuki(alias = "pengin")]
#[uwuki(usage = "client")]
pub async fn client(state: State, _: Message, _: Option<String>) -> CommandResult {
    state.send("https://elu.pages.dev").await?;

    Ok(())
}
