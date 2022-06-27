extern crate parser_combinator;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;

use parser_combinator::*;

fn parse_success(c: &mut Criterion) {
    let any_number = any_of(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);
    let number_parser = pchar('-').optional().then(any_number.many1());

    let to_number = number_parser.map(move |(negate, value): (Option<char>, Vec<char>)| {
        let string: String = value.into_iter().collect();
        let number = string.parse::<i32>().unwrap();
        match negate {
            Some(_) => -number,
            None => number,
        }
    });

    c.bench_function("Parse Success", |b| {
        b.iter(|| {
            for _ in 0..100 {
                let _ = black_box(to_number.parse("-123456789"));
            }
        })
    });
}

fn parse_fail(c: &mut Criterion) {
    let any_number = any_of(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);
    let number_parser = pchar('-').optional().then(any_number.many1());

    let to_number = number_parser.map(move |(negate, value): (Option<char>, Vec<char>)| {
        let string: String = value.into_iter().collect();
        let number = string.parse::<i32>().unwrap();
        match negate {
            Some(_) => -number,
            None => number,
        }
    });

    c.bench_function("Parse Fail", |b| {
        b.iter(|| {
            for _ in 0..100 {
                let _ = black_box(to_number.parse("-12345678B"));
            }
        })
    });
}

fn vm_addition(c: &mut Criterion) {
    let mut instructions = vec![Instruction::Push(Values::Int(0))];
    for i in 0..1000 {
        instructions.push(Instruction::Push(Values::Int(i)));
        instructions.push(Instruction::Add);
    }

    let function = Function::new(Vec::new(), instructions);
    let program = Program::new(HashMap::new());

    c.bench_function("VM Addition", |b| {
        b.iter(|| {
            black_box(program.eval(&function, &Vec::new()));
        })
    });
}

fn vm_loop(c: &mut Criterion) {
    let mut instructions = vec![];
    instructions.push(Instruction::Push(Values::Int(0)));
    instructions.push(Instruction::StoreLocal("Local".to_string())); // <- Load 0 into local
    instructions.push(Instruction::LoadLocal("Local".to_string()));
    instructions.push(Instruction::Push(Values::Int(1)));
    instructions.push(Instruction::Add); // <- Add 1 to local
    instructions.push(Instruction::StoreLocal("Local".to_string())); // <- Store local back into local (Store pops the stack)
    instructions.push(Instruction::LoadLocal("Local".to_string()));
    instructions.push(Instruction::Push(Values::Int(1000))); // <--Load 1000
    instructions.push(Instruction::JumpNotEqual(2)); // <-- Jump if local != 1000
    instructions.push(Instruction::LoadLocal("Local".to_string()));
    instructions.push(Instruction::Ret);
    let function = Function::new(Vec::new(), instructions);
    let program = Program::new(HashMap::new());

    c.bench_function("VM Loop", |b| {
        b.iter(|| {
            black_box(program.eval(&function, &Vec::new()));
        })
    });
}

criterion_group!(benches, parse_success, parse_fail, vm_addition, vm_loop);
criterion_main!(benches);
