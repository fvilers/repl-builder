use std::{collections, result};

use question::Question;

use crate::{
    command::{Command, CommandResult},
    repl_error::ReplError,
};

pub type Thunk<Context> = fn(args: Vec<&str>, &mut Context) -> CommandResult;

pub struct Repl<Context> {
    context: Context,
    thunks: collections::HashMap<String, Thunk<Context>>,
}

impl<Context> Repl<Context> {
    pub fn new(commands: Vec<Command<Context>>, context: Context) -> Self {
        let thunks =
            collections::HashMap::from_iter(commands.iter().map(|c| (c.name.to_owned(), c.thunk)));

        Self { context, thunks }
    }

    pub fn run(&mut self) -> result::Result<(), ReplError> {
        let mut question = Question::default();

        while let Some(user_input) = question.ask("> ")? {
            let mut segments = user_input.split_whitespace();

            if let Some(name) = segments.next() {
                match self.execute_command(name, segments.collect()) {
                    Ok(Some(output)) => println!("{}", output),
                    Ok(None) => {}
                    Err(e) => eprintln!("{}", e),
                }
            }
        }

        Ok(())
    }

    fn execute_command(&mut self, name: &str, args: Vec<&str>) -> CommandResult {
        match self.thunks.get(name) {
            Some(thunk) => thunk(args, &mut self.context),
            _ => Err(ReplError::CommandNotFound),
        }
    }
}
