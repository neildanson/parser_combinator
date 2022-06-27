use crate::ast::Expr;
use crate::virtual_machine::*;

pub fn emit(expr: &Expr) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    match expr {
        Expr::Int(i) => instructions.push(Instruction::Push(Values::Int(*i))),
        Expr::Str(s) => instructions.push(Instruction::Push(Values::String(s.to_string()))),
        Expr::Bool(b) => instructions.push(Instruction::Push(Values::Bool(*b))),
        Expr::Symbol(s) => instructions.push(Instruction::LoadLocal(s.clone())),
        Expr::Ident(s, expr) => {
            let mut exprs = emit(expr);
            instructions.append(&mut exprs);
            instructions.push(Instruction::StoreLocal(s.clone()));
        }
        Expr::Return(expr) => {
            let mut exprs = emit(expr);
            instructions.append(&mut exprs);
            instructions.push(Instruction::Ret);
        }
        Expr::Call(s, _e) => println!("{}", s),
        Expr::Add(lhs, rhs) => {
            let mut exprs = emit(lhs);
            instructions.append(&mut exprs);
            let mut exprs = emit(rhs);
            instructions.append(&mut exprs);
            instructions.push(Instruction::Add);
        }
        Expr::Subtract(lhs, rhs) => {
            let mut exprs = emit(lhs);
            instructions.append(&mut exprs);
            let mut exprs = emit(rhs);
            instructions.append(&mut exprs);
            instructions.push(Instruction::Sub);
        }
        Expr::Multiply(lhs, rhs) => {
            let mut exprs = emit(lhs);
            instructions.append(&mut exprs);
            let mut exprs = emit(rhs);
            instructions.append(&mut exprs);
            instructions.push(Instruction::Mul);
        }
        Expr::Divide(lhs, rhs) => {
            let mut exprs = emit(lhs);
            instructions.append(&mut exprs);
            let mut exprs = emit(rhs);
            instructions.append(&mut exprs);
            instructions.push(Instruction::Div);
        }
    }
    instructions
}

pub fn emit_body(exprs: &[Expr]) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for e in exprs {
        let mut exprs = emit(e);
        instructions.append(&mut exprs);
    }
    instructions
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
