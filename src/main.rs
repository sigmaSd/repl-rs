use std::io;
use std::io::Write;

use std::iter;

use std::cmp::{max, min};
use tuikit::attr::*;
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
    fn advance_cursor(&mut self) {
        self.cursor.1 += 1;
        self.term.set_cursor(self.cursor.0, self.cursor.1);
    }
    fn handle_letter(&mut self, letter: char) {
        self.buffer.push(letter);
        //self.advance_cursor();
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
/* fn main() {
    let term = Term::with_height(TermHeight::Percent(10)).unwrap();
    let mut repl = Repl::new();

    let mut buffer = String::new();
    let mut cursor = 0;

    while let Ok(ev) = term.poll_event() {
        let _ = term.clear();
        let _ = term.print(0, 0, "press arrow key to move the text, (q) to quit");

        let (width, height) = term.term_size().unwrap();
        match ev {
            Event::Key(Key::Up) => {
                let _ = term.clear();
                let _ = term.print(0, 0, "hello");
                term.present();
            },
            Event::Key(Key::Down) => (),
            Event::Key(Key::Enter) => {
                parse_first_order(&mut repl, buffer.clone());
                buffer.clear();
            },
            _ => match ev {
                Event::Key(Key::Char(letter)) => {
                    buffer.push(letter);
                    let _ = term.clear();
                    cursor +=1;
                    term.set_cursor(0, cursor);
                    let _ = term.print(0, 0, &buffer);

                    term.present();
                },
                _ => ()
            },
        }

    }
} */

/* fn main() {
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
} */

/* fn parse_first_order(repl: &mut Repl, input: String) {
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
} */
