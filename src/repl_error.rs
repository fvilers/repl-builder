use std::{error, fmt, io};

#[derive(Debug)]
pub enum ReplError {
    CommandNotFound,
    MissingArgument(String),
    Custom(String),
    IO(io::Error),
}

impl fmt::Display for ReplError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReplError::CommandNotFound => write!(f, "Command not found"),
            ReplError::MissingArgument(name) => write!(f, "Missing argument `{}`", name),
            ReplError::Custom(msg) => write!(f, "{}", msg),
            ReplError::IO(e) => e.fmt(f),
        }
    }
}

impl error::Error for ReplError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            ReplError::CommandNotFound => None,
            ReplError::MissingArgument(_) => None,
            ReplError::Custom(_) => None,
            ReplError::IO(ref e) => Some(e),
        }
    }
}

impl From<io::Error> for ReplError {
    fn from(value: io::Error) -> ReplError {
        ReplError::IO(value)
    }
}
