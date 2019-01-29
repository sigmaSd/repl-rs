mod cargo_cmds;
mod enums;
mod eval;
mod helper_fns;
mod history;
mod repl;
mod terminal;

use std::env;
use terminal::Terminal;

fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| "".to_string());
    match arg.as_str() {
        "" => {}
        "-h" | "--help" => {
            println!(" Rust repl!");
            std::process::exit(0);
        }
        "-v" | "--version" => {
            println!(" Rust Repl version 0.1.0");
            std::process::exit(0);
        }
        _ => {}
    }
    let mut terminal = Terminal::new();
    terminal.run();
}
