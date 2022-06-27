use parser_combinator::language::lang_parser;
use parser_combinator::language::*;
use parser_combinator::vm::*;
use std::collections::HashMap;

fn main() {
    let _args: Vec<String> = std::env::args().collect();

    let function = lang_parser::function();

    let program_source = "function main {
        let     hello = 5
        let a = (hello * hello)
        let b = (a - 20)
        return   b
    }";

    let expr = function.parse(program_source);
    let (result, remaining) = expr.unwrap();



    let il = vm_emit::emit_function(&result);



    println!("######################################################");
    println!("{:?} -> {:?}", result.body, remaining);
    println!("######################################################");
    println!("{:?}", il);
    println!("######################################################");

    let program = Program::new(HashMap::new());

    let result = program.eval(&il, &[]);

    println!("{:?}", result);
}
