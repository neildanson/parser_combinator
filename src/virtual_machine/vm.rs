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

const EMPTY_STACK : &str = "Stack Empty - please check IL";
const ADDITION_NOT_SUPPORTED : &str = "Addition not supported for Values";

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
    fn new(params:HashMap<String, Values>) -> StackFrame {
        StackFrame {
            stack: Vec::new(),
            locals: params,
            return_value: None,
        }
    }
}

#[derive(Debug)]
pub struct Function {
    pub parameters: Vec<String>,
    pub instructions: Vec<Instruction>,
}

impl Function {
    pub fn new(parameters:Vec<String>, instructions: Vec<Instruction>) -> Self {
        Function {
            parameters,
            instructions,
        }
    }
}

#[derive(Debug, Default)]
pub struct Module {
    pub functions : HashMap<String, Function>
}

impl Module {
    pub fn new(functions : HashMap<String, Function>) -> Self {
        Module { functions }
    }
}

fn add(left: Values, right: Values) -> Values {
    match (left, right) {
        (Values::Int(left), Values::Int(right)) => Values::Int(left + right),
        (Values::Float(left), Values::Float(right)) => Values::Float(left + right),
        (Values::String(left), Values::String(right)) => Values::String(left + &right),
        _ => panic!("{ADDITION_NOT_SUPPORTED}"),
    }
}

fn subtract(left: Values, right: Values) -> Values {
    match (left, right) {
        (Values::Int(left), Values::Int(right)) => Values::Int(left - right),
        (Values::Float(left), Values::Float(right)) => Values::Float(left - right),
        _ => panic!("{ADDITION_NOT_SUPPORTED}"),
    }
}

fn mul(left: Values, right: Values) -> Values {
    match (left, right) {
        (Values::Int(left), Values::Int(right)) => Values::Int(left * right),
        (Values::Float(left), Values::Float(right)) => Values::Float(left * right),
        _ => panic!("{ADDITION_NOT_SUPPORTED}"),
    }
}

fn div(left: Values, right: Values) -> Values {
    match (left, right) {
        (Values::Int(left), Values::Int(right)) => Values::Int(left / right),
        (Values::Float(left), Values::Float(right)) => Values::Float(left / right),
        _ => panic!("{ADDITION_NOT_SUPPORTED}"),
    }
}

fn modulus(left: Values, right: Values) -> Values {
    match (left, right) {
        (Values::Int(left), Values::Int(right)) => Values::Int(left % right),
        _ => panic!("{ADDITION_NOT_SUPPORTED}"),
    }
}

fn lt(left: Values, right: Values) -> Values {
    match (left, right) {
        (Values::Int(left), Values::Int(right)) => Values::Bool(left < right),
        (Values::Float(left), Values::Float(right)) => Values::Bool(left < right),
        _ => panic!("{ADDITION_NOT_SUPPORTED}"),
    }
}

fn lte(left: Values, right: Values) -> Values {
    match (left, right) {
        (Values::Int(left), Values::Int(right)) => Values::Bool(left <= right),
        (Values::Float(left), Values::Float(right)) => Values::Bool(left <= right),
        _ => panic!("{ADDITION_NOT_SUPPORTED}"),
    }
}

fn gt(left: Values, right: Values) -> Values {
    match (left, right) {
        (Values::Int(left), Values::Int(right)) => Values::Bool(left > right),
        (Values::Float(left), Values::Float(right)) => Values::Bool(left > right),
        _ => panic!("{ADDITION_NOT_SUPPORTED}"),
    }
}

fn gte(left: Values, right: Values) -> Values {
    match (left, right) {
        (Values::Int(left), Values::Int(right)) => Values::Bool(left >= right),
        (Values::Float(left), Values::Float(right)) => Values::Bool(left >= right),
        _ => panic!("{ADDITION_NOT_SUPPORTED}"),
    }
}

fn and(left: Values, right: Values) -> Values {
    match (left, right) {
        (Values::Bool(left), Values::Bool(right)) => Values::Bool(left && right),
        _ => panic!("{ADDITION_NOT_SUPPORTED}"),
    }
}

fn or(left: Values, right: Values) -> Values {
    match (left, right) {
        (Values::Bool(left), Values::Bool(right)) => Values::Bool(left || right),
        _ => panic!("{ADDITION_NOT_SUPPORTED}"),
    }
}

pub struct Program {
    functions: HashMap<String, Function>,
}

