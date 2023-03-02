mod command;
mod prompt;
mod repl;
mod repl_builder;
mod repl_error;

pub mod prelude {
    pub use crate::command::{Command, CommandResult};
    pub use crate::repl_builder::ReplBuilder;
    pub use crate::repl_error::ReplError;
}
