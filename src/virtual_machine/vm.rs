use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Values {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    /*Array,
    Map,
    Function,
    Null,
    Undefined,*/
}

#[derive(Debug, Clone, PartialEq)]
pub enum Types {
    Int,
    Float,
    Bool,
    String,
    /*Array,
    Map,
    Function,
    Null,
    Undefined,*/
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Nop,
    StoreLocal(String),
    LoadLocal(String),
    LoadArg(usize),
    Push(Values),
    //Math
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    //Comparison
    Equal,
    NotEqual,
    Gt,
    Gte,
    Lt,
    Lte,
    //Logical Operators
    And,
    Or,
    Ret,
    //Control Flow
    JumpEqual(usize),
    JumpNotEqual(usize),
    JumpUnconditional(usize),
    Call(String),
}

struct StackFrame {
    stack: Vec<Values>,
    locals: HashMap<String, Values>,
    return_value: Option<Values>,
}

impl StackFrame {
    fn new() -> StackFrame {
        StackFrame {
            stack: Vec::new(),
            locals: HashMap::new(),
            return_value: None,
        }
    }
}

#[derive(Debug)]
pub struct Function {
    pub parameters: Vec<Types>,
    pub instructions: Vec<Instruction>,
}

impl Function {
    pub fn new(parameters: Vec<Types>, instructions: Vec<Instruction>) -> Function {
        Function {
            parameters,
            instructions,
        }
    }
}

fn add(left: Values, right: Values) -> Values {
    match (left, right) {
        (Values::Int(left), Values::Int(right)) => Values::Int(left + right),
        (Values::Float(left), Values::Float(right)) => Values::Float(left + right),
        (Values::String(left), Values::String(right)) => Values::String(left + &right),
        _ => panic!("Addition not supported for Values"),
    }
}

fn subtract(left: Values, right: Values) -> Values {
    match (left, right) {
        (Values::Int(left), Values::Int(right)) => Values::Int(left - right),
        (Values::Float(left), Values::Float(right)) => Values::Float(left - right),
        _ => panic!("Addition not supported for Values"),
    }
}

fn mul(left: Values, right: Values) -> Values {
    match (left, right) {
        (Values::Int(left), Values::Int(right)) => Values::Int(left * right),
        (Values::Float(left), Values::Float(right)) => Values::Float(left * right),
        _ => panic!("Addition not supported for Values"),
    }
}

fn div(left: Values, right: Values) -> Values {
    match (left, right) {
        (Values::Int(left), Values::Int(right)) => Values::Int(left / right),
        (Values::Float(left), Values::Float(right)) => Values::Float(left / right),
        _ => panic!("Addition not supported for Values"),
    }
}

fn modulus(left: Values, right: Values) -> Values {
    match (left, right) {
        (Values::Int(left), Values::Int(right)) => Values::Int(left % right),
        _ => panic!("Addition not supported for Values"),
    }
}

fn lt(left: Values, right: Values) -> Values {
    match (left, right) {
        (Values::Int(left), Values::Int(right)) => Values::Bool(left < right),
        (Values::Float(left), Values::Float(right)) => Values::Bool(left < right),
        _ => panic!("Addition not supported for Values"),
    }
}

fn lte(left: Values, right: Values) -> Values {
    match (left, right) {
        (Values::Int(left), Values::Int(right)) => Values::Bool(left <= right),
        (Values::Float(left), Values::Float(right)) => Values::Bool(left <= right),
        _ => panic!("Addition not supported for Values"),
    }
}

fn gt(left: Values, right: Values) -> Values {
    match (left, right) {
        (Values::Int(left), Values::Int(right)) => Values::Bool(left > right),
        (Values::Float(left), Values::Float(right)) => Values::Bool(left > right),
        _ => panic!("Addition not supported for Values"),
    }
}

fn gte(left: Values, right: Values) -> Values {
    match (left, right) {
        (Values::Int(left), Values::Int(right)) => Values::Bool(left >= right),
        (Values::Float(left), Values::Float(right)) => Values::Bool(left >= right),
        _ => panic!("Addition not supported for Values"),
    }
}

fn and(left: Values, right: Values) -> Values {
    match (left, right) {
        (Values::Bool(left), Values::Bool(right)) => Values::Bool(left && right),
        _ => panic!("Addition not supported for Values"),
    }
}

fn or(left: Values, right: Values) -> Values {
    match (left, right) {
        (Values::Bool(left), Values::Bool(right)) => Values::Bool(left || right),
        _ => panic!("Addition not supported for Values"),
    }
}

pub struct Program {
    functions: HashMap<&'static str, Function>,
}

