mod cargo_cmds;
mod eval;
mod history;
mod repl;
mod terminal;

use terminal::Terminal;

fn main() {
    let mut terminal = Terminal::new();
    terminal.run();
}
