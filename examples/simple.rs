use std::{error::Error, process};

use repl_builder::prelude::*;

fn main() {
    let mut repl = ReplBuilder::default()
        .add_command(Command::new("hi", |_, _| Ok(Some(String::from("Hi!")))))
        .add_command(Command::new("hello", hello))
        .build();

    if let Err(e) = repl.run() {
        eprintln!("{}", e);

        if let Some(source) = e.source() {
            eprintln!("Source: {}", source);
        }

        process::exit(1);
    }
}

fn hello(args: Vec<&str>, _context: &mut ()) -> CommandResult {
    let Some(name) = args.first() else {
        return Err(ReplError::MissingArgument("name".into()));
    };

    Ok(Some(format!("Hello, {}!", name)))
}
