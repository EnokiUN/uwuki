mod amendments;
mod apply;
mod ban;
mod crates;
mod define;
mod exec;
mod imposter;
mod links;
mod say;
mod vtuber;

use std::sync::Arc;

use amendments::AMENDMENT_COMMAND;
use apply::APPLY_COMMAND;
use ban::{BAN_COMMAND, BONK_COMMAND, UNBAN_COMMAND, UNBONK_COMMAND};
use crates::CRATES_COMMAND;
use define::DEFINE_COMMAND;
use exec::EXEC_COMMAND;
use imposter::IMPOSTER_COMMAND;
use links::{
    AWESOME_COMMAND, BLOG_COMMAND, CLIENT_COMMAND, COMMUNITY_COMMAND, DOCS_COMMAND, GITHUB_COMMAND,
    INFO_COMMAND, REDDIT_COMMAND, TWITTER_COMMAND,
};
use say::SAY_COMMAND;
use vtuber::{STELLAR_COMMAND, WAA_COMMAND};

use crate::{command_handler::Command, state::UwukiState};

pub fn commands<'a>() -> Vec<Command<'a, Arc<UwukiState>>> {
    vec![
        AMENDMENT_COMMAND.clone(),
        APPLY_COMMAND.clone(),
        BAN_COMMAND.clone(),
        BONK_COMMAND.clone(),
        UNBAN_COMMAND.clone(),
        UNBONK_COMMAND.clone(),
        CRATES_COMMAND.clone(),
        DEFINE_COMMAND.clone(),
        EXEC_COMMAND.clone(),
        IMPOSTER_COMMAND.clone(),
        AWESOME_COMMAND.clone(),
        BLOG_COMMAND.clone(),
        CLIENT_COMMAND.clone(),
        COMMUNITY_COMMAND.clone(),
        DOCS_COMMAND.clone(),
        GITHUB_COMMAND.clone(),
        INFO_COMMAND.clone(),
        REDDIT_COMMAND.clone(),
        TWITTER_COMMAND.clone(),
        SAY_COMMAND.clone(),
        WAA_COMMAND.clone(),
        STELLAR_COMMAND.clone(),
    ]
}
