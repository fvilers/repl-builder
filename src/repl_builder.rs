use std::fmt;

use crate::{command::Command, prompt, repl::Repl};

pub struct ReplBuilder<Context = ()> {
    commands: Vec<Command<Context>>,
    context: Context,
    prompt: Box<dyn fmt::Display>,
}

impl Default for ReplBuilder<()> {
    fn default() -> Self {
        ReplBuilder::new(())
    }
}

impl<Context> ReplBuilder<Context> {
    pub fn new(context: Context) -> Self {
        let commands = vec![];

        Self {
            commands,
            context,
            prompt: Box::<prompt::Prompt>::default(),
        }
    }

    pub fn add_command(mut self, command: Command<Context>) -> Self {
        self.commands.push(command);
        self
    }

    pub fn with_prompt(mut self, prompt: impl fmt::Display + 'static) -> Self {
        self.prompt = Box::new(prompt);
        self
    }

    pub fn build(self) -> Repl<Context> {
        Repl::new(self.commands, self.context, self.prompt)
    }
}
