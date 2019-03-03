use tuikit::attr::Color;
use tuikit::event::Event;
use tuikit::key::Key;
use tuikit::term::{Term, TermHeight};

use crate::cargo_cmds::CargoCmds;
use crate::enums::{Arrow, Direction};
use crate::history::History;
use crate::repl::Repl;

mod event_handler;
mod parser;
mod wrappers;
mod writer;

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
    screen_cursor: (usize, usize),
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
            screen_cursor: (0, 0),
        };
        terminal.term.show_cursor(true).unwrap();
        terminal
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
                Event::Key(Key::PageUp) => self.scroll_up(),
                Event::Key(Key::PageDown) => self.scroll_down(),
                Event::Key(Key::Backspace) => {
                    self.back_space();
                }
                Event::Key(Key::CtrlLeft) => {
                    // for testing

                    //self.write(&self.terminal_screen.join(""),Color::default());
                    //self.rewrite();
                }
                Event::Key(Key::Ctrl('l')) => {
                    self.custom_clear("");
                }
                Event::Key(Key::Ctrl('c')) => {
                    self.exit();
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
