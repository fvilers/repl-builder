use repl_builder::prelude::*;

fn main() {
    let builder =
        ReplBuilder::default().add_command(Command::new("hello", || Ok(Some(String::from("Hi!")))));

    println!("{:?}", builder);
}
