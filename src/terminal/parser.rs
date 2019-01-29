// parsing
use crate::enums::{KeyWords, Kind};
use crate::eval::eval;
use crate::repl::Repl;
use crate::terminal::Terminal;

impl Terminal {
    pub fn parse_first_order(&mut self, mut repl: &mut Repl) -> Kind {
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

    pub fn parse_second_order(&self, repl: &mut Repl) -> Kind {
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
}
