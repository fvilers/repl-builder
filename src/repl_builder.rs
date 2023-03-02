use crate::{command::Command, repl::Repl};

#[derive(Debug, Default)]
pub struct ReplBuilder {
    commands: Vec<Command>,
}

impl ReplBuilder {
    pub fn add_command(mut self, command: Command) -> Self {
        self.commands.push(command);
        self
    }

    pub fn build(self) -> Repl {
        Repl::new(self.commands)
    }
}
