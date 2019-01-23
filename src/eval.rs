use crate::cargo_cmd::cargo_run;
use crate::Repl;
use std::io;

pub fn eval(mut repl: Repl, input: String) -> Result<String, io::Error> {
    let eval_statement = format!("println!(\"{{:?}}\", {});", input);
    repl.insert(eval_statement);

    let current_code: String = repl.body.join("");

    cargo_run(current_code)
}
