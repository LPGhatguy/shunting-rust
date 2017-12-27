use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OperatorKind {
    Plus,
    Minus,
    Times,
    Divide,
}

impl OperatorKind {
    pub fn precedence(&self) -> u8 {
        match *self {
            OperatorKind::Plus | OperatorKind::Minus => 1,
            OperatorKind::Times | OperatorKind::Divide => 2,
        }
    }

    pub fn is_left_associative(&self) -> bool {
        match *self {
            _ => true,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Constant(i64),
    Operator(OperatorKind),
    OpenParen,
    CloseParen,
}

lazy_static! {
    static ref PATTERN_WHITESPACE: Regex = Regex::new(r"^\s+").unwrap();
    static ref PATTERN_CONSTANT: Regex = Regex::new(r"^\d+").unwrap();
    static ref PATTERN_OPERATOR: Regex = Regex::new(r"^(\+|-|\*|/)").unwrap();
    static ref PATTERN_OPEN_PAREN: Regex = Regex::new(r"^\(").unwrap();
    static ref PATTERN_CLOSE_PAREN: Regex = Regex::new(r"^\)").unwrap();
}

fn eat_whitespace<'a>(source: &'a str) -> &'a str {
    if let Some(range) = PATTERN_WHITESPACE.find(source) {
        &source[range.end()..]
    } else {
        source
    }
}

fn match_constant<'a>(source: &'a str) -> Option<(&'a str, Token)> {
    if let Some(range) = PATTERN_CONSTANT.find(source) {
        let matched = &source[range.start()..range.end()];
        let rest = &source[range.end()..];

        let value: i64 = matched.parse().unwrap();

        Some((rest, Token::Constant(value)))
    } else {
        None
    }
}

fn match_operator<'a>(source: &'a str) -> Option<(&'a str, Token)> {
    if let Some(range) = PATTERN_OPERATOR.find(source) {
        let rest = &source[range.end()..];

        let kind = match &source[range.start()..range.end()] {
            "+" => OperatorKind::Plus,
            "-" => OperatorKind::Minus,
            "*" => OperatorKind::Times,
            "/" => OperatorKind::Divide,
            _ => unreachable!(),
        };

        Some((rest, Token::Operator(kind)))
    } else {
        None
    }
}

pub fn lex(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current = source;

    loop {
        current = eat_whitespace(current);

        if let Some((next, token)) = match_constant(current) {
            tokens.push(token);
            current = next;
        } else if let Some((next, token)) = match_operator(current) {
            tokens.push(token);
            current = next;
        } else {
            break;
        }
    }

    if !current.is_empty() {
        eprintln!("Found unexpected sequence: {}", current);
    }

    tokens
}
