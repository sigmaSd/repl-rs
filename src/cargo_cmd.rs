use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use std::process::Command;

pub fn cargo_new() -> Result<(), io::Error> {
    let _ = fs::remove_dir_all("/tmp/rust_repl_playground");
    Command::new("cargo")
        .current_dir("/tmp")
        .args(&["new", "rust_repl_playground"])
        .spawn()?
        .wait()?;
    Ok(())
}

pub fn cargo_run(code: String) -> Result<(), io::Error> {
    let mut main = File::create("/tmp/rust_repl_playground/src/main.rs")?;
    write!(main, "{}", code)?;
    let out = Command::new("cargo")
        .current_dir("/tmp/rust_repl_playground")
        .arg("run")
        .output()?;

    let stdout = String::from_utf8(out.stdout).expect("Invalid input (Not Utf-8)");
    let stderr = String::from_utf8(out.stderr).expect("Invalid input (Not Utf-8)");
    if stdout.is_empty() {
        print!("{}", stderr);
    } else {
        print!("{}", stdout);
    }
    Ok(())
}

pub fn cargo_add(add_dep: &str) -> Result<(), io::Error> {
    let add_dep: Vec<&str> = add_dep.split(' ').collect();
    if add_dep.len() < 2 {
        println!("missing dependency for cargo add cmd");
        return Ok(());
    }
    Command::new("cargo")
        .current_dir("/tmp/rust_repl_playground")
        .args(&add_dep)
        .spawn()?
        .wait()?;
    Ok(())
}
