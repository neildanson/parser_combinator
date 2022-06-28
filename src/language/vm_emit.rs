use crate::ast::Expr;
use crate::virtual_machine::*;

fn append(instructions: &mut Vec<Instruction>, instructions_to_add: &[Instruction]) {
    let base_offset = instructions.len();

    for instruction in instructions_to_add {
        let new_instruction = match instruction {
            Instruction::JumpEqual(offset) => Instruction::JumpEqual(base_offset + offset),
            Instruction::JumpNotEqual(offset) => Instruction::JumpNotEqual(base_offset + offset),
            _ => instruction.clone(),
        };
        instructions.push(new_instruction);
    }
}

fn emit(expr: &Expr) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    match expr {
        Expr::Int(i) => instructions.push(Instruction::Push(Values::Int(*i))),
        Expr::Str(s) => instructions.push(Instruction::Push(Values::String(s.to_string()))),
        Expr::Bool(b) => instructions.push(Instruction::Push(Values::Bool(*b))),
        Expr::Symbol(s) => instructions.push(Instruction::LoadLocal(s.clone())),
        Expr::Ident(s, expr) => {
            let exprs = emit(expr);
            append(&mut instructions, &exprs);
            instructions.push(Instruction::StoreLocal(s.clone()));
        }
        Expr::Return(expr) => {
            let exprs = emit(expr);
            append(&mut instructions, &exprs);
            instructions.push(Instruction::Ret);
        }
        Expr::Call(s, _e) => println!("{}", s),
        Expr::Add(lhs, rhs) => {
            let exprs = emit(lhs);
            append(&mut instructions, &exprs);
            let exprs = emit(rhs);
            append(&mut instructions, &exprs);
            instructions.push(Instruction::Add);
        }
        Expr::Subtract(lhs, rhs) => {
            let exprs = emit(lhs);
            append(&mut instructions, &exprs);
            let exprs = emit(rhs);
            append(&mut instructions, &exprs);
            instructions.push(Instruction::Sub);
        }
        Expr::Multiply(lhs, rhs) => {
            let exprs = emit(lhs);
            append(&mut instructions, &exprs);
            let exprs = emit(rhs);
            append(&mut instructions, &exprs);
            instructions.push(Instruction::Mul);
        }
        Expr::Divide(lhs, rhs) => {
            let exprs = emit(lhs);
            append(&mut instructions, &exprs);
            let exprs = emit(rhs);
            append(&mut instructions, &exprs);
            instructions.push(Instruction::Div);
        }
        Expr::If(cond, body, else_) => {
            let body = emit_body(body);
            let else_ = emit_body(else_);

            instructions.push(Instruction::Push(Values::Bool(true)));
            let cond = emit(&cond);
            append(&mut instructions, &cond);
            let else_start = cond.len() + body.len() + 3;//Not sure why 3? 1 for push bool, 1 for UnconditionalJump. 1 for ?
            instructions.push(Instruction::JumpNotEqual(else_start));
            append(&mut instructions, &body);
            instructions.push(Instruction::JumpUnconditional(else_start + else_.len()));
            append(&mut instructions, &else_);
        }
        Expr::Equals(lhs, rhs) => {
            let lhs = emit(lhs);
            let rhs = emit(rhs);

            append(&mut instructions, &lhs);
            append(&mut instructions, &rhs);

            instructions.push(Instruction::Equal);
        },
        Expr::LessThan(lhs,rhs) => {},
        Expr::GreaterThan(lhs,rhs) => {}
    }
    instructions
}

fn emit_body(exprs: &[Expr]) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for e in exprs {
        let instructions_to_add = emit(e);
        append(&mut instructions, &instructions_to_add);
    }
    instructions
}

pub fn emit_function(function: &crate::ast::Function) -> Function {
    let body = emit_body(&function.body);
    Function::new(Vec::new(), body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emit_assign() {
        let expr = Expr::Ident("x".to_string(), Box::new(Expr::Int(1)));
        let result = emit(&expr);

        let expected = vec![
            Instruction::Push(Values::Int(1)),
            Instruction::StoreLocal("x".to_string()),
        ];

        assert_eq!(result, expected);
    }
}
