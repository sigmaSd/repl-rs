use std::fs::{remove_file, File};
use std::io::prelude::*;
use std::process::Command;
use std::str;

pub fn eval(form: &str) -> String {
    let mut rust_temp = File::create("/tmp/rust_temp").unwrap();
    write!(
        rust_temp,
        "fn main() {{
    	println!(\"{{}}\",{});
        }}",
        form
    )
    .unwrap();

    if let Ok(()) = remove_file("/tmp/rust_temp_bin") {};

    Command::new("rustc")
        .args(&["/tmp/rust_temp", "-o", "/tmp/rust_temp_bin"])
        .output()
        .expect("error while compiling tmp file");

    str::from_utf8(
        &Command::new("/tmp/rust_temp_bin")
            .output()
            .expect("No file found")
            .stdout,
    )
    .unwrap()
    .to_string()
}
