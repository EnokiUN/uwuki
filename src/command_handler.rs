use std::{collections::HashMap, fmt::Debug, sync::Arc};

use eludrs::todel::Message;
use futures::future::BoxFuture;

pub type CommandResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone)]
pub struct Command<'a, S: Debug + Send + Sync + Clone> {
    pub names: &'a [&'a str],
    pub description: &'a str,
    pub usage: &'a str,
    pub func: fn(S, Message, Option<String>) -> BoxFuture<'a, CommandResult>,
}

impl<'a, S: Debug + Send + Sync + Clone> Debug for Command<'a, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Command")
            .field("names", &self.names)
            .field("description", &self.description)
            .field("usage", &self.usage)
            .finish()
    }
}

#[derive(Debug)]
pub struct CommandRunner<'a, S: Debug + Send + Sync> {
    prefix: String,
    commands: Vec<Command<'a, Arc<S>>>,
    state: Arc<S>,
    lookup: HashMap<&'a str, usize>,
}

impl<'a, S: Debug + Send + Sync> CommandRunner<'a, S> {
    pub fn new(prefix: String, state: Arc<S>) -> Self {
        Self {
            prefix,
            commands: vec![],
            state,
            lookup: HashMap::new(),
        }
    }

    pub fn add_command(&mut self, command: Command<'a, Arc<S>>) {
        for name in command.names {
            self.lookup.insert(name, self.commands.len());
        }
        self.commands.push(command);
    }

    #[allow(dead_code)]
    pub fn command(mut self, command: Command<'a, Arc<S>>) -> Self {
        self.add_command(command);
        self
    }

    pub fn commands(mut self, commands: &[Command<'a, Arc<S>>]) -> Self {
        for command in commands.iter().cloned() {
            self.add_command(command);
        }
        self
    }

    #[allow(dead_code)]
    pub fn state(mut self, state: Arc<S>) -> Self {
        self.state = state;
        self
    }

    pub fn get_commands(&self) -> &Vec<Command<'a, Arc<S>>> {
        &self.commands
    }

    pub fn get_command(&self, name: &str) -> Option<&Command<'a, Arc<S>>> {
        let index = self.lookup.get(name)?;
        self.commands.get(*index)
    }

    pub async fn run_command(&self, message: Message) -> CommandResult {
        if message.content.starts_with(&self.prefix) {
            let (command, args) = message.content[self.prefix.len()..]
                .split_once([' ', '\n'])
                .map(|(cmd, args)| (cmd, Some(args.to_string())))
                .unwrap_or((message.content[self.prefix.len()..].trim(), None));
            if let Some(index) = self.lookup.get(command) {
                if let Some(command) = self.commands.get(*index) {
                    (command.func)(Arc::clone(&self.state), message, args).await?;
                }
            }
        }
        Ok(())
    }
}
