use std::rc::Rc;

pub trait Parser<'a> {
    type Output;
    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), String>;
    fn to_rc(self) -> RcParser<'a, Self::Output>;

    fn map<F: 'a, Out: 'a>(&self, f: F) -> RcParser<'a, Out>
    where
        F: Fn(Self::Output) -> Out,
        Self: Sized + 'a + Clone,
    {
        MapParser {
            f,
            parser: self.clone().to_rc(),
        }
        .to_rc()
    }

    fn optional(&self) -> RcParser<'a, Option<Self::Output>>
    where
        Self: Sized + 'a + Clone,
    {
        OptionParser {
            parser: self.clone().to_rc(),
        }
        .to_rc()
    }

    fn or(&self, other: Self) -> RcParser<'a, Self::Output>
    where
        Self: Sized + 'a + Clone,
    {
        let parsers = vec![self.clone().to_rc(), other.to_rc()];
        choice(parsers)
    }

    fn then<T: 'a>(&self, other: RcParser<'a, T>) -> RcParser<'a, (Self::Output, T)>
    where
        Self: Sized + 'a + Clone,
    {
        AndThenParser {
            parser_a: self.clone().to_rc(),
            parser_b: other.to_rc(),
        }
        .to_rc()
    }

    fn many(&self) -> RcParser<'a, Vec<Self::Output>>
    where
        Self: Sized + 'a + Clone,
    {
        ManyParser {
            parser: self.clone().to_rc(),
        }
        .to_rc()
    }

    fn many1(&self) -> RcParser<'a, Vec<Self::Output>>
    where
        Self: Sized + 'a + Clone,
    {
        Many1Parser {
            parser: self.clone().to_rc(),
        }
        .to_rc()
    }

    fn left<U: 'a>(&self, other: RcParser<'a, U>) -> RcParser<'a, Self::Output>
    where
        Self: Sized + 'a + Clone,
    {
        self.then(other).map(|(l, _)| l)
    }

    fn right<U: 'a>(&self, other: RcParser<'a, U>) -> RcParser<'a, U>
    where
        Self: Sized + 'a + Clone,
    {
        self.then(other).map(|(_, r)| r)
    }

    fn between<U: 'a, V: 'a>(
        &self,
        left: RcParser<'a, U>,
        right: RcParser<'a, V>,
    ) -> RcParser<'a, Self::Output>
    where
        Self: Sized + 'a + Clone,
    {
        left.right(self.left(right))
    }

    fn ws(&self) -> RcParser<'a, Self::Output>
    where
        Self: Sized + 'a + Clone,
    {
        let ws = any_of(&[' ', '\t', '\n', '\r']).many();
        self.left(ws)
    }

    fn ws1(&self) -> RcParser<'a, Self::Output>
    where
        Self: Sized + 'a + Clone,
    {
        let ws = any_of(&[' ', '\t', '\n', '\r']).many1();
        self.left(ws)
    }
}

pub type RcParser<'a, R> = Rc<dyn Parser<'a, Output = R> + 'a>;

impl<'a, R> Parser<'a> for RcParser<'a, R> {
    type Output = R;

    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), String> {
        let parser = self.as_ref();
        parser.parse(input)
    }

    fn to_rc(self) -> RcParser<'a, R> {
        self
    }
}

struct CharParser {
    c: char,
}

impl<'a> Parser<'a> for CharParser {
    type Output = char;
    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), String> {
        if input.is_empty() {
            Result::Err(format!("Empty String - expected {}", self.c))
        } else {
            let head = input.chars().next().unwrap();
            if head == self.c {
                Result::Ok((head, &input[1..]))
            } else {
                Result::Err(format!(
                    "Expected {}, got {}. Remaining {}",
                    self.c, head, input
                ))
            }
        }
    }

    fn to_rc(self) -> RcParser<'a, Self::Output> {
        Rc::new(self)
    }
}

struct StringParser {
    string: &'static str,
}

impl<'a> Parser<'a> for StringParser {
    type Output = &'static str;
    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), String> {
        if let Some(value) = input.strip_prefix(self.string) {
            Result::Ok((self.string, value))
        } else {
            Result::Err(format!("Expected {}", self.string))
        }
    }
    fn to_rc(self) -> RcParser<'a, Self::Output> {
        Rc::new(self)
    }
}

struct AndThenParser<'a, Output1, Output2> {
    parser_a: RcParser<'a, Output1>,
    parser_b: RcParser<'a, Output2>,
}

pub fn and_then<'a, Output1: 'a, Output2: 'a>(
    parser_a: RcParser<'a, Output1>,
    parser_b: RcParser<'a, Output2>,
) -> RcParser<'a, (Output1, Output2)> {
    AndThenParser { parser_a, parser_b }.to_rc()
}

