use std::io;
use std::io::Write;

pub mod cargo_cmd;
mod eval;

use crate::cargo_cmd::{cargo_add, cargo_new};
use crate::eval::eval;

enum KeyWords {
    Reset,
    Code,
    Show,
    Add,
}

#[derive(Clone)]
pub struct Repl {
    body: Vec<String>,
    cursor: usize,
}

impl Repl {
    fn new() -> Self {
        Self {
            body: vec!["fn main() {\n".to_string(), "}".to_string()],
            cursor: 1,
        }
    }
    fn insert(&mut self, mut input: String) {
        input.push('\n');
        self.body.insert(self.cursor, input);
        self.cursor += 1;
    }
    fn reset(&mut self) {
        prepare_ground().expect("Error while resetting Repl");
        *self = Self::new();
    }
    fn show(&self) {
        println!("Current Repl Code:\n{}", self.body.clone().join(""));
    }
}

fn main() {
    prepare_ground().expect("Error while preparing repl");

    let mut repl = Repl::new();

    loop {
        let mut input = String::new();
        print!(">>>");
        io::stdout().flush().expect("Error while flushing stdout");
        io::stdin()
            .read_line(&mut input)
            .expect("Error while reding stdin");
        parse_first_order(&mut repl, input);
    }
}

fn parse_first_order(repl: &mut Repl, input: String) {
    // avoid all kind of bugs by trim()
    let input = input.trim();
    let cmd = match input {
        "reset" => KeyWords::Reset,
        "show" => KeyWords::Show,
        cmd if cmd.starts_with("add") => KeyWords::Add,
        _ => KeyWords::Code,
    };
    match cmd {
        KeyWords::Code => {
            parse_second_order(repl, input);
        }
        KeyWords::Reset => {
            repl.reset();
            println!("Repl reseted!")
        }
        KeyWords::Show => repl.show(),
        KeyWords::Add => cargo_add(input).expect("Error while trying to add dependency"),
    }
}

fn parse_second_order(repl: &mut Repl, input: &str) {
    let input = input.to_string();
    if input.ends_with(';') {
        repl.insert(input);
    } else {
        eval(repl.clone(), input).expect("Error while evaluating expression");
    }
}

// prepare ground
fn prepare_ground() -> Result<(), io::Error> {
    cargo_new().unwrap_or_default();
    Ok(())
}
