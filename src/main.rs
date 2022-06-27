use parser_combinator::language::lang_parser;
use parser_combinator::language::*;
use parser_combinator::vm::*;
use std::collections::HashMap;

fn main() {
    let _args: Vec<String> = std::env::args().collect();

    let body = lang_parser::body();

    let program_source = "let    hello = 5
        let a = (hello * hello)
        let b = (a - 20)
        return b";
    let expr = body.parse(program_source);
    let (result, remaining) = expr.unwrap();
    let il = vm_emit::emit_body(&result);

    println!("######################################################");
    println!("{:?} -> {:?}", result, remaining);
    println!("######################################################");
    println!("{:?}", il);
    println!("######################################################");

    let func = Function::new(vec![], il);
    let program = Program::new(HashMap::new());

    let result = program.eval(&func, &[]);

    println!("{:?}", result);
}