impl<'a, Output1: 'a, Output2: 'a> Parser<'a> for AndThenParser<'a, Output1, Output2> {
    type Output = (Output1, Output2);
    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), String> {
        let result1 = self.parser_a.parse(input);
        match result1 {
            Ok((success1, remaining)) => {
                let result2 = self.parser_b.parse(remaining);
                match result2 {
                    Ok((success2, remaining)) => {
                        let x = (success1, success2);
                        Ok((x, remaining))
                    }
                    Err(error) => Err(format!("Then 2nd : {}", error)),
                }
            }
            Err(error) => Err(format!("Then 1st : {}", error)),
        }
    }

    fn to_rc(self) -> RcParser<'a, Self::Output> {
        Rc::new(self)
    }
}

struct ChoiceParser<'a, Output> {
    parsers: Vec<RcParser<'a, Output>>,
}

impl<'a, Output: 'a> Parser<'a> for ChoiceParser<'a, Output> {
    type Output = Output;
    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), String> {
        for p in &self.parsers {
            let result = p.parse(input);
            match result {
                Ok(success) => return Ok(success),
                Err(_) => continue,
            }
        }
        Err("Expected one of the parsers to succeed".to_string())
    }

    fn to_rc(self) -> RcParser<'a, Self::Output> {
        Rc::new(self)
    }
}

struct MapParser<'a, F, Input, Output>
where
    F: Fn(Input) -> Output,
{
    f: F,
    parser: RcParser<'a, Input>,
}

impl<'a, F: 'a, Input: 'a, Output: 'a> Parser<'a> for MapParser<'a, F, Input, Output>
where
    F: Fn(Input) -> Output,
{
    type Output = Output;
    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), String> {
        let result = self.parser.parse(input);
        match result {
            Ok((success, remaining)) => {
                let mapped = (self.f)(success);
                Ok((mapped, remaining))
            }
            Err(error) => Err(format!("MapParser : {}", error)),
        }
    }

    fn to_rc(self) -> RcParser<'a, Self::Output> {
        Rc::new(self)
    }
}

struct OptionParser<'a, Output> {
    parser: RcParser<'a, Output>,
}

impl<'a, Output: 'a> Parser<'a> for OptionParser<'a, Output> {
    type Output = Option<Output>;
    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), String> {
        let result1 = self.parser.parse(input);
        match result1 {
            Ok((success, remaining)) => Result::Ok((Some(success), remaining)),
            Err(_) => Result::Ok((None, input)),
        }
    }

    fn to_rc(self) -> RcParser<'a, Self::Output> {
        Rc::new(self)
    }
}

struct ManyParser<'a, Output> {
    parser: RcParser<'a, Output>,
}

impl<'a, Output: 'a> Parser<'a> for ManyParser<'a, Output> {
    type Output = Vec<Output>;
    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), String> {
        let mut result = self.parser.parse(input);
        let mut values = Vec::new();
        let mut outerremaining = input;

        while let Ok((success, remaining)) = result {
            values.push(success);
            outerremaining = remaining;
            result = self.parser.parse(remaining);
        }
        Result::Ok((values, outerremaining))
    }

    fn to_rc(self) -> RcParser<'a, Self::Output> {
        Rc::new(self)
    }
}

struct Many1Parser<'a, Output> {
    parser: RcParser<'a, Output>,
}

impl<'a, Output: 'a> Parser<'a> for Many1Parser<'a, Output> {
    type Output = Vec<Output>;
    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), String> {
        let result = self.parser.parse(input);
        let many_parser = self.parser.clone().many();
        match result {
            Ok((success, remaining)) => {
                let (mut result, remain) = many_parser.parse(remaining).unwrap();
                result.insert(0, success);
                Ok((result, remain))
            }
            Err(err) => Err(err),
        }
    }

    fn to_rc(self) -> RcParser<'a, Self::Output> {
        Rc::new(self)
    }
}

pub struct ForwardParser<'a, Output> {
    pub parser: Option<RcParser<'a, Output>>,
}

impl<'a, Output: 'a> Parser<'a> for ForwardParser<'a, Output> {
    type Output = Output;
    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), String> {
        let p = self.parser.as_ref();
        match p {
            Some(parser) => parser.parse(input),
            None => {
                println!("Failed because empty parser");
                Result::Err("Forward Parser not implemented".to_string())
            }
        }
    }

    fn to_rc(self) -> RcParser<'a, Self::Output> {
        Rc::new(self)
    }
}

pub fn forward<'a, Output>() -> Rc<ForwardParser<'a, Output>> {
    Rc::new(ForwardParser { parser: None })
}

