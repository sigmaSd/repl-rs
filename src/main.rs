use std::io;
use std::io::Write;

use std::cmp::{max, min};
use tuikit::attr::*;
use tuikit::event::Event;
use tuikit::key::Key;
use tuikit::term::{Term, TermHeight};

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

struct Terminal {
    term: Term,
    repl: Repl,
    buffer: String,
    cursor: (usize, usize),
}

impl Terminal {
    fn new() -> Self {
        Self {
            term: Term::with_height(TermHeight::Percent(30)).unwrap(),
            repl: Repl::new(),
            buffer: String::new(),
            cursor: (1, 0),
        }
    }

    fn run(&mut self) {
        while let Ok(ev) = self.term.poll_event() {
            let _ = self.term.clear();
            let (width, height) = self.term.term_size().unwrap();
            let _ = self.term.print(
                0,
                0,
                &format!(
                    "{0}Welcome to Rust REPL{0}",
                    "-".chars().cycle().take(5).collect::<String>()
                ),
            );
            self.term.present();

            match ev {
                Event::Key(Key::Up) => {
                    let _ = self.term.clear();
                    let _ = self.term.print(0, 0, "hello");
                    self.term.present();
                }
                Event::Key(Key::Down) => (),
                Event::Key(Key::Enter) => {
                    //parse_first_order(&mut repl, self.buffer.clone());
                    self.cursor = (1, 0);
                    let _ = self.term.clear();
                    self.term.set_cursor(self.cursor.0, self.cursor.1);
                    self.buffer.clear();
                    self.term.present();
                }
                Event::Key(Key::Ctrl(_)) => {
                    dbg!("reached");
                    std::process::exit(0)
                }
                _ => {
                    if let Event::Key(Key::Char(letter)) = ev {
                        self.buffer.push(letter);
                        let _ = self.term.clear();
                        self.cursor.1 += 1;
                        self.term.set_cursor(self.cursor.0, self.cursor.1);
                        let _ = self.term.print(1, 0, &self.buffer);

                        self.term.present();
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
