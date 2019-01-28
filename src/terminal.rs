use tuikit::attr::{Attr, Color};
use tuikit::event::Event;
use tuikit::key::Key;
use tuikit::term::{Term, TermHeight};

use crate::cargo_cmds::CargoCmds;
use crate::enums::{Arrow, Direction, KeyWords, Kind};
use crate::eval::eval;
use crate::history::History;
use crate::repl::Repl;

use std::iter;

pub struct Terminal {
    term: Term,
    buffer: String,
    cursor: (usize, usize),
    history: History,
    cargo_cmds: CargoCmds,
    blinking_cursor: (usize, usize),
    left_margin: usize,
    terminal_screen: Vec<(String, Color)>,
    record: bool,
}

impl Terminal {
    pub fn new() -> Self {
        let terminal = Self {
            term: Term::with_height(TermHeight::Percent(100)).unwrap(),
            buffer: String::new(),
            cursor: (0, 0),
            blinking_cursor: (0, 8),
            history: Default::default(),
            cargo_cmds: Default::default(),
            left_margin: 8,
            terminal_screen: Vec::new(),
            record: true
        };
        terminal.term.show_cursor(true).unwrap();
        terminal
    }

    // write methods
    fn write(&mut self, message: &str, color: Color) {
        let attr = Attr {
            fg: color,
            ..Attr::default()
        };
        if self.record {
            self.terminal_screen.push((message.to_string(), color));
        }
        self.print_blinking_cursor();
        self.term
            .print_with_attr(self.cursor.0, self.cursor.1, message, attr)
            .unwrap();
        self.term.present().unwrap();
        self.record = true;
    }
    fn rewrite(&mut self) {
        // for val in self.terminal_screen.clone() {
        //     self.writeln(&val);
        // }
        //self.writeln("------------------5555555555555555555-------------------");
        self.terminal_screen = slide(&self.terminal_screen);
        self.clear();
        self.cursor = (0, 1);
        for (val, color) in self.terminal_screen.clone() {
            let attr = Attr {
                fg: color,
                ..Attr::default()
            };
            self.term
                .print_with_attr(self.cursor.0, self.cursor.1, &val, attr)
                .unwrap();
            self.cursor.0 += 1;
        }
        self.term.present().unwrap();
    }
    fn writeln(&mut self, message: &str) {
        self.cursor.0 += 1;
        self.write(message, Color::LIGHT_RED);
    }
    fn write_output(&mut self, out: String) {
        out.split('\n').enumerate().for_each(|(idx, chunk)| {
            if idx != 0 {
                self.writeln(&format!("            {}", chunk));
            } else {
                self.writeln(&format!("Out[{}]: {}", self.history.last_idx() - 1, chunk));
            }
        });
        self.empty_new_line(1);
        self.buffer.clear();
        self.write_input();
    }

    fn write_input(&mut self) {
        self.write(
            &format!("In[{}]: {}", self.history.last_idx(), self.buffer),
            Color::YELLOW,
        );
    }