pub fn set_implementation<'a, Output>(
    forward: &mut Rc<ForwardParser<'a, Output>>,
    implementation: RcParser<'a, Output>,
) {
    unsafe {
        let forward_ref = Rc::get_mut_unchecked(forward);
        forward_ref.parser = Some(implementation);
    }
}

pub fn pchar<'a>(c: char) -> RcParser<'a, char> {
    CharParser { c }.to_rc()
}

pub fn pstring<'a>(string: &'static str) -> RcParser<'a, &'static str> {
    StringParser { string }.to_rc()
}

pub fn choice<'a, Output: 'a>(parsers: Vec<RcParser<'a, Output>>) -> RcParser<'a, Output> {
    ChoiceParser { parsers }.to_rc()
}

pub fn any_of<'a>(chars: &[char]) -> RcParser<'a, char> {
    let char_parsers: Vec<RcParser<char>> = chars.iter().map(|c| pchar(*c)).collect();
    choice(char_parsers)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn char_parse() {
        let parse_a = pchar('a');
        let result = parse_a.parse("a");
        assert_eq!(result, Result::Ok(('a', "")));
    }

    #[test]
    fn char_parse_with_remaining() {
        let parse_a = pchar('a');
        let result = parse_a.parse("ab");
        assert_eq!(result, Result::Ok(('a', "b")));
    }

    #[test]
    fn str_parse() {
        let parse_hello = pstring("hello");
        let result = parse_hello.parse("hello");
        assert_eq!(result, Result::Ok(("hello", "")));
    }

    #[test]
    fn str_parse_with_remaining() {
        let parse_hello = pstring("hello");
        let result = parse_hello.parse("helloworld");
        assert_eq!(result, Result::Ok(("hello", "world")));
    }

    #[test]
    fn or_a() {
        let parse_a = pchar('a');
        let parse_b = pchar('b');
        let parser = parse_a.or(parse_b);
        let result = parser.parse("a");
        assert_eq!(result, Result::Ok(('a', "")));
    }

    #[test]
    fn or_b() {
        let parse_a = pchar('a');
        let parse_b = pchar('b');
        let parser = parse_a.or(parse_b);
        let result = parser.parse("b");
        assert_eq!(result, Result::Ok(('b', "")));
    }

    #[test]
    fn a_then_b() {
        let parse_a = pchar('a');
        let parse_b = pchar('b');
        let parser = and_then(parse_a, parse_b);
        let result = parser.parse("ab");
        assert_eq!(result, Result::Ok((('a', 'b'), "")));
    }

    #[test]
    fn simple_choice() {
        let parse_a = pchar('a');
        let parse_b = pchar('b');
        let parse_c = pchar('c');

        let parsers = vec![parse_a, parse_b, parse_c];
        let choice_parser = choice(parsers);
        let result = choice_parser.parse("c");
        assert_eq!(result, Result::Ok(('c', "")));
    }

    #[test]
    fn any_of_test() {
        let parsers = vec!['1', '2', '3'];
        let choice_parser = any_of(&parsers);
        let result = choice_parser.parse("3");
        assert_eq!(result, Result::Ok(('3', "")));
    }

    #[test]
    fn map_parser_test() {
        let true_parser = pchar('t');
        let false_parser = pchar('f');

        let true_parser = true_parser.map(move |_| true);
        let false_parser = false_parser.map(move |_| false);

        let true_result = true_parser.parse("t");
        let false_result = false_parser.parse("f");
        assert_eq!(true_result, Result::Ok((true, "")));
        assert_eq!(false_result, Result::Ok((false, "")));
    }

    #[test]
    fn option_parser_test() {
        let true_parser = pchar('t');
        let true_option_parser = true_parser.optional();
        let true_result = true_option_parser.parse("t");

        assert_eq!(true_result, Result::Ok((Some('t'), "")));
    }

    #[test]
    fn option_parser_test_negative() {
        let true_parser = pchar('t');
        let true_option_parser = true_parser.optional();
        let true_result = true_option_parser.parse("-t");

        assert_eq!(true_result, Result::Ok((None, "-t")));
    }

    #[test]
    fn test_arbitrary_string() {
        let mut allowed_chars = Vec::new();
        for c in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            allowed_chars.push(c);
        }
        let chars = any_of(&allowed_chars).many();

        let stringified: RcParser<String> =
            chars.map(move |value: Vec<char>| value.into_iter().collect());

        let result = stringified.parse("SomeValue A");

        assert_eq!(result, Result::Ok(("SomeValue".to_string(), " A")));
    }

    #[test]
    fn between_test() {
        let foo = pstring("foo");
        let lparen = pchar('(');
        let rparen = pchar(')');

        let result = foo.between(lparen, rparen).parse("(foo)");

        assert_eq!(result, Result::Ok(("foo", "")));
    }
}