impl Program {
    pub fn new(functions: HashMap<&'static str, Function>) -> Program {
        Program { functions }
    }
    pub fn eval(&self, function: &Function, params: &[Values]) -> Option<Values> {
        let mut stack_frame = StackFrame::new();
        let mut ip = 0;
        while ip < function.instructions.len() {
            //println!("Executing \t - {} {:?}", ip, function.instructions[ip]);
            match &function.instructions[ip] {
                Instruction::Nop => {},
                Instruction::Push(value) => {
                    stack_frame.stack.push(value.clone());
                    ip += 1;
                }
                Instruction::StoreLocal(name) => {
                    stack_frame
                        .locals
                        .insert(name.clone(), stack_frame.stack.pop().unwrap());
                    ip += 1;
                }
                Instruction::LoadLocal(name) => {
                    let variable = stack_frame.locals.get(name).unwrap();
                    stack_frame.stack.push(variable.clone());
                    ip += 1;
                }
                Instruction::LoadArg(arg) => {
                    let param = &params[*arg];
                    stack_frame.stack.push(param.clone());
                    ip += 1;
                }
                Instruction::Add => {
                    let right = stack_frame.stack.pop().unwrap();
                    let left = stack_frame.stack.pop().unwrap();
                    stack_frame.stack.push(add(left, right));
                    ip += 1;
                }
                Instruction::Sub => {
                    let right = stack_frame.stack.pop().unwrap();
                    let left = stack_frame.stack.pop().unwrap();
                    stack_frame.stack.push(subtract(left, right));
                    ip += 1;
                }
                Instruction::Mul => {
                    let right = stack_frame.stack.pop().unwrap();
                    let left = stack_frame.stack.pop().unwrap();
                    stack_frame.stack.push(mul(left, right));
                    ip += 1;
                }
                Instruction::Div => {
                    let right = stack_frame.stack.pop().unwrap();
                    let left = stack_frame.stack.pop().unwrap();
                    stack_frame.stack.push(div(left, right));
                    ip += 1;
                }
                Instruction::Mod => {
                    let right = stack_frame.stack.pop().unwrap();
                    let left = stack_frame.stack.pop().unwrap();
                    stack_frame.stack.push(modulus(left, right));
                    ip += 1;
                }
                Instruction::Ret => {
                    let return_value = stack_frame.stack.pop();
                    stack_frame.return_value = return_value;
                    break;
                }
                Instruction::Equal => {
                    let right = stack_frame.stack.pop().unwrap();
                    let left = stack_frame.stack.pop().unwrap();
                    stack_frame.stack.push(Values::Bool(left == right));
                    ip += 1;
                }
                Instruction::NotEqual => {
                    let right = stack_frame.stack.pop().unwrap();
                    let left = stack_frame.stack.pop().unwrap();
                    stack_frame.stack.push(Values::Bool(left != right));
                    ip += 1;
                }
                Instruction::Gt => {
                    let right = stack_frame.stack.pop().unwrap();
                    let left = stack_frame.stack.pop().unwrap();
                    stack_frame.stack.push(gt(left, right));
                    ip += 1;
                }
                Instruction::Gte => {
                    let right = stack_frame.stack.pop().unwrap();
                    let left = stack_frame.stack.pop().unwrap();
                    stack_frame.stack.push(gte(left, right));
                    ip += 1;
                }
                Instruction::Lt => {
                    let right = stack_frame.stack.pop().unwrap();
                    let left = stack_frame.stack.pop().unwrap();
                    stack_frame.stack.push(lt(left, right));
                    ip += 1;
                }
                Instruction::Lte => {
                    let right = stack_frame.stack.pop().unwrap();
                    let left = stack_frame.stack.pop().unwrap();
                    stack_frame.stack.push(lte(left, right));
                    ip += 1;
                }
                Instruction::Call(name) if name == "print" => {
                    let value_to_print = stack_frame.stack.pop().unwrap();
                    println!("{:?}", value_to_print);
                    ip += 1;
                }

                Instruction::Call(function_name) => {
                    let function = self.functions.get(function_name.as_str()).unwrap();
                    let mut parameters = Vec::new();
                    for _ in &function.parameters {
                        let value = stack_frame.stack.pop().unwrap();
                        parameters.push(value);
                    }
                    parameters.reverse();
                    let return_value = self.eval(function, &parameters);
                    stack_frame.stack.push(return_value.unwrap());
                    ip += 1;
                }
                Instruction::And => {
                    let right = stack_frame.stack.pop().unwrap();
                    let left = stack_frame.stack.pop().unwrap();
                    stack_frame.stack.push(and(left, right));
                    ip += 1;
                }
                Instruction::Or => {
                    let right = stack_frame.stack.pop().unwrap();
                    let left = stack_frame.stack.pop().unwrap();
                    stack_frame.stack.push(or(left, right));
                    ip += 1;
                }
                Instruction::JumpEqual(location) => {
                    let right = stack_frame.stack.pop().unwrap();
                    let left = stack_frame.stack.pop().unwrap();
                    if left == right {
                        ip = *location;
                    } else {
                        ip += 1;
                    }
                }
                Instruction::JumpNotEqual(location) => {
                    let right = stack_frame.stack.pop().unwrap();
                    let left = stack_frame.stack.pop().unwrap();
                    if left != right {
                        ip = *location;
                    } else {
                        ip += 1;
                    }
                },
                Instruction::JumpUnconditional(location) => {
                    ip = *location ;
                }
            }
        }
        stack_frame.return_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_2_integers() {
        let instructions = vec![
            Instruction::Push(Values::Int(1)),
            Instruction::Push(Values::Int(2)),
            Instruction::Add,
            Instruction::Ret,
        ];

        let function = Function::new(Vec::new(), instructions);
        let program = Program::new(HashMap::new());

        let result = program.eval(&function, &Vec::new());

        assert_eq!(result, Some(Values::Int(3)));
    }

    #[test]
    fn add_2_floats() {
        let instructions = vec![
            Instruction::Push(Values::Float(1.)),
            Instruction::Push(Values::Float(2.)),
            Instruction::Add,
            Instruction::Ret,
        ];

        let function = Function::new(Vec::new(), instructions);
        let program = Program::new(HashMap::new());

        let result = program.eval(&function, &Vec::new());

        assert_eq!(result, Some(Values::Float(3.)));
    }

    #[test]
    fn add_2_strings() {
        let instructions = vec![
            Instruction::Push(Values::String("hello".to_string())),
            Instruction::Push(Values::String("hello".to_string())),
            Instruction::Add,
            Instruction::Ret,
        ];

        let function = Function::new(Vec::new(), instructions);
        let program = Program::new(HashMap::new());

        let result = program.eval(&function, &Vec::new());
        assert_eq!(result, Some(Values::String("hellohello".to_string())));
    }

    #[test]
    fn mul_2_floats() {
        let instructions = vec![
            Instruction::Push(Values::Float(1.)),
            Instruction::Push(Values::Float(2.)),
            Instruction::Mul,
            Instruction::Ret,
        ];

        let function = Function::new(Vec::new(), instructions);
        let program = Program::new(HashMap::new());
        let result = program.eval(&function, &Vec::new());

        assert_eq!(result, Some(Values::Float(2.)));
    }
    #[test]
    fn div_2_floats() {
        let instructions = vec![
            Instruction::Push(Values::Float(1.)),
            Instruction::Push(Values::Float(2.)),
            Instruction::Div,
            Instruction::Ret,
        ];

        let function = Function::new(Vec::new(), instructions);

        let program = Program::new(HashMap::new());
        let result = program.eval(&function, &Vec::new());

        assert_eq!(result, Some(Values::Float(0.5)));
    }

    #[test]
    fn add_2_floats_with_lookup() {
        let instructions = vec![
            Instruction::Push(Values::Float(1.)),
            Instruction::StoreLocal("One".to_string()),
            Instruction::Push(Values::Float(2.)),
            Instruction::LoadLocal("One".to_string()),
            Instruction::Add,
            Instruction::Ret,
        ];

        let function = Function::new(Vec::new(), instructions);
        let program = Program::new(HashMap::new());

        let result = program.eval(&function, &Vec::new());

        assert_eq!(result, Some(Values::Float(3.)));
    }

    #[test]
    fn add_2_floats_no_return() {
        let instructions = vec![
            Instruction::Push(Values::Float(1.)),
            Instruction::Push(Values::Float(2.)),
            Instruction::Add,
        ];

        let function = Function::new(Vec::new(), instructions);
        let program = Program::new(HashMap::new());

        let result = program.eval(&function, &Vec::new());

        assert_eq!(result, None);
    }

    #[test]
    fn immediate_no_return() {
        let instructions = vec![Instruction::Ret];

        let function = Function::new(Vec::new(), instructions);
        let program = Program::new(HashMap::new());

        let result = program.eval(&function, &Vec::new());

        assert_eq!(result, None);
    }

    #[test]
    fn call_method() {
        let div_instructions = vec![
            Instruction::LoadArg(0),
            Instruction::LoadArg(1),
            Instruction::Div,
            Instruction::Ret,
        ];

        let div = Function::new(vec![Types::Int, Types::Int], div_instructions);

        let call_instructions = vec![
            Instruction::Push(Values::Int(10)),
            Instruction::Push(Values::Int(2)),
            Instruction::Call("div".to_string()),
            Instruction::Ret,
        ];
        let call_div = Function::new(Vec::new(), call_instructions);

        let mut functions = HashMap::new();
        functions.insert("div", div);
        let program = Program::new(functions);

        let result = program.eval(&call_div, &Vec::new());

        assert_eq!(result, Some(Values::Int(5)));
    }
}
