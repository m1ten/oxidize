pub mod args;
pub mod file;
pub mod setup;
pub mod vars;

use std::io::Write;

// Other useful functions

// get input function
pub fn readln() -> String {
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => input.replace("\n", ""),
        Err(e) => panic!("{}", ansi_term::Color::Red.paint(e.to_string())),
    }
}