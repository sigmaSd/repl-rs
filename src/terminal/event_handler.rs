// events handling
use crate::enums::{Arrow, Direction, Kind};
use crate::helper_fns::is_it_pow;
use crate::repl::Repl;
use crate::terminal::Terminal;
use tuikit::attr::Color;

impl Terminal {
    pub fn handle_enter_key(&mut self, repl: &mut Repl) {
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
        self.history.go_to_last();
    }
    pub fn handle_character(&mut self, letter: char) {
        self.scroll_to_end();
        self.buffer.insert(self.blinking_cursor_col_pos(), letter);
        self.move_blinking_cursor_auto(Direction::Right);
        self.write_input();
    }
    fn scroll_to_end(&mut self) {
        if self.screen_cursor.0 != self.screen_cursor.1 {
            self.screen_cursor.0 = self.screen_cursor.1;
            self.write_screen();
        }
    }
    pub fn scroll_up(&mut self) {
        if self.screen_cursor.0 > 0 {
            self.screen_cursor.0 -= 1;
            self.write_screen();
        }
    }
    pub fn scroll_down(&mut self) {
        if self.cursor.0 as f32 >= 3.0 / 4.0 * self.get_size().1 as f32 {
            self.screen_cursor.0 += 1;
            self.screen_cursor.1 += 1;
            self.write_screen();
        }
    }
    pub fn cycle_history(&mut self, to: Arrow) {
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
    pub fn exit(&mut self) {
        self.clear();
        self.reset_cursors();
        self.empty_new_line(0);
        std::process::exit(0)
    }
}
