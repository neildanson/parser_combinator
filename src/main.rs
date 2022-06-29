use parser_combinator::language::lang_parser;
use parser_combinator::language::*;
use parser_combinator::vm::*;
use std::collections::HashMap;

fn print_il(il : &[Instruction]) {
    let mut line = 0;
    for instruction in il {
        println!("{} \t: {:?}", line, instruction);
        line = line + 1;
    }
}

fn main() {
    let _args: Vec<String> = std::env::args().collect();

    let function = lang_parser::function();

    let program_source = "function main () {
        let someValue = 50
        let returnValue =  
            if (someValue < 70) {
                let foo = 50
                 (foo + 10)
            } else {
                20
            }
        return returnValue
    }";

    let expr = function.parse(program_source);

    match expr {
        Result::Ok((result, remaining)) => {
            let function = vm_emit::emit_function(&result);

            println!("# AST   ##############################################");
            println!("{:#?} -> {:?}", result.body, remaining);
            println!("# IL    ##############################################");
            print_il(&function.instructions);
            println!("# Result #############################################");
            let program = Program::new(HashMap::new());
            let result = program.eval(&function, &[]);
            println!("{:?}", result);
        }, 
        Result::Err(error) => println!("{}", error)
    }

    
}
