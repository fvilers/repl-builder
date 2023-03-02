use repl_builder::prelude::*;

fn main() {
    let repl = ReplBuilder::default()
        .add_command(Command::new("hello", || Ok(Some(String::from("Hi!")))))
        .build();

    println!("{:?}", repl);
}
