use std::collections::HashMap;

use quote::__private::HasIterator;

use crate::ast::*;
use crate::parser_combinator::parser::*;

fn int<'a>() -> RcParser<'a, Expr> {
    let any_number = any_of(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);
    let number_parser = pchar('-').optional().then(any_number.many1());

    number_parser
        .map(move |(negate, value): (Option<char>, Vec<char>)| {
            let string: String = value.into_iter().collect();
            let number = string.parse::<i32>().expect("Incorrectly parsed sequence as an Integer");
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

fn math(symbol: char, expr: RcParser<Expr>) -> RcParser<(Expr, Expr)> {
    let lparen = pchar('(').ws();
    let rparen = pchar(')').ws();
    let operator = pchar(symbol).ws();

    lparen
        .right(expr.clone())
        .left(operator)
        .then(expr.clone())
        .left(rparen)
}

fn add(expr: RcParser<Expr>) -> RcParser<Expr> {
    math('+', expr).map(|(lhs, rhs)| Expr::Add(Box::new(lhs), Box::new(rhs)))
}

fn subtract(expr: RcParser<Expr>) -> RcParser<Expr> {
    math('-', expr).map(|(lhs, rhs)| Expr::Subtract(Box::new(lhs), Box::new(rhs)))
}

fn multiply(expr: RcParser<Expr>) -> RcParser<Expr> {
    math('*', expr).map(|(lhs, rhs)| Expr::Multiply(Box::new(lhs), Box::new(rhs)))
}

fn divide(expr: RcParser<Expr>) -> RcParser<Expr> {
    math('/', expr).map(|(lhs, rhs)| Expr::Divide(Box::new(lhs), Box::new(rhs)))
}

fn modulus(expr: RcParser<Expr>) -> RcParser<Expr> {
    math('%', expr).map(|(lhs, rhs)| Expr::Modulus(Box::new(lhs), Box::new(rhs)))
}

fn comparison<'a>(symbol: &'static str, expr: RcParser<'a, Expr>) -> RcParser<'a, (Expr, Expr)> {
    let comparison = pstring(symbol).ws();
    let lparen = pchar('(').ws();
    let rparen = pchar(')').ws();

    lparen
        .right(expr.clone())
        .left(comparison)
        .then(expr.clone())
        .left(rparen)
}

fn equals(expr: RcParser<Expr>) -> RcParser<Expr> {
    comparison("==", expr).map(|(lhs, rhs)| Expr::Equals(Box::new(lhs), Box::new(rhs)))
}

fn lt(expr: RcParser<Expr>) -> RcParser<Expr> {
    comparison("<", expr).map(|(lhs, rhs)| Expr::LessThan(Box::new(lhs), Box::new(rhs)))
}

fn gt(expr: RcParser<Expr>) -> RcParser<Expr> {
    comparison(">", expr).map(|(lhs, rhs)| Expr::GreaterThan(Box::new(lhs), Box::new(rhs)))
}

fn and(expr: RcParser<Expr>) -> RcParser<Expr> {
    comparison("&&", expr).map(|(lhs, rhs)| Expr::And(Box::new(lhs), Box::new(rhs)))
}

fn while_loop<'a>(expr: RcParser<'a, Expr>, body: RcParser<'a, Vec<Expr>>) -> RcParser<'a, Expr> {
    let while_ = pstring("while").ws1();
    let cond = expr.clone();

    while_
        .right(cond)
        .then(body.clone())
        .map(|(cond, body)| Expr::While(Box::new(cond), body))
}

fn condition<'a>(expr: RcParser<'a, Expr>, body: RcParser<'a, Vec<Expr>>) -> RcParser<'a, Expr> {
    let if_ = pstring("if").ws1();
    let cond = expr.clone();

    if_.right(cond)
        .then(body.clone())
        .left(pstring("else").ws())
        .then(body.clone())
        .map(|((cond, true_body), false_body)| Expr::If(Box::new(cond), true_body, false_body))
}

fn function_call(expr: RcParser<Expr>) -> RcParser<Expr> {
    let function_name = string_ident().ws();
    //TODO implement sepBy then support multiple parameters
    let parameters = expr.between(pchar('('), pchar(')'));

    function_name
        .then(parameters)
        .map(|(name, parameters)| Expr::Call(name, vec![parameters]))
}

pub fn body<'a>() -> RcParser<'a, Vec<Expr>> {
    let mut body = forward();

    let expr: RcParser<'a, Expr> = {
        let int_ = int();
        let symbol = string_symbol();
        let quoted_string = string_ident()
            .between(pchar('\"'), pchar('\"'))
            .map(Expr::Str);
        let bool_ = bool();

        let mut forward = forward();

        let add = add(forward.clone());
        let subtract = subtract(forward.clone());
        let multiply = multiply(forward.clone());
        let divide = divide(forward.clone());
        let modulus = modulus(forward.clone());
        let if_ = condition(forward.clone(), body.clone());
        let while_ = while_loop(forward.clone(), body.clone());
        let equals = equals(forward.clone());
        let lt = lt(forward.clone());
        let gt = gt(forward.clone());
        let and = and(forward.clone());
        let function_call = function_call(forward.clone());
        let return_ = pstring("return")
            .ws1()
            .right(forward.clone())
            .map(|value| Expr::Return(Box::new(value)));

        let assign = {
            let ident = string_ident();
            let equal = pchar('=').ws();
            let name = ident.left(equal);

            name.then(forward.clone())
                .map(|(name, value)| Expr::Ident(name, Box::new(value)))
                .ws()
        };

        let parsers = vec![
            equals,
            lt,
            gt,
            and,
            while_,
            if_,
            assign,
            int_,
            bool_,
            return_,
            function_call,
            symbol,
            quoted_string,
            add,
            subtract,
            multiply,
            divide,
            modulus,
        ];
        let expr = choice(parsers).ws();

        set_implementation(&mut forward, expr);
        forward
    };

    let body_content = expr
        .clone()
        .many1()
        .between(pchar('{').ws(), pchar('}'))
        .ws();

    set_implementation(&mut body, body_content);

    body
}

pub fn function<'a>() -> RcParser<'a, Function> {
    let name = pstring("function")
        .ws1()
        .right(string_ident())
        .ws()
        .then(
            /*string_ident().left(pchar(',').optional().ws())*/
            string_ident().many().between(pchar('(').ws(), pchar(')'))
        )
        .ws(); 
    let func = name.then(body());
    func.map(|((name, params), body)| Function { name, params, body })
}

pub fn module<'a>() -> RcParser<'a, HashMap<String, Function>> {
    function().many1().map(|fns| {
        let mut fns_map = HashMap::new();
        for f in fns {
            fns_map.insert(f.name.clone(), f);
        }
        fns_map
    } )
}