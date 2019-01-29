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
    terminal_screen: Vec<(String, Color)>,
    left_margin: usize,
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
            terminal_screen: Vec::new(),
            left_margin: 8,
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
        self.print_blinking_cursor();
        self.term
            .print_with_attr(self.cursor.0, self.cursor.1, message, attr)
            .unwrap();
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
                self.writeln(&format!(" Out[{}]: {}", self.history.last_idx() - 1, chunk));
            }
        });
        self.scroll_down();
        self.empty_new_line(1);
        self.buffer.clear();
        self.write_input();
    }

    fn write_input(&mut self) {
        self.write(
            &format!(" In[{}]: {}", self.history.last_idx(), self.buffer),
            Color::YELLOW,
        );
    }
    fn rewrite(&mut self) {
        self.terminal_screen.remove(0);
        self.clear();
        self.reset_cursors();
        self.empty_new_line(1);
        for (val, color) in self.terminal_screen.clone() {
            let space = if color == Color::LIGHT_RED { 2 } else { 1 };
            let attr = Attr {
                fg: color,
                ..Attr::default()
            };
            self.term
                .print_with_attr(self.cursor.0, self.cursor.1, &val, attr)
                .unwrap();
            self.cursor.0 += space;
        }
        if self.terminal_screen.last().unwrap().1 == Color::YELLOW {
            self.cursor.0 -= 1;
        }
        self.print_blinking_cursor();
        self.term.present().unwrap();
    }
    fn scroll_down(&mut self) {
        if self.cursor.0 as f32 >= 3.0 / 4.0 * self.get_size().1 as f32 {
            self.rewrite();
        }
    }
    fn back_space(&mut self) {
        self.move_blinking_cursor_auto(Direction::Left);
        if !self.buffer.is_empty() {
            self.buffer.remove(self.blinking_cursor_actual_pos());
        }
        self.empty_input_line();
        self.write_input();
    }

    // cursor + blinking cursor
    fn blinking_cursor_actual_pos(&self) -> usize {
        self.blinking_cursor.1 - self.left_margin
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
        if self.blinking_cursor.1 > self.left_margin + self.buffer.len() {
            self.blinking_cursor.1 = self.left_margin + self.buffer.len();
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
        self.blinking_cursor = (0, self.left_margin);
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
        if self.history.last_idx() != 0 && is_it_pow(self.history.last_idx() + 1, 10) {
            self.left_margin += 1;
        }
        self.buffer.trim();
        self.terminal_screen.push((
            format!(" In[{}]: {}", self.history.last_idx(), self.buffer),
            Color::YELLOW,
        ));
        self.scroll_down();

        let kind = self.parse_first_order(repl);
        match kind {
            Kind::Statement => {
                self.history.push(self.buffer.clone());
                self.empty_new_line(1);
                self.buffer.clear();
                self.reset_blinking_cursor_col();
                self.write_input();
            }
            Kind::Expression(out) => {
                self.history.push(self.buffer.clone());
                self.terminal_screen.push((
                    format!(" Out[{}]: {}", self.history.last_idx() - 1, out),
                    Color::LIGHT_RED,
                ));
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
        self.left_margin = 8;
        self.clear();
        self.history.reset();
        self.reset_cursors();
        self.terminal_screen.clear();
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
                    self.back_space();
                }
                Event::Key(Key::CtrlLeft) => {
                    // for testing

                    //self.write(&self.terminal_screen.join(""),Color::default());
                    //self.rewrite();
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

fn is_it_pow(input: usize, mut candidate: usize) -> bool {
    let original_candidiate = candidate;
    if input < candidate {
        return false;
    }
    loop {
        if input == candidate {
            return true;
        }
        candidate *= original_candidiate;
        if input < candidate {
            return false;
        }
    }
}
