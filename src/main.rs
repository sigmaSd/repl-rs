//let shell = vec!["fn main() {","}"]

use std::io;
mod eval;
use crate::eval::{eval, prepare_ground};

enum KeyWords {
    Reset,
    Code,
    Show,
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