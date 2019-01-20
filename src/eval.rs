use std::fs::File;
use std::io;
use std::io::prelude::*;

use crate::cargo_cmd::cargo_run;
use crate::Repl;

pub fn eval(mut repl: Repl, input: String) -> Result<(), io::Error> {
    let print_statement = format!("println!(\"{{}}\", {})", input);
    repl.insert(print_statement);

    let current_code: String = repl.body.join("");
    let mut main = File::create("./rust_repl_bot/src/main.rs")?;
    write!(main, "{}", current_code)?;
    cargo_run()?;
    Ok(())
}
