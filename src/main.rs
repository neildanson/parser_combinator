use parser_combinator::language::lang_parser;
use parser_combinator::language::*;
use parser_combinator::vm::*;
use std::collections::HashMap;

fn main() {
    let _args: Vec<String> = std::env::args().collect();

    let function = lang_parser::function();

    let program_source = "function main () {
        let someValue = true
        let returnValue =  
            if (someValue == true) {
                let foo = 50
                foo
            } else {
                20
            }
        return returnValue
    }";

    let expr = function.parse(program_source);

    match expr {
        Result::Ok((result, remaining)) => {
            let il = vm_emit::emit_function(&result);

            println!("######################################################");
            println!("{:?} -> {:?}", result.body, remaining);
            println!("######################################################");
            println!("{:?}", il);
            println!("######################################################");

            let program = Program::new(HashMap::new());

            let result = program.eval(&il, &[]);

            println!("{:?}", result);
        }, 
        Result::Err(error) => println!("{}", error)
    }

    
}
