use std::{error, fmt, io};

#[derive(Debug)]
pub enum ReplError {
    Run,
    CommandNotFound,
    MissingArgument(String),
    IO(io::Error),
}

impl fmt::Display for ReplError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReplError::Run => write!(f, "Error while running the REPL"),
            ReplError::CommandNotFound => write!(f, "Command not found"),
            ReplError::MissingArgument(name) => write!(f, "Missing argument `{}`", name),
            ReplError::IO(e) => e.fmt(f),
        }
    }
}

impl error::Error for ReplError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            ReplError::Run => None,
            ReplError::CommandNotFound => None,
            ReplError::MissingArgument(_) => None,
            ReplError::IO(ref e) => Some(e),
        }
    }
}

impl From<io::Error> for ReplError {
    fn from(value: io::Error) -> ReplError {
        ReplError::IO(value)
    }
}
