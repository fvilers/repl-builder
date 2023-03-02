use std::{error, fmt};

#[derive(Debug)]
pub enum ReplError {
    Run,
}

impl fmt::Display for ReplError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReplError::Run => write!(f, "Error while running the REPL"),
        }
    }
}

impl error::Error for ReplError {}
