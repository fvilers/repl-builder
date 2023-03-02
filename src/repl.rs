use std::{collections, result};

use crate::{command::Command, repl_error::ReplError};

pub type ThunkResult = result::Result<Option<String>, ReplError>;
pub type Thunk = fn() -> ThunkResult;

#[derive(Debug)]
pub struct Repl {
    _thunks: collections::HashMap<String, Thunk>,
}

impl Repl {
    pub fn new(commands: Vec<Command>) -> Self {
        // let commands: Vec<(String, Thunk)> = commands.iter().map(|c| (c.name, c.thunk)).collect();
        let mut thunks = collections::HashMap::with_capacity(commands.len());

        for command in commands {
            thunks.insert(command.name, command.thunk);
        }

        Self { _thunks: thunks }
    }

    pub fn run(self) -> result::Result<(), ReplError> {
        Err(ReplError::Run)
    }
}
