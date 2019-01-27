use tuikit::event::Event;
use tuikit::key::Key;
use tuikit::term::{Term, TermHeight};

use crate::cargo_cmds::CargoCmds;
use crate::eval::eval;
use crate::history::History;
use crate::repl::Repl;

use std::iter;

enum Kind {
    Statement,
    Expression(String),
    Cmd,
}
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

pub struct Terminal {
    term: Term,
    buffer: String,
    cursor: (usize, usize),
    history: History,
    cargo_cmds: CargoCmds,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            term: Term::with_height(TermHeight::Percent(100)).unwrap(),
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
        self.term
            .print(self.cursor.0, self.cursor.1, message)
            .unwrap();
        self.term.present().unwrap();
    }
    fn writeln(&mut self, message: &str) {
        self.cursor.0 += 1;
        self.term
            .print(self.cursor.0, self.cursor.1, message)
            .unwrap();
        self.term.present().unwrap();
    }
    fn write_output(&mut self, exp: String) {
        self.history.push(exp.clone());
        self.writeln(&format!("Out[{}]:{}", self.history.last_idx() - 1, exp));
        self.buffer.clear();
        self.writeln("");
        self.write_input();
    }

    fn write_input(&self) {
        self.write(&format!("In[{}]:{}", self.history.last_idx(), self.buffer));
    }
    fn clear(&self) {
        self.term.clear().unwrap();
        self.term.present().unwrap();
    }

    fn handle_letter(&mut self, letter: char) {
        self.buffer.push(letter);
        self.write_input();
    }
    fn reset(&mut self, repl: &mut Repl) {
        repl.reset();
        self.clear();
        self.history.reset();
        self.cursor = (1, 0);
    }

    // parsing
    fn parse_first_order(&mut self, mut repl: &mut Repl) -> Kind {
        let cmd = match self.buffer.as_str() {
            "reset" => KeyWords::Reset,
            "show" => KeyWords::Show,
            cmd if cmd.starts_with("add") => KeyWords::Add,
            _ => KeyWords::Code,
        };
        match cmd {
            KeyWords::Code => self.parse_second_order(repl),
            KeyWords::Reset => {
                self.reset(&mut repl);
                self.writeln("Repl reseted!");
                self.writeln("");
                self.buffer.clear();
                self.write_input();
                Kind::Cmd
            }
            KeyWords::Show => {
                self.writeln(&repl.show());
                self.writeln("");
                self.buffer.clear();
                self.write_input();
                Kind::Cmd
            }
            KeyWords::Add => {
                self.cargo_cmds
                    .cargo_add(&self.buffer)
                    .expect("Error while trying to add dependency");
                self.writeln("");
                self.buffer.clear();
                self.write_input();
                Kind::Cmd
            }
        }
    }

    fn parse_second_order(&self, repl: &mut Repl) -> Kind {
        if self.buffer.ends_with(';') {
            repl.insert(self.buffer.clone());
            Kind::Statement
        } else {
            let current_code = eval(repl.clone(), self.buffer.clone());
            let result = self
                .cargo_cmds
                .cargo_run(current_code)
                .expect("error while running playground");

            Kind::Expression(result)
        }
    }

    fn handle_enter_key(&mut self, repl: &mut Repl) {
        self.buffer.trim();

        let kind = self.parse_first_order(repl);
        match kind {
            Kind::Statement => {
                self.history.push(self.buffer.clone());
            }
            Kind::Expression(exp) => {
                self.history.push(self.buffer.clone());
                self.write_output(exp);
            }
            _ => {}
        }
    }

    fn prepare_repl(&mut self) -> Repl {
        // welcome msg
        let width = self.get_size().0;

        self.clear();
        self.writeln(&format!(
            "{0}Welcome to Rust REPL{0}",
            iter::repeat('-').take(width / 10).collect::<String>()
        ));
        self.writeln("");
        let repl = Repl::new();
        repl.prepare_ground()
            .expect("Error while preparing playground");
        repl
    }

    fn cycle_history(&mut self, to: Arrow) {
        match to {
            Arrow::Up => {
                self.buffer = self.history.up();
                self.empty_input_line();
                self.write_input();
            }
            Arrow::Down => {
                self.buffer = self.history.down();
                self.empty_input_line();
                self.write_input();
            }
        }
    }
    fn empty_input_line(&self) {
        self.write(
            &iter::repeat(" ")
                // magic number
                .take(500)
                .collect::<String>(),
        );
    }
    pub fn run(&mut self) {
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
                    self.empty_input_line();
                    self.write_input();
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
