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

    let program_source = "
    function main () {
        counter = 1
        
        while (counter < 100) {
            result = 
                if (((counter % 3) == 0) && ((counter % 5) == 0)) {
                    \"FizzBuzz\"
                } else {
                    if ((counter % 3) == 0) {
                        \"Fizz\"
                    } else {
                        if ((counter % 3) == 0) {
                            \"Buzz\"
                        } else {
                            counter
                        }
                    }
                }
            print(result)
            counter = (counter + 1)
        }
    }";

    let expr = function.parse(program_source.trim_start());

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
