// Run as cargo run --  --source-file example.pc

use parser_combinator::language::lang_parser;
use parser_combinator::language::*;
use parser_combinator::vm::*;
use std::collections::HashMap;
use std::time::Instant;

use clap::*;

fn print_il(il: &[Instruction]) {
    for (line, instruction) in il.iter().enumerate() {
        println!("{} \t: {:?}", line, instruction);
    }
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long)]
   source_file: String,
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    let program_source = std::fs::read_to_string(args.source_file).map_err(|e| format!("{e}"))?;
    let program_source = program_source.as_str();

    let module = lang_parser::module();

    let parse_start = Instant::now();
    let expr = module.parse(program_source.trim_start());
    let parse_end = Instant::now();
    let parse_time = parse_end - parse_start;
    match expr {
        Result::Ok((module, remaining)) => {
            let emit_start = Instant::now();
            let module = vm_emit::emit_module(module);
            let emit_end = Instant::now();
            let emit_time = emit_end - emit_start;
            /*
            println!(
                "# AST   (Parsed {:?} ##############################################",
                parse_time
            );
            println!("{:#?} -> {:?}", module, remaining);
            println!(
                "# IL    (Emit   {:?} ##############################################",
                emit_time
            );
            print_il(&function.instructions);
            println!("# Result #############################################");*/
            let program = Program::new(module);

            let run_start = Instant::now();
            let result = program.eval(program.main(), &[]);
            let run_end = Instant::now();
            let run_time = run_end - run_start;
            println!("{:?} in {:?}", result, run_time);
        }
        Result::Err(error) => println!("{}", error),
    }
    Ok (())
}
