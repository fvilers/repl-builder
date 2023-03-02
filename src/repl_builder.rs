use crate::command::Command;

#[derive(Debug, Default)]
pub struct ReplBuilder {
    commands: Vec<Command>,
}

impl ReplBuilder {
    pub fn add_command(mut self, command: Command) -> Self {
        self.commands.push(command);
        self
    }
}
