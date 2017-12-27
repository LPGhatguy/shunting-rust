#[macro_use]
extern crate lazy_static;
extern crate regex;

mod lexer;
mod parser;

use std::io::{self, BufRead, Write};

use lexer::lex;
use parser::{AstNode, BinaryOperatorKind, parse_expression};

fn evaluate(ast: &AstNode) -> f64 {
    match *ast {
        AstNode::Constant { value } => value as f64,
        AstNode::BinaryOperator { ref kind, ref left, ref right } => {
            match *kind {
                BinaryOperatorKind::Plus => evaluate(left) + evaluate(right),
                BinaryOperatorKind::Minus => evaluate(left) - evaluate(right),
                BinaryOperatorKind::Times => evaluate(left) * evaluate(right),
                BinaryOperatorKind::Divide => evaluate(left) / evaluate(right),
                BinaryOperatorKind::Exponent => evaluate(left).powf(evaluate(right)),
            }
        },
        // AstNode::UnaryOperator { kind, ref value } => {
        //     match kind {
        //         Operator::UnaryPlus => evaluate(value),
        //         Operator::UnaryMinus => -evaluate(value),
        //         _ => unreachable!(),
        //     }
        // },
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
