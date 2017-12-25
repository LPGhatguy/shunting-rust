#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
enum BinaryOperatorKind {
    Plus,
    Minus,
    Times,
    Divide,
}

#[derive(Debug, PartialEq, Eq)]
enum Token {
    Constant(u64),
    Operator(BinaryOperatorKind),
}

#[derive(Debug, PartialEq, Eq)]
enum AstNode {
    Constant {
        value: u64,
    },
    BinaryOperator {
        kind: BinaryOperatorKind,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
}

lazy_static! {
    static ref PATTERN_WHITESPACE: Regex = Regex::new(r"^\s+").unwrap();
    static ref PATTERN_CONSTANT: Regex = Regex::new(r"^\d+").unwrap();
    static ref PATTERN_OPERATOR: Regex = Regex::new(r"^(\+|-|\*|/)").unwrap();
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

        let value: u64 = matched.parse().unwrap();

        Some((rest, Token::Constant(value)))
    } else {
        None
    }
}

fn match_operator<'a>(source: &'a str) -> Option<(&'a str, Token)> {
    if let Some(range) = PATTERN_OPERATOR.find(source) {
        let rest = &source[range.end()..];

        let kind = match &source[range.start()..range.end()] {
            "+" => BinaryOperatorKind::Plus,
            "-" => BinaryOperatorKind::Minus,
            "*" => BinaryOperatorKind::Times,
            "/" => BinaryOperatorKind::Divide,
            _ => unreachable!(),
        };

        Some((rest, Token::Operator(kind)))
    } else {
        None
    }
}

fn lex(source: &str) -> Vec<Token> {
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
        eprintln!("Found garbage at end: {}", current);
    }

    tokens
}

fn parse(tokens: Vec<Token>) {

}

fn main() {
    let input = "5 + 6 + 3";

    let tokens = lex(input);

    println!("Tokens: {:?}", tokens);
}
