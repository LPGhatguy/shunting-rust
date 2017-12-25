#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
enum Token<'a> {
    Constant(u64),
    Operator(&'a str),
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

fn match_constant<'a>(source: &'a str) -> Option<(&'a str, Token<'a>)> {
    if let Some(range) = PATTERN_CONSTANT.find(source) {
        let matched = &source[range.start()..range.end()];
        let rest = &source[range.end()..];

        let value: u64 = matched.parse().unwrap();

        Some((rest, Token::Constant(value)))
    } else {
        None
    }
}

fn match_operator<'a>(source: &'a str) -> Option<(&'a str, Token<'a>)> {
    None
}

fn lex(source: &str) -> Vec<Token> {
    Vec::new()
}

fn main() {
    let input = "5 + 6 + 3";

    let tokens = lex(input);

    println!("Tokens: {:?}", tokens);
}
