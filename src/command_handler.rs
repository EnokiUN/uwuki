use std::{collections::HashMap, fmt::Debug, sync::Arc};

use eludrs::{todel::Message, HttpClient};
use futures::future::BoxFuture;

pub type CommandResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone)]
pub struct Command<'a> {
    pub names: &'a [&'a str],
    pub description: &'a str,
    pub usage: &'a str,
    pub func: fn(Arc<HttpClient>, Message, Option<String>) -> BoxFuture<'a, CommandResult>,
}

impl<'a> Debug for Command<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Command")
            .field("names", &self.names)
            .field("description", &self.description)
            .field("usage", &self.usage)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct CommandRunner<'a> {
    prefix: String,
    commands: Vec<Command<'a>>,
    lookup: HashMap<&'a str, usize>,
}

impl<'a> CommandRunner<'a> {
    pub fn new(prefix: String) -> Self {
        Self {
            prefix,
            commands: vec![],
            lookup: HashMap::new(),
        }
    }

    pub fn add_command(&mut self, command: Command<'a>) {
        for name in command.names {
            self.lookup.insert(name, self.commands.len());
        }
        self.commands.push(command);
    }

    #[allow(dead_code)]
    pub fn command(mut self, command: Command<'a>) -> Self {
        self.add_command(command);
        self
    }

    pub fn commands(mut self, commands: &[Command<'a>]) -> Self {
        for command in commands.iter().cloned() {
            self.add_command(command);
        }
        self
    }

    pub fn get_commands(&self) -> &Vec<Command<'a>> {
        &self.commands
    }

    pub async fn run_command(&self, client: Arc<HttpClient>, message: Message) -> CommandResult {
        if message.content.starts_with(&self.prefix) {
            let (command, args) = message.content[self.prefix.len()..]
                .split_once([' ', '\n'])
                .map(|(cmd, args)| (cmd, Some(args.to_string())))
                .unwrap_or((message.content[self.prefix.len()..].trim(), None));
            if let Some(index) = self.lookup.get(command) {
                if let Some(command) = self.commands.get(*index) {
                    (command.func)(client, message, args).await?;
                }
            }
        }
        Ok(())
    }
}
