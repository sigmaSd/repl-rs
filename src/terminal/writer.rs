// write methods
use crate::enums::Direction;
use crate::terminal::Terminal;
use tuikit::attr::{Attr, Color};

impl Terminal {
    pub fn write(&mut self, message: &str, color: Color) {
        let attr = Attr {
            fg: color,
            ..Attr::default()
        };
        self.correct_blinking_cursor_row();
        self.term
            .print_with_attr(self.cursor.0, self.cursor.1, message, attr)
            .unwrap();
        self.term.present().unwrap();
    }

    pub fn writeln(&mut self, message: &str) {
        self.cursor.0 += 1;
        self.write(message, Color::LIGHT_RED);
    }
    pub fn write_output(&mut self, out: String) {
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

    pub fn write_input(&mut self) {
        self.write(
            &format!(" In[{}]: {}", self.history.last_idx(), self.buffer),
            Color::YELLOW,
        );
    }
    pub fn write_screen(&mut self) {
        self.clear();
        self.reset_cursors();
        self.empty_new_line(1);
        for (val, color) in self.terminal_screen[self.screen_cursor.0..].to_owned() {
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
        self.correct_blinking_cursor_row();
        self.term.present().unwrap();
        self.write_input();
    }
    pub fn back_space(&mut self) {
        self.move_blinking_cursor_auto(Direction::Left);
        if !self.buffer.is_empty() {
            self.buffer.remove(self.blinking_cursor_col_pos());
        }
        self.empty_input_line();
        self.write_input();
    }

    // cursor + blinking cursor
    pub fn blinking_cursor_col_pos(&self) -> usize {
        self.blinking_cursor.1 - self.left_margin
    }
    pub fn move_blinking_cursor_manuel(&mut self, direction: Direction) {
        self.move_blinking_cursor_auto(direction);
        self.correct_blinking_cursor_row();
        self.term.present().unwrap();
    }
    pub fn move_blinking_cursor_auto(&mut self, direction: Direction) {
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
    pub fn correct_blinking_cursor_row(&mut self) {
        self.blinking_cursor.0 = self.cursor.0;
        self.term
            .set_cursor(self.blinking_cursor.0, self.blinking_cursor.1)
            .unwrap();
    }
    pub fn reset_blinking_cursor_col(&mut self) {
        self.blinking_cursor.1 = self.buffer.len() + self.left_margin;
    }
    pub fn reset_cursors(&mut self) {
        self.cursor = (0, 0);
        //self.blinking_cursor = (0, self.left_margin);
    }
}
