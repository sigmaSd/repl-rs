use crate::Repl;

pub fn eval(mut repl: Repl, input: String) -> String {
    let eval_statement = format!("println!(\"{{:?}}\", {});", input);
    repl.insert(eval_statement);

    repl.body.join("")
}
