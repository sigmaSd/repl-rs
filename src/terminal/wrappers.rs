// wrapers
use crate::repl::Repl;
use crate::terminal::Terminal;
use std::iter;
use tuikit::attr::Color;

impl Terminal {
    pub fn clear(&self) {
        self.term.clear().unwrap();
        self.term.present().unwrap();
    }

    pub fn custom_clear(&mut self, msg: &str) {
        self.left_margin = 8;
        self.clear();
        self.history.reset();
        self.reset_cursors();
        self.blinking_cursor.1 = self.left_margin;
        self.terminal_screen.clear();
        self.screen_cursor = (0, 0);
        if !msg.is_empty() {
            self.writeln(msg);
        }
        self.empty_new_line(1);
        self.buffer.clear();
        self.write_input();
    }
    pub fn reset(&mut self, repl: &mut Repl, msg: &str) {
        repl.reset();
        self.custom_clear(msg);
    }
    pub fn empty_new_line(&mut self, n: usize) {
        if n == 0 {
            self.write("", Color::Default);
        } else {
            for _ in 0..=n {
                self.writeln("");
            }
        }
    }
    pub fn empty_input_line(&mut self) {
        self.write(
            &iter::repeat(" ")
                // magic number
                .take(500)
                .collect::<String>(),
            Color::LIGHT_BLUE,
        );
    }
    pub fn get_size(&self) -> (usize, usize) {
        self.term.term_size().unwrap()
    }
}
