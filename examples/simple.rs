use std::process;

use repl_builder::prelude::*;

fn main() {
    let repl = ReplBuilder::default()
        .add_command(Command::new("hello", || Ok(Some(String::from("Hi!")))))
        .build();

    if let Err(e) = repl.run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}
