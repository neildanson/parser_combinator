use crate::ast::*;
use crate::parser_combinator::parser::*;
use std::rc::Rc;

fn int<'a>() -> RcParser<'a, Expr> {
    let any_number = any_of(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);
    let number_parser = pchar('-').optional().then(any_number.many1());

    number_parser
        .map(move |(negate, value): (Option<char>, Vec<char>)| {
            let string: String = value.into_iter().collect();
            let number = string.parse::<i32>().unwrap();
            match negate {
                Some(_) => -number,
                None => number,
            }
        })
        .map(Expr::Int)
}

fn string_ident<'a>() -> RcParser<'a, String> {
    let mut allowed_chars = Vec::new();
    for c in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
        allowed_chars.push(c);
    }
    let chars = any_of(&allowed_chars).many1();

    chars
        .map(move |value: Vec<char>| value.into_iter().collect())
        .ws()
}

fn string_symbol<'a>() -> RcParser<'a, Expr> {
    let mut allowed_chars = Vec::new();
    for c in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
        allowed_chars.push(c);
    }
    let chars = any_of(&allowed_chars).many1();

    chars
        .map(move |value: Vec<char>| value.into_iter().collect())
        .map(Expr::Symbol)
}

fn bool<'a>() -> RcParser<'a, Expr> {
    let true_ = pstring("true");
    let false_ = pstring("false");
    true_.or(false_).map(|s| Expr::Bool(s == "true"))
}

fn math(symbol: char, expr: Rc<ForwardParser<Expr>>) -> RcParser<(Expr, Expr)> {
    let lparen = pchar('(').ws();
    let rparen = pchar(')').ws();
    let plus = pchar(symbol).ws();

    lparen
        .right(expr.clone())
        .left(plus)
        .then(expr.clone())
        .left(rparen)
}

fn add(expr: Rc<ForwardParser<Expr>>) -> RcParser<Expr> {
    math('+', expr).map(|(lhs, rhs)| Expr::Add(Box::new(lhs), Box::new(rhs)))
}

fn subtract(expr: Rc<ForwardParser<Expr>>) -> RcParser<Expr> {
    math('-', expr).map(|(lhs, rhs)| Expr::Subtract(Box::new(lhs), Box::new(rhs)))
}

fn multiply(expr: Rc<ForwardParser<Expr>>) -> RcParser<Expr> {
    math('*', expr).map(|(lhs, rhs)| Expr::Multiply(Box::new(lhs), Box::new(rhs)))
}

fn divide(expr: Rc<ForwardParser<Expr>>) -> RcParser<Expr> {
    math('/', expr).map(|(lhs, rhs)| Expr::Divide(Box::new(lhs), Box::new(rhs)))
}


fn condition<'a>(expr: RcParser<'a, Expr>) -> RcParser<'a, Expr> {
    let if_ = pstring("if").ws();
    let cond = expr.clone().between(pchar('(').ws(), pchar(')')).ws();
    let body = expr.clone();

    if_.right(cond).then(body).map(|(cond, body) | Expr::If(Box::new(cond), vec![body]))
}

pub fn expr<'a>() -> RcParser<'a, Expr> {
    let int_ = int();
    let symbol = string_symbol(); //Make quoted
    let quoted_string = string_ident()
        .between(pchar('\"'), pchar('\"'))
        .map(Expr::Str);
    let bool_ = bool();

    let forward_ref: ForwardParser<'a, Expr> = forward();
    let mut forward = Rc::new(forward_ref);
    let add = add(forward.clone());
    let subtract = subtract(forward.clone());
    let multiply = multiply(forward.clone());
    let divide = divide(forward.clone());
    let if_ = condition(forward.clone());

    let parsers = vec![
        int_,
        bool_,
        symbol,
        quoted_string,
        add,
        subtract,
        multiply,
        divide,
        if_
    ];
    let expr = choice(parsers).ws();
    unsafe {
        let forward_ref = Rc::get_mut_unchecked(&mut forward);
        forward_ref.parser = Some(expr);
    }
    forward
}

fn assign<'a>() -> RcParser<'a, Expr> {
    let ident = string_ident();
    let let_ = pstring("let").ws1();
    let equal = pchar('=').ws();
    let name = ident.between(let_, equal);

    name.then(expr())
        .map(|(name, value)| Expr::Ident(name, Box::new(value)))
}

fn returns<'a>() -> RcParser<'a, Expr> {
    let return_ = pstring("return").ws1();
    return_
        .right(expr())
        .map(|value| Expr::Return(Box::new(value)))
}


pub fn body<'a>() -> RcParser<'a, Vec<Expr>> {
    let assign = assign();
    let return_ = returns();

    choice(vec![assign, return_]).ws().many()
}

pub fn function<'a>() -> RcParser<'a, Function> {
    let name = pstring("function").ws1().right(string_ident()).ws().left(pchar('{')).ws();
    let func = name.then(body()).ws().left(pchar('}'));
    func.map(|(name, body)| { Function { name, body }})
}
