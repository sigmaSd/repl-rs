use std::iter;

use tuikit::event::Event;
use tuikit::key::Key;
use tuikit::term::{Term, TermHeight};

pub mod cargo_cmd;
mod eval;
mod repl;

use crate::cargo_cmd::cargo_add;
use crate::eval::eval;
use crate::repl::Repl;

enum KeyWords {
    Reset,
    Code,
    Show,
    Add,
}

struct Terminal {
    term: Term,
    buffer: String,
    cursor: (usize, usize),
}

impl Terminal {
    fn new() -> Self {
        Self {
            term: Term::with_height(TermHeight::Percent(30)).unwrap(),
            buffer: String::new(),
            cursor: (1, 0),
        }
    }

    fn get_size(&self) -> (usize, usize) {
        self.term.term_size().unwrap()
    }

    fn write(&self, message: &str) {
        self.clear();
        self.term.print(self.cursor.0, self.cursor.1, message);
        self.term.present();
    }
    fn clear(&self) {
        self.term.clear();
        self.term.present();
    }

    fn handle_letter(&mut self, letter: char) {
        self.buffer.push(letter);
        self.write(&self.buffer.clone());
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
            KeyWords::Add => cargo_add(&self.buffer).expect("Error while trying to add dependency"),
        }
    }

    fn parse_second_order(&self, repl: &mut Repl) {
        if self.buffer.ends_with(';') {
            repl.insert(self.buffer.clone());
        } else {
            let result =
                eval(repl.clone(), self.buffer.clone()).expect("Error while evaluating expression");
            self.write(&result);
        }
    }

    fn handle_enter_key(&mut self, repl: &mut Repl) {
        self.buffer.trim();
        self.clear();
        self.parse_first_order(repl);
        self.buffer.clear();
    }

    fn prepare_repl(&self) -> Repl {
        // welcome msg
        let width = self.get_size().0;
        self.write(&format!(
            "{0}Welcome to Rust REPL{0}",
            iter::repeat('-').take(width / 10).collect::<String>()
        ));
        Repl::prepare_ground();
        Repl::new()
    }

    fn run(&mut self) {
        let mut repl = self.prepare_repl();
        while let Ok(ev) = self.term.poll_event() {
            match ev {
                Event::Key(Key::Up) => {}
                Event::Key(Key::Down) => (),
                Event::Key(Key::Enter) => {
                    self.handle_enter_key(&mut repl);
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
