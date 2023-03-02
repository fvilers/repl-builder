use std::result;

use crate::{repl::Thunk, repl_error::ReplError};

pub type CommandResult = result::Result<Option<String>, ReplError>;

pub struct Command<Context> {
    pub(crate) name: String,
    pub(crate) thunk: Thunk<Context>,
}

impl<Context> Command<Context> {
    pub fn new(name: impl Into<String>, thunk: Thunk<Context>) -> Self {
        Self {
            name: name.into(),
            thunk,
        }
    }
}
