use std::io;
use std::process::Command;

pub fn cargo_new() -> Result<(), io::Error> {
    Command::new("cargo")
        .args(&["new", "rust_repl_bot"])
        .spawn()?
        .wait()?;
    Ok(())
}

pub fn cargo_run() -> Result<(), io::Error> {
    let out = Command::new("cargo")
        .current_dir("./rust_repl_bot")
        .arg("run")
        .output()?
        .stdout;
    let out = String::from_utf8(out).expect("Invalid input (Not Utf-8)");
    print!(" {}", out);
    Ok(())
}

pub fn cargo_add(add_dep: &str) -> Result<(), io::Error> {
    let add_dep: Vec<&str> = add_dep.split(' ').collect();
    if add_dep.len() < 2 {
        println!("missing dependency for cargo add cmd");
        return Ok(());
    }
    Command::new("cargo")
        .current_dir("./rust_repl_bot")
        .args(&add_dep)
        .spawn()?
        .wait()?;
    Ok(())
}
