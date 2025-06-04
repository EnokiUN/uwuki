use eludrs::models::Message;
use uwuki_macros::command;

use crate::{command_handler::CommandResult, state::State};

#[command]
#[uwuki(description = "Bans someone because they deserve it")]
#[uwuki(usage = "ban <target>")]
pub async fn ban(state: State, msg: Message, args: Option<String>) -> CommandResult {
    if let Some(args) = args {
        state
            .send(msg.channel.get_id(), format!("Banned {} :hammer:", args))
            .await?;
    }

    Ok(())
}

#[command]
#[uwuki(description = "Unbans someone because fuck you")]
#[uwuki(usage = "unban <target>")]
pub async fn unban(state: State, msg: Message, args: Option<String>) -> CommandResult {
    if let Some(args) = args {
        state
            .send(
                msg.channel.get_id(),
                format!("unBanned {} un:hammer:", args),
            )
            .await?;
    }

    Ok(())
}

#[command]
#[uwuki(description = "no horny bonk")]
#[uwuki(usage = "bonk <target>")]
pub async fn bonk(state: State, msg: Message, args: Option<String>) -> CommandResult {
    if let Some(args) = args {
        state
            .send(msg.channel.get_id(), format!("Bonkned {} :hammer:", args))
            .await?;
    }

    Ok(())
}

#[command]
#[uwuki(description = "knob ynroh on")]
#[uwuki(usage = "unbonk <target>")]
pub async fn unbonk(state: State, msg: Message, args: Option<String>) -> CommandResult {
    if let Some(args) = args {
        state
            .send(
                msg.channel.get_id(),
                format!("unBonkned {} un:hammer:", args),
            )
            .await?;
    }

    Ok(())
}
