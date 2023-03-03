use std::{error::Error, process};

use repl_builder::prelude::*;

#[derive(Default)]
struct State {
    count: usize,
}

fn main() {
    let mut repl = ReplBuilder::new(State::default())
        .add_command(Command::new("inc", inc))
        .add_command(Command::new("dec", dec))
        .add_command(Command::new("count", |_, context| {
            Ok(Some(context.count.to_string()))
        }))
        .build();

    if let Err(e) = repl.run() {
        eprintln!("{}", e);

        if let Some(source) = e.source() {
            eprintln!("Source: {}", source);
        }

        process::exit(1);
    }
}

fn inc(_args: Vec<&str>, context: &mut State) -> CommandResult {
    if context.count.checked_add(1).is_none() {
        return Err(ReplError::Execution("Overflow while incrementing".into()));
    }

    Ok(None)
}

fn dec(_args: Vec<&str>, context: &mut State) -> CommandResult {
    if context.count.checked_sub(1).is_none() {
        return Err(ReplError::Execution("Overflow while decrementing".into()));
    }

    Ok(None)
}
