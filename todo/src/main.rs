use std::{env, process};
use todo::Input;

fn main() {
    let input = Input::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Invalid input: {}", err);
        process::exit(1);
    });

    todo::run(input);
}
