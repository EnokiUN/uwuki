use eludrs::models::Message;
use rand::seq::SliceRandom;
use uwuki_macros::command;

use crate::{command_handler::CommandResult, state::State, utils::get_arg};

const OUTCOMES: [&str; 5] = [
    "It was super effective, rip.",
    "That was the most mid thing I've evew seen",
    "It wasn't effective at all L",
    "It missed... how'd you even miss that????",
    "Then they proceeded to fricking missfire it back at themselves, pog",
];

#[command]
#[uwuki(name = "use")]
#[uwuki(description = "Uses something on something ;)")]
#[uwuki(usage = "use <thing> <target>")]
pub async fn apply(state: State, msg: Message, args: Option<String>) -> CommandResult {
    if let Some(args) = args {
        let mut content = args.to_string();
        let thing = get_arg(&mut content);
        if content.is_empty() {
            state
                .send(msg.channel.get_id(), format!(
                    "**{}** used **{}** on **themselves** like an idiot and then **fricking imploded uwu**",
                    msg.author, thing
                ))
                .await?;
        } else {
            state
                .send(
                    msg.channel.get_id(),
                    format!(
                        "**{}** used **{}** on **{}**.\n\n{}",
                        msg.author,
                        thing,
                        content,
                        OUTCOMES
                            .choose(&mut *state.rng.lock().await)
                            .unwrap_or(&OUTCOMES[0])
                    ),
                )
                .await?;
        }
    }

    Ok(())
}
