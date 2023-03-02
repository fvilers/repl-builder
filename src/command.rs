use crate::repl::Thunk;

#[derive(Debug)]
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
