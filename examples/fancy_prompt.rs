// extern crate colored;
use colored::*;

use std::{error::Error, fmt, process};

use repl_builder::prelude::*;

struct FancyPrompt;

impl fmt::Display for FancyPrompt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", " -> ".black().on_white())
    }
}

fn main() {
    let mut repl = ReplBuilder::default()
        .add_command(Command::new("hi", |_, _| Ok(Some(String::from("Hi!")))))
        .with_prompt(FancyPrompt)
        .build();

    if let Err(e) = repl.run() {
        eprintln!("{}", e);

        if let Some(source) = e.source() {
            eprintln!("Source: {}", source);
        }

        process::exit(1);
    }
}
