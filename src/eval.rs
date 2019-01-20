use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

use crate::Repl;

pub fn prepare_ground() -> Result<(), io::Error> {
    if !Path::new("./rust_repl_bot").is_dir() {
        cargo_new()?;
    }

    Ok(())
}
pub fn eval(mut repl: Repl, input: String) -> Result<(), io::Error> {
    let print_statement = format!("println!(\"{{}}\", {})", input);
    repl.insert(print_statement);

    let current_code: String = repl.body.join("");
    let mut main = File::create("./rust_repl_bot/src/main.rs")?;
    write!(main, "{}", current_code)?;
    cargo_run()?;
    Ok(())
}

fn cargo_new() -> Result<(), io::Error> {
    Command::new("cargo")
        .args(&["new", "rust_repl_bot"])
        .spawn()?
        .wait()?;
    Ok(())
}

fn cargo_run() -> Result<(), io::Error> {
    let out = Command::new("cargo")
        .current_dir("./rust_repl_bot")
        .arg("run")
        .output()?
        .stdout;
    let out = String::from_utf8(out).expect("Invalid input (Not Utf-8)");
    println!("{}", out);
    Ok(())
}
