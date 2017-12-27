/// A straightforward regex-based lexer.

use regex::Regex;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
    Exponent,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Constant(f64),
    Operator(Operator),
    OpenParen,
    CloseParen,
}

lazy_static! {
    static ref PATTERN_WHITESPACE: Regex = Regex::new(r"^\s+").unwrap();
    static ref PATTERN_CONSTANT: Regex = Regex::new(r"^\d+").unwrap();
    static ref PATTERN_OPERATOR: Regex = Regex::new(r"^(\+|-|\*|/|\^)").unwrap();
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

/// Matches the given pattern against the source, using a tokenizer function to
/// turn the match into a token.
fn match_simple<'a, F>(source: &'a str, pattern: &Regex, tokenizer: F) -> Option<(&'a str, Token)>
where
    F: Fn(&'a str) -> Token
{
    if let Some(range) = pattern.find(source) {
        let matched = &source[range.start()..range.end()];
        let rest = &source[range.end()..];

        Some((rest, tokenizer(matched)))
    } else {
        None
    }
}

fn match_constant<'a>(source: &'a str) -> Option<(&'a str, Token)> {
    match_simple(source, &PATTERN_CONSTANT, |v| Token::Constant(v.parse::<f64>().unwrap()))
}

fn match_operator<'a>(source: &'a str) -> Option<(&'a str, Token)> {
    match_simple(source, &PATTERN_OPERATOR, |v| {
        let kind = match v {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            "*" => Operator::Times,
            "/" => Operator::Divide,
            "^" => Operator::Exponent,
            _ => unreachable!(),
        };

        Token::Operator(kind)
    })
}

fn match_paren<'a>(source: &'a str) -> Option<(&'a str, Token)> {
    match_simple(source, &PATTERN_OPEN_PAREN, |_| Token::OpenParen)
        .or_else(|| match_simple(source, &PATTERN_CLOSE_PAREN, |_| Token::CloseParen))
}

/// Create a list of tokens out of the given source.
pub fn lex(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current = source;

    loop {
        // We don't care about whitespace between tokens
        current = eat_whitespace(current);

        let result = match_constant(current)
            .or_else(|| match_operator(current))
            .or_else(|| match_paren(current));

        match result {
            Some((next, token)) => {
                tokens.push(token);
                current = next;
            },
            None => break,
        }
    }

    // If there's stuff left over, that's bad news
    if !current.is_empty() {
        eprintln!("Found unexpected sequence in lexer: {}", current);
    }

    tokens
}

#[test]
fn test_simple() {
    let source = "5 + 6 * 9 ^ 2";
    let tokens = lex(source);
    let expect_tokens = vec![
        Token::Constant(5.0),
        Token::Operator(Operator::Plus),
        Token::Constant(6.0),
        Token::Operator(Operator::Times),
        Token::Constant(9.0),
        Token::Operator(Operator::Exponent),
        Token::Constant(2.0),
    ];

    assert_eq!(tokens, expect_tokens);
}

#[test]
fn test_parens() {
    let source = "(1 * (2 + 3))";
    let tokens = lex(source);
    let expect_tokens = vec![
        Token::OpenParen,
        Token::Constant(1.0),
        Token::Operator(Operator::Times),
        Token::OpenParen,
        Token::Constant(2.0),
        Token::Operator(Operator::Plus),
        Token::Constant(3.0),
        Token::CloseParen,
        Token::CloseParen,
    ];

    assert_eq!(tokens, expect_tokens);
}