    // cursor + blinking cursor
    fn blinking_cursor_actual_pos(&self) -> usize {
        if self.blinking_cursor.1 >= self.left_margin {
            self.blinking_cursor.1 - self.left_margin
        } else {
            0
        }
    }
    fn move_blinking_cursor_manuel(&mut self, direction: Direction) {
        self.move_blinking_cursor_auto(direction);
        self.print_blinking_cursor();
        self.term.present().unwrap();
    }
    fn move_blinking_cursor_auto(&mut self, direction: Direction) {
        match direction {
            Direction::Right => self.blinking_cursor.1 += 1,
            Direction::Left => self.blinking_cursor.1 -= 1,
        }
        if self.blinking_cursor.1 < self.left_margin {
            self.blinking_cursor.1 = self.left_margin;
        }
    }
    fn print_blinking_cursor(&mut self) {
        self.blinking_cursor.0 = self.cursor.0;
        self.term
            .set_cursor(self.blinking_cursor.0, self.blinking_cursor.1)
            .unwrap();
    }
    fn reset_blinking_cursor_col(&mut self) {
        self.blinking_cursor.1 = self.buffer.len() + self.left_margin;
    }
    fn reset_cursors(&mut self) {
        self.cursor = (0, 0);
        self.left_margin = 7;
        self.blinking_cursor = (0, 7);
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
                self.reset(&mut repl, "Repl reseted!");
                Kind::Cmd
            }
            KeyWords::Show => {
                self.writeln(&repl.show());
                self.empty_new_line(1);
                self.buffer.clear();
                self.reset_blinking_cursor_col();
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
    // events handling
    fn handle_enter_key(&mut self, repl: &mut Repl) {
        self.buffer.trim();
        let kind = self.parse_first_order(repl);
        match kind {
            Kind::Statement => {
                self.history.push(self.buffer.clone());
                self.empty_new_line(2);
                self.buffer.clear();
                self.reset_blinking_cursor_col();
                self.write_input();
            }
            Kind::Expression(out) => {
                self.history.push(self.buffer.clone());
                self.write_output(out);
                self.reset_blinking_cursor_col();
                self.write_input();
            }
            _ => {}
        }
    }
    fn handle_character(&mut self, letter: char) {
        self.buffer
            .insert(self.blinking_cursor_actual_pos(), letter);
        self.move_blinking_cursor_auto(Direction::Right);
        self.record = false;
        self.write_input();
    }
    fn cycle_history(&mut self, to: Arrow) {
        match to {
            Arrow::Up => {
                self.buffer = self.history.up();
                self.empty_input_line();
                self.reset_blinking_cursor_col();
                self.write_input();
            }
            Arrow::Down => {
                self.buffer = self.history.down();
                self.empty_input_line();
                self.reset_blinking_cursor_col();
                self.write_input();
            }
        }
    }

    // wrapers

    fn clear(&self) {
        self.term.clear().unwrap();
        self.term.present().unwrap();
    }

    fn custom_clear(&mut self, msg: &str) {
        self.clear();
        self.history.reset();
        self.reset_cursors();
        if !msg.is_empty() {
            self.writeln(msg);
        }
        self.empty_new_line(1);
        self.buffer.clear();
        self.write_input();
    }
    fn reset(&mut self, repl: &mut Repl, msg: &str) {
        repl.reset();
        self.custom_clear(msg);
    }
    fn empty_new_line(&mut self, n: usize) {
        if n == 0 {
            self.write("", Color::Default);
        } else {
            for _ in 0..=n {
                self.writeln("");
            }
        }
    }
    fn empty_input_line(&mut self) {
        self.write(
            &iter::repeat(" ")
                // magic number
                .take(500)
                .collect::<String>(),
            Color::LIGHT_BLUE,
        );
    }
    fn get_size(&self) -> (usize, usize) {
        self.term.term_size().unwrap()
    }

    // prepare repl
    fn prepare_repl(&mut self) -> Repl {
        // welcome msg
        let repl = Repl::new();
        repl.prepare_ground()
            .expect("Error while preparing playground");

        let width = self.get_size().0;

        self.clear();
        self.cursor.1 += 1;
        self.write(
            &format!(
                "{0}Welcome to Rust REPL{0}",
                iter::repeat('-').take(width / 3).collect::<String>()
            ),
            Color::BLUE,
        );
        self.empty_new_line(2);
        self.write_input();

        repl
    }

    // run
    pub fn run(&mut self) {
        let mut repl = self.prepare_repl();

        while let Ok(ev) = self.term.poll_event() {
            match ev {
                Event::Key(Key::Up) => self.cycle_history(Arrow::Up),
                Event::Key(Key::Down) => self.cycle_history(Arrow::Down),
                Event::Key(Key::Right) => self.move_blinking_cursor_manuel(Direction::Right),
                Event::Key(Key::Left) => self.move_blinking_cursor_manuel(Direction::Left),
                Event::Key(Key::Enter) => {
                    self.handle_enter_key(&mut repl);
                }
                Event::Key(Key::Backspace) => {
                    self.move_blinking_cursor_auto(Direction::Left);
                    self.buffer.remove(self.blinking_cursor_actual_pos());
                    self.empty_input_line();
                    self.write_input();
                }
                Event::Key(Key::CtrlLeft) => {
                    //self.write(&self.terminal_screen.join(""),Color::default());
                    self.rewrite();
                }
                Event::Key(Key::Ctrl('L')) => {
                    self.custom_clear("");
                }
                Event::Key(Key::Ctrl('C')) => {
                    self.clear();
                    self.reset_cursors();
                    self.empty_new_line(0);
                    std::process::exit(0)
                }
                _ => {
                    if let Event::Key(Key::Char(letter)) = ev {
                        self.handle_character(letter);
                    } else {
                        // some keys we dont need?
                    }
                }
            }
        }
    }
}
fn slide(v: &Vec<(String, Color)>) -> Vec<(String, Color)> {
    let mut new_v = Vec::new();
    for (val,c) in v.iter().skip(1) {
        new_v.push((val.clone(), c.clone()));
    }
    new_v.push(v.last().unwrap().clone());
    new_v
}
