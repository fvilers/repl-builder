use std::fmt;

#[derive(Default)]
pub struct Prompt;

impl fmt::Display for Prompt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ">")
    }
}
