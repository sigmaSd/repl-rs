use std::iter;

use tuikit::event::Event;
use tuikit::key::Key;
use tuikit::term::{Term, TermHeight};

pub mod cargo_cmds;
mod eval;
mod repl;

use crate::cargo_cmds::CargoCmds;
use crate::eval::eval;
use crate::repl::Repl;

enum Arrow {
    Up,
    Down,
}
enum KeyWords {
    Reset,
    Code,
    Show,
    Add,
}

#[derive(Default)]
struct History {
    history: Vec<String>,
    cursor: usize,
}
impl History {
    fn down(&mut self) -> String {
        if self.cursor != self.history.len() - 1 {
            self.cursor += 1;
        }
        self.history[self.cursor].clone()
    }
    fn up(&mut self) -> String {
        if self.cursor != 0 {
            self.cursor -= 1;
        }
        self.history[self.cursor].clone()
    }
    fn push(&mut self, buffer: String) {
        self.history.push(buffer);
        self.cursor += 1;
    }
}
struct Terminal {
    term: Term,
    buffer: String,
    cursor: (usize, usize),
    history: History,
    cargo_cmds: CargoCmds,
}

impl Terminal {
    fn new() -> Self {
        Self {
            term: Term::with_height(TermHeight::Percent(30)).unwrap(),
            buffer: String::new(),
            cursor: (1, 0),
            history: Default::default(),
            cargo_cmds: Default::default(),
        }
    }

    fn get_size(&self) -> (usize, usize) {
        self.term.term_size().unwrap()
    }
    fn write(&self, message: &str) {
        self.clear();
        self.term
            .print(self.cursor.0, self.cursor.1, message)
            .unwrap();
        self.term.present().unwrap();
    }
    fn write_output(&self, message: &str) {
        self.clear();
        self.term
            .print(
                self.cursor.0,
                self.cursor.1,
                &format!("Out[{}]: {}", self.history.cursor, message),
            )
            .unwrap();
        self.term.present().unwrap();
    }
    fn write_input_buffer(&self) {
        self.clear();
        self.term
            .print(
                self.cursor.0,
                self.cursor.1,
                &format!("In [{}]: {}", self.history.cursor, self.buffer),
            )
            .unwrap();
        self.term.present().unwrap();
    }
    fn clear(&self) {
        self.term.clear().unwrap();
        self.term.present().unwrap();
    }

    fn handle_letter(&mut self, letter: char) {
        self.buffer.push(letter);
        self.write_input_buffer();
    }

    // parsing
    fn parse_first_order(&self, repl: &mut Repl) {
        let cmd = match self.buffer.as_str() {
            "reset" => KeyWords::Reset,
            "show" => KeyWords::Show,
            cmd if cmd.starts_with("add") => KeyWords::Add,
            _ => KeyWords::Code,
        };
        match cmd {
            KeyWords::Code => {
                self.parse_second_order(repl);
            }
            KeyWords::Reset => {
                repl.reset();
                self.write("Repl reseted!")
            }
            KeyWords::Show => repl.show(),
            KeyWords::Add => self
                .cargo_cmds
                .cargo_add(&self.buffer)
                .expect("Error while trying to add dependency"),
        }
    }

    fn parse_second_order(&self, repl: &mut Repl) {
        if self.buffer.ends_with(';') {
            repl.insert(self.buffer.clone());
        } else {
            let current_code = eval(repl.clone(), self.buffer.clone());
            let result = self
                .cargo_cmds
                .cargo_run(current_code)
                .expect("error while running playground");

            self.write_output(&result);
        }
    }
    fn clear_buffer_save_history(&mut self) {
        self.history.push(self.buffer.clone());
        self.buffer.clear();
    }

    fn handle_enter_key(&mut self, repl: &mut Repl) {
        self.buffer.trim();
        self.clear();
        self.parse_first_order(repl);
        self.clear_buffer_save_history();
    }

    fn prepare_repl(&self) -> Repl {
        // welcome msg
        let width = self.get_size().0;
        self.write(&format!(
            "{0}Welcome to Rust REPL{0}",
            iter::repeat('-').take(width / 10).collect::<String>()
        ));
        let repl = Repl::new();
        repl.prepare_ground()
            .expect("Error while preparing playground");
        repl
    }

    fn cycle_history(&mut self, to: Arrow) {
        match to {
            Arrow::Up => {
                self.buffer = self.history.up();
                self.write_input_buffer();
            }
            Arrow::Down => {
                self.buffer = self.history.down();
                self.write_input_buffer();
            }
        }
    }

    fn run(&mut self) {
        let mut repl = self.prepare_repl();

        while let Ok(ev) = self.term.poll_event() {
            match ev {
                Event::Key(Key::Up) => self.cycle_history(Arrow::Up),
                Event::Key(Key::Down) => self.cycle_history(Arrow::Down),
                Event::Key(Key::Enter) => {
                    self.handle_enter_key(&mut repl);
                }
                Event::Key(Key::Backspace) => {
                    self.buffer.pop();
                    self.write_input_buffer();
                }
                Event::Key(Key::Ctrl('C')) => std::process::exit(0),
                _ => {
                    if let Event::Key(Key::Char(letter)) = ev {
                        self.handle_letter(letter);
                    } else {
                        // some keys we dont need
                    }
                }
            }
        }
    }
}
fn main() {
    let mut terminal = Terminal::new();
    terminal.run();
}