impl Program {
    pub fn new(module: Module) -> Program {
        Program { functions : module.functions }
    }

    pub fn main(&self) -> &Function {
        self.functions.get("main").expect("Missing `main` entry point")
    }

    pub fn eval(&self, function: &Function, params: HashMap<String,Values>) -> Option<Values> {
        let mut stack_frame = StackFrame::new(params);
        
        let mut ip = 0;
        while ip < function.instructions.len() {
            //println!("Executing \t - {} {:?}", ip, function.instructions[ip]);
            match &function.instructions[ip] {
                Instruction::Nop => {}
                Instruction::Push(value) => {
                    stack_frame.stack.push(value.clone());
                    ip += 1;
                }
                Instruction::StoreLocal(name) => {
                    stack_frame
                        .locals
                        .insert(name.clone(), stack_frame.stack.pop().expect(EMPTY_STACK));
                    ip += 1;
                }
                Instruction::LoadLocal(name) => {
                    let variable = stack_frame.locals.get(name).unwrap_or_else(|| panic!("Missing {name}"));
                    stack_frame.stack.push(variable.clone());
                    ip += 1;
                }
                Instruction::Add => {
                    let right = stack_frame.stack.pop().expect(EMPTY_STACK);
                    let left = stack_frame.stack.pop().expect(EMPTY_STACK);
                    stack_frame.stack.push(add(left, right));
                    ip += 1;
                }
                Instruction::Sub => {
                    let right = stack_frame.stack.pop().expect(EMPTY_STACK);
                    let left = stack_frame.stack.pop().expect(EMPTY_STACK);
                    stack_frame.stack.push(subtract(left, right));
                    ip += 1;
                }
                Instruction::Mul => {
                    let right = stack_frame.stack.pop().expect(EMPTY_STACK);
                    let left = stack_frame.stack.pop().expect(EMPTY_STACK);
                    stack_frame.stack.push(mul(left, right));
                    ip += 1;
                }
                Instruction::Div => {
                    let right = stack_frame.stack.pop().expect(EMPTY_STACK);
                    let left = stack_frame.stack.pop().expect(EMPTY_STACK);
                    stack_frame.stack.push(div(left, right));
                    ip += 1;
                }
                Instruction::Mod => {
                    let right = stack_frame.stack.pop().expect(EMPTY_STACK);
                    let left = stack_frame.stack.pop().expect(EMPTY_STACK);
                    stack_frame.stack.push(modulus(left, right));
                    ip += 1;
                }
                Instruction::Ret => {
                    let return_value = stack_frame.stack.pop();
                    stack_frame.return_value = return_value;
                    break;
                }
                Instruction::Equal => {
                    let right = stack_frame.stack.pop().expect(EMPTY_STACK);
                    let left = stack_frame.stack.pop().expect(EMPTY_STACK);
                    stack_frame.stack.push(Values::Bool(left == right));
                    ip += 1;
                }
                Instruction::NotEqual => {
                    let right = stack_frame.stack.pop().expect(EMPTY_STACK);
                    let left = stack_frame.stack.pop().expect(EMPTY_STACK);
                    stack_frame.stack.push(Values::Bool(left != right));
                    ip += 1;
                }
                Instruction::Gt => {
                    let right = stack_frame.stack.pop().expect(EMPTY_STACK);
                    let left = stack_frame.stack.pop().expect(EMPTY_STACK);
                    stack_frame.stack.push(gt(left, right));
                    ip += 1;
                }
                Instruction::Gte => {
                    let right = stack_frame.stack.pop().expect(EMPTY_STACK);
                    let left = stack_frame.stack.pop().expect(EMPTY_STACK);
                    stack_frame.stack.push(gte(left, right));
                    ip += 1;
                }
                Instruction::Lt => {
                    let right = stack_frame.stack.pop().expect(EMPTY_STACK);
                    let left = stack_frame.stack.pop().expect(EMPTY_STACK);
                    stack_frame.stack.push(lt(left, right));
                    ip += 1;
                }
                Instruction::Lte => {
                    let right = stack_frame.stack.pop().expect(EMPTY_STACK);
                    let left = stack_frame.stack.pop().expect(EMPTY_STACK);
                    stack_frame.stack.push(lte(left, right));
                    ip += 1;
                }
                Instruction::Call(name) if name == "print" => {
                    let value_to_print = stack_frame.stack.pop().expect(EMPTY_STACK);
                    println!("{:?}", value_to_print);
                    ip += 1;
                }

                Instruction::Call(function_name) => {
                    let function = self.functions.get(function_name.as_str()).unwrap();
                    let mut parameters_values = Vec::new();
                    for _ in 0.. function.parameters.len() {
                        let value = stack_frame.stack.pop().expect(EMPTY_STACK);
                        parameters_values.push(value);
                    }
                    parameters_values.reverse();
                    let mut parameters = HashMap::new();
                    for (p, pvalue) in parameters_values.iter().enumerate()  {
                        parameters.insert(function.parameters[p].clone(), pvalue.clone());
                    }

                    let return_value = self.eval(function, parameters);

                    if let Some(return_value) = return_value {
                        stack_frame.stack.push(return_value);
                    }
                    ip += 1;
                }
                Instruction::And => {
                    let right = stack_frame.stack.pop().expect(EMPTY_STACK);
                    let left = stack_frame.stack.pop().expect(EMPTY_STACK);
                    stack_frame.stack.push(and(left, right));
                    ip += 1;
                }
                Instruction::Or => {
                    let right = stack_frame.stack.pop().expect(EMPTY_STACK);
                    let left = stack_frame.stack.pop().expect(EMPTY_STACK);
                    stack_frame.stack.push(or(left, right));
                    ip += 1;
                }
                Instruction::JumpEqual(location) => {
                    let right = stack_frame.stack.pop().expect(EMPTY_STACK);
                    let left = stack_frame.stack.pop().expect(EMPTY_STACK);
                    if left == right {
                        ip = *location;
                    } else {
                        ip += 1;
                    }
                }
                Instruction::JumpNotEqual(location) => {
                    let right = stack_frame.stack.pop().expect(EMPTY_STACK);
                    let left = stack_frame.stack.pop().expect(EMPTY_STACK);
                    if left != right {
                        ip = *location;
                    } else {
                        ip += 1;
                    }
                }
                Instruction::JumpUnconditional(location) => {
                    ip = *location;
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

        let function = Function::new(Vec::default(), instructions);
        let program = Program::new(Module::default());

        let result = program.eval(&function, HashMap::default());

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

        let function = Function::new(Vec::default(), instructions);
        let program = Program::new(Module::default());

        let result = program.eval(&function, HashMap::default());

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

        let function = Function::new(Vec::default(), instructions);
        let program = Program::new(Module::default());

        let result = program.eval(&function, HashMap::default());
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

        let function = Function::new(Vec::default(), instructions);
        let program = Program::new(Module::default());
        let result = program.eval(&function,HashMap::default());

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

        let function = Function::new(Vec::default(), instructions);

        let program = Program::new(Module::default());
        let result = program.eval(&function, HashMap::default());

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

        let function = Function::new(Vec::default(), instructions);
        let program = Program::new(Module::default());

        let result = program.eval(&function, HashMap::default());

        assert_eq!(result, Some(Values::Float(3.)));
    }

    #[test]
    fn add_2_floats_no_return() {
        let instructions = vec![
            Instruction::Push(Values::Float(1.)),
            Instruction::Push(Values::Float(2.)),
            Instruction::Add,
        ];

        let function = Function::new(Vec::default(), instructions);
        let program = Program::new(Module::default());

        let result = program.eval(&function, HashMap::default());

        assert_eq!(result, None);
    }

    #[test]
    fn immediate_no_return() {
        let instructions = vec![Instruction::Ret];

        let function = Function::new(Vec::default(), instructions);
        let program = Program::new(Module::default());

        let result = program.eval(&function, HashMap::default());

        assert_eq!(result, None);
    }

    #[test]
    fn call_method() {
        let div_instructions = vec![
            Instruction::LoadLocal("param1".to_string()),
            Instruction::LoadLocal("param2".to_string()),
            Instruction::Div,
            Instruction::Ret,
        ];

        let div = Function::new(vec!("param1".to_string(), "param2".to_string()), div_instructions);

        let call_instructions = vec![
            Instruction::Push(Values::Int(10)),
            Instruction::Push(Values::Int(2)),
            Instruction::Call("div".to_string()),
            Instruction::Ret,
        ];
        let call_div = Function::new(Vec::default(), call_instructions);

        let mut functions = HashMap::new();
        functions.insert("div".to_string(), div);
        let module = Module::new(functions);
        let program = Program::new(module);

        let result = program.eval(&call_div, HashMap::default());

        assert_eq!(result, Some(Values::Int(5)));
    }
}
