#[macro_use]
extern crate lazy_static;
extern crate regex;

mod lexer;
mod parser;

use std::io::{self, BufRead, Write};

use lexer::{OperatorKind, lex};
use parser::{AstNode, parse_expression};

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
                OperatorKind::Plus => evaluate(value),
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
