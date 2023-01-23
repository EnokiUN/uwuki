use eludrs::todel::Message;
use uwuki_macros::command;

use crate::{command_handler::CommandResult, state::State, utils::get_arg};

#[command]
#[uwuki(description = "Imposters your imposter")]
#[uwuki(usage = "imposter <author> <content> | imposter <content>")]
pub async fn imposter(state: State, _: Message, args: Option<String>) -> CommandResult {
    if let Some(args) = args {
        let mut content = args.to_string();
        let author = get_arg(&mut content);
        if author.len() < 2 || author.len() > 32 {
            state
                .send("The author name should be between 2-32 characters long *b..baka!* >//<")
                .await?;
        } else if content.is_empty() {
            state.send_message(author, "I am sus").await?;
        } else {
            state.send_message(author, content).await?;
        }
    }

    Ok(())
}
