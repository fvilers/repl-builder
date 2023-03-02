use std::{error::Error, process};

use repl_builder::prelude::*;

fn main() {
    let repl = ReplBuilder::default()
        .add_command(Command::new("hello", |_| Ok(Some(String::from("Hi!")))))
        .build();

    if let Err(e) = repl.run() {
        eprintln!("{}", e);

        if let Some(source) = e.source() {
            eprintln!("Source: {}", source);
        }

        process::exit(1);
    }
}
