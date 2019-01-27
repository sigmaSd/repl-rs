use crate::repl::Repl;

pub fn eval(mut repl: Repl, input: String) -> String {
    let eval_statement = format!("println!(\"{{:?}}\", {{\n{}\n}});", input);
    repl.insert(eval_statement);

    repl.body.join("")
}
