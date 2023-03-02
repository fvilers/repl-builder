use crate::{command::Command, repl::Repl};

#[derive(Default)]
pub struct ReplBuilder<Context = ()> {
    commands: Vec<Command<Context>>,
    context: Context,
}

impl<Context> ReplBuilder<Context> {
    pub fn new(context: Context) -> Self {
        let commands = vec![];

        Self { commands, context }
    }

    pub fn add_command(mut self, command: Command<Context>) -> Self {
        self.commands.push(command);
        self
    }

    pub fn build(self) -> Repl<Context> {
        Repl::new(self.commands, self.context)
    }
}
