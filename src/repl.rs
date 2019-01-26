use crate::cargo_cmds::CargoCmds;
use std::io;

#[derive(Clone)]
pub struct Repl {
    pub body: Vec<String>,
    cursor: usize,
    cargo_cmds: CargoCmds,
}

impl Repl {
    pub fn new() -> Self {
        Self {
            body: vec!["fn main() {\n".to_string(), "}".to_string()],
            cursor: 1,
            cargo_cmds: Default::default(),
        }
    }
    pub fn insert(&mut self, mut input: String) {
        input.push('\n');
        self.body.insert(self.cursor, input);
        self.cursor += 1;
    }
    pub fn reset(&mut self) {
        self.prepare_ground().expect("Error while resetting Repl");
        *self = Self::new();
    }
    pub fn show(&self) {
        println!("Current Repl Code:\n{}", self.body.clone().join(""));
    }
    // prepare ground
    pub fn prepare_ground(&self) -> Result<(), io::Error> {
        self.cargo_cmds.cargo_new().unwrap_or_default();
        Ok(())
    }
}
