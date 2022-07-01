use parser_combinator::language::lang_parser;
use parser_combinator::language::*;
use parser_combinator::vm::*;
use std::collections::HashMap;
use std::time::Instant;

fn print_il(il: &[Instruction]) {
    for (line, instruction) in il.iter().enumerate() {
        println!("{} \t: {:?}", line, instruction);
    }
}

fn main() {
    let _args: Vec<String> = std::env::args().collect();

    let function = lang_parser::function();

    let program_source = "
    function main () {
        counter = 1
        
        while (counter < 1000) {
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

    let parse_start = Instant::now();
    let expr = function.parse(program_source.trim_start());
    let parse_end = Instant::now();
    let parse_time = parse_end - parse_start;
    match expr {
        Result::Ok((result, remaining)) => {
            let emit_start = Instant::now();
            let function = vm_emit::emit_function(&result);
            let emit_end = Instant::now();
            let emit_time = emit_end - emit_start;

            println!(
                "# AST   (Parsed {:?} ##############################################",
                parse_time
            );
            println!("{:#?} -> {:?}", result.body, remaining);
            println!(
                "# IL    (Emit   {:?} ##############################################",
                emit_time
            );
            print_il(&function.instructions);
            println!("# Result #############################################");
            let program = Program::new(HashMap::new());
            let run_start = Instant::now();
            let result = program.eval(&function, &[]);
            let run_end = Instant::now();
            let run_time = run_end - run_start;
            println!("{:?} in {:?}", result, run_time);
        }
        Result::Err(error) => println!("{}", error),
    }
}
