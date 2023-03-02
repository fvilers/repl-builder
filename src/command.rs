use std::result;

pub enum CommandError {}
pub type Thunk = fn() -> result::Result<Option<String>, CommandError>;

#[derive(Debug)]
pub struct Command {
    _name: String,
    _thunk: Thunk,
}

impl Command {
    pub fn new(name: impl Into<String>, thunk: Thunk) -> Self {
        Self {
            _name: name.into(),
            _thunk: thunk,
        }
    }
}
