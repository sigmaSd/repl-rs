use crate::cargo_cmd::cargo_new;
use std::io;

#[derive(Clone)]
pub struct Repl {
    pub body: Vec<String>,
    cursor: usize,
}

impl Repl {
    pub fn new() -> Self {
        Self {
            body: vec!["fn main() {\n".to_string(), "}".to_string()],
            cursor: 1,
        }
    }
    pub fn insert(&mut self, mut input: String) {
        input.push('\n');
        self.body.insert(self.cursor, input);
        self.cursor += 1;
    }
    pub fn reset(&mut self) {
        Repl::prepare_ground().expect("Error while resetting Repl");
        *self = Self::new();
    }
    pub fn show(&self) {
        println!("Current Repl Code:\n{}", self.body.clone().join(""));
    }
    // prepare ground
    pub fn prepare_ground() -> Result<(), io::Error> {
        cargo_new().unwrap_or_default();
        Ok(())
    }
}
