use std::result;

use crate::{repl::Thunk, repl_error::ReplError};

pub type CommandResult = result::Result<Option<String>, ReplError>;

pub struct Command {
    pub(crate) name: String,
    pub(crate) thunk: Thunk,
}

impl Command {
    pub fn new(name: impl Into<String>, thunk: Thunk) -> Self {
        Self {
            name: name.into(),
            thunk,
        }
    }
}
