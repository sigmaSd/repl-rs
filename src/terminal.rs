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
    fn write_output(&mut self, exp: String) {
        self.history.push(exp);
        self.write_history();
        self.term.present().unwrap();
        self.buffer.clear();
        self.write_input();
    }
    fn write_history(&self) {
        self.clear();

        for (idx, (in_out_idx, line)) in self
            .history
            .buffer_vec
            .iter()
            .zip(&*self.history.line_idx)
            .enumerate()
        {
            let prefix = if idx % 2 == 0 { "In[" } else { "Out[" };

            self.term
                .print(idx, 0, &format!("{0}{2}]: {1}", prefix, in_out_idx, line))
                .unwrap();
        }
    }
    fn write_input(&self) {
        self.clear();
        self.write_history();
        self.term
            .print(
                self.history.buffer_vec.len(),
                0,
                &format!("In[{}]: {}", self.history.last_idx(), self.buffer),
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
        self.write_input();
    }

    // parsing
    fn parse_first_order(&self, repl: &mut Repl) -> Kind {
        let cmd = match self.buffer.as_str() {
            "reset" => KeyWords::Reset,
            "show" => KeyWords::Show,
            cmd if cmd.starts_with("add") => KeyWords::Add,
            _ => KeyWords::Code,
        };
        match cmd {
            KeyWords::Code => self.parse_second_order(repl),
            KeyWords::Reset => {
                repl.reset();
                self.write("Repl reseted!");
                Kind::Cmd
            }
            KeyWords::Show => {
                repl.show();
                Kind::Cmd
            }
            KeyWords::Add => {
                self.cargo_cmds
                    .cargo_add(&self.buffer)
                    .expect("Error while trying to add dependency");
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
        self.history.push(self.buffer.clone());

        let kind = self.parse_first_order(repl);
        match kind {
            Kind::Statement => {}
            Kind::Expression(exp) => {
                self.write_output(exp);
            }
            _ => unreachable!(),
        }
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
                self.write_input();
            }
            Arrow::Down => {
                self.buffer = self.history.down();
                self.write_input();
            }
        }
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
