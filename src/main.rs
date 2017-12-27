#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::io::{self, BufRead, Write};

use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum OperatorKind {
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
enum Token {
    Constant(i64),
    Operator(OperatorKind),
}

#[derive(Debug, PartialEq, Eq)]
enum AstNode {
    Constant {
        value: i64,
    },
    BinaryOperator {
        kind: OperatorKind,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    UnaryOperator {
        kind: OperatorKind,
        value: Box<AstNode>,
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
        eprintln!("Found unexpected sequence: {}", current);
    }

    tokens
}

fn parse_expression(mut tokens: &[Token]) -> Option<AstNode> {
    let mut operator_stack: Vec<OperatorKind> = Vec::new();
    let mut operand_stack: Vec<AstNode> = Vec::new();

    loop {
        let token = match tokens.first() {
            Some(token) => token,
            None => break,
        };

        tokens = &tokens[1..];

        match *token {
            Token::Constant(value) => {
                operand_stack.push(AstNode::Constant {
                    value
                });
            },
            Token::Operator(operator) => {
                // clear all operators of higher precedence from the stack
                loop {
                    {
                        let top_operator = match operator_stack.last() {
                            Some(operator) => operator,
                            None => break,
                        };

                        if operator.precedence() >= top_operator.precedence() {
                            break;
                        }
                    }

                    let top_operator = operator_stack.pop().unwrap();
                    let right = Box::new(operand_stack.pop().unwrap());
                    let left = Box::new(operand_stack.pop().unwrap());

                    operand_stack.push(AstNode::BinaryOperator {
                        kind: top_operator,
                        left,
                        right,
                    });
                }

                operator_stack.push(operator);
            },
        }
    }

    while !operator_stack.is_empty() {
        let top_operator = operator_stack.pop().unwrap();
        let right = Box::new(operand_stack.pop().unwrap());
        let left = Box::new(operand_stack.pop().unwrap());

        operand_stack.push(AstNode::BinaryOperator {
            kind: top_operator,
            left,
            right,
        });
    }

    if operand_stack.is_empty() {
        None
    } else {
        Some(operand_stack.pop().unwrap())
    }
}

fn evaluate(ast: &AstNode) -> i64 {
    match *ast {
        AstNode::Constant { value } => value,
        AstNode::BinaryOperator { kind, ref left, ref right } => {
            match kind {
                OperatorKind::Plus => evaluate(left) + evaluate(right),
                OperatorKind::Minus => evaluate(left) - evaluate(right),
                OperatorKind::Times => evaluate(left) * evaluate(right),
                OperatorKind::Divide => evaluate(left) / evaluate(right),
            }
        },
        AstNode::UnaryOperator { kind, ref value } => {
            match kind {
                OperatorKind::Minus => -evaluate(value),
                _ => unreachable!(),
            }
        },
    }
}

fn main() {
    let stdin = io::stdin();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        stdin
            .lock()
            .read_line(&mut input)
            .expect("Couldn't read line!");

        let tokens = lex(&input);

        let ast = match parse_expression(&tokens) {
            Some(ast) => ast,
            None => {
                eprintln!("Could not parse expression!");
                continue;
            },
        };

        let result = evaluate(&ast);

        println!("{}", result);
    }
}
