use crate::ast::*;
use crate::parser_combinator::parser::*;

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

fn comparison<'a>(symbol: &'static str, expr: RcParser<'a, Expr>) -> RcParser<'a, (Expr, Expr)> {
    let comparison = pstring(symbol).ws();

    expr
        .left(comparison)
        .then(expr.clone())
}

fn equals<'a>(expr: RcParser<'a, Expr>) -> RcParser<'a, Expr>  {
    comparison("==", expr).map(|(lhs,rhs)| Expr::Equals(Box::new(lhs), Box::new(rhs)))
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

pub fn body<'a>() -> RcParser<'a, Vec<Expr>> {
    let mut body = forward();

    let expr = {
        let int_ = int();
        let symbol = string_symbol(); //Make quoted
        let quoted_string = string_ident()
            .between(pchar('\"'), pchar('\"'))
            .map(Expr::Str);
        let bool_ = bool();

        let mut forward = forward();

        let add = add(forward.clone());
        let subtract = subtract(forward.clone());
        let multiply = multiply(forward.clone());
        let divide = divide(forward.clone());
        let if_ = condition(forward.clone(), body.clone());
        let equals = equals(forward.clone());
        let return_ = pstring("return")
            .ws1()
            .right(forward.clone())
            .map(|value| Expr::Return(Box::new(value)));

        let parsers = vec![
            if_,
            int_,
            bool_,
            return_,
            symbol,
            quoted_string,
            add,
            subtract,
            multiply,
            divide,
            //equals,
        ];
        let expr = choice(parsers).ws();

        set_implementation(&mut forward, expr);
        forward
    };

    let assign = {
        let ident = string_ident();
        let let_ = pstring("let").ws1();
        let equal = pchar('=').ws();
        let name = ident.between(let_, equal);

        name.then(expr.clone())
            .map(|(name, value)| Expr::Ident(name, Box::new(value))).ws()
    };

    let body_content = choice(vec![assign, expr.clone()])
        .many()
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
        .left(pstring("()"))
        .ws(); //TODO params parsing
    let func = name.then(body());
    func.map(|(name, body)| Function { name, body })
}
