use std::sync::Arc;

use eludrs::{todel::Message, HttpClient};
use uwuki_macros::command;

use crate::command_handler::CommandResult;

#[command]
#[uwuki(description = "Bans someone because they deserve it")]
#[uwuki(usage = "ban <target>")]
pub async fn ban(client: Arc<HttpClient>, _: Message, args: Option<String>) -> CommandResult {
    if let Some(args) = args {
        client.send(format!("Banned {} :hammer:", args)).await?;
    }

    Ok(())
}

#[command]
#[uwuki(description = "Unbans someone because fuck you")]
#[uwuki(usage = "unban <target>")]
pub async fn unban(client: Arc<HttpClient>, _: Message, args: Option<String>) -> CommandResult {
    if let Some(args) = args {
        client.send(format!("unBanned {} un:hammer:", args)).await?;
    }

    Ok(())
}

#[command]
#[uwuki(description = "no horny bonk")]
#[uwuki(usage = "bonk <target>")]
pub async fn bonk(client: Arc<HttpClient>, _: Message, args: Option<String>) -> CommandResult {
    if let Some(args) = args {
        client.send(format!("Bonkned {} :hammer:", args)).await?;
    }

    Ok(())
}

#[command]
#[uwuki(description = "knob ynroh on")]
#[uwuki(usage = "unbonk <target>")]
pub async fn unbonk(client: Arc<HttpClient>, _: Message, args: Option<String>) -> CommandResult {
    if let Some(args) = args {
        client
            .send(format!("unBonkned {} un:hammer:", args))
            .await?;
    }

    Ok(())
}
