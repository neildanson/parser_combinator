#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Expr {
    Int(i32),
    Str(String),
    Bool(bool),
    Symbol(String),
    Ident(String, Box<Expr>),
    Call(String, Vec<Expr>),
    Return(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Vec<Expr>, Vec<Expr>),
}

pub struct Function {
    pub name: String,
    pub body: Vec<Expr>,
}
