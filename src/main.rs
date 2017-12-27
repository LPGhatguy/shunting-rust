#[macro_use]
extern crate lazy_static;
extern crate regex;

mod lexer;
mod parser;

use std::io::{self, BufRead, Write};

use lexer::lex;
use parser::{AstNode, BinaryOperatorKind, UnaryOperatorKind, parse_expression};

fn evaluate(ast: &AstNode) -> f64 {
    match *ast {
        AstNode::Constant { value } => value,
        AstNode::BinaryOperator { ref kind, ref left, ref right } => {
            match *kind {
                BinaryOperatorKind::Plus => evaluate(left) + evaluate(right),
                BinaryOperatorKind::Minus => evaluate(left) - evaluate(right),
                BinaryOperatorKind::Times => evaluate(left) * evaluate(right),
                BinaryOperatorKind::Divide => evaluate(left) / evaluate(right),
                BinaryOperatorKind::Exponent => evaluate(left).powf(evaluate(right)),
            }
        },
        AstNode::UnaryOperator { ref kind, ref value } => {
            match *kind {
                UnaryOperatorKind::Plus => evaluate(value),
                UnaryOperatorKind::Minus => -evaluate(value),
            }
        },
    }
}

#[test]
fn test_evaluate() {
    fn check(source: &'static str, expected: f64) {
        let ast = parse_expression(&lex(source)).unwrap();
        let result = evaluate(&ast);

        assert_eq!(result, expected);
    }

    // basic suite
    check("1", 1.0);
    check("2 + 3", 5.0);
    check("(3 + 4)", 7.0);
    check("((4 + 3))", 7.0);
    check("3 * 3 * 3", 27.0);

    // communicativity
    check("1 + 2 * 3", 7.0);
    check("2 * 3 + 1", 7.0);

    // parens
    check("(1 + 2) * 3", 9.0);
    check("3 * (1 + 2)", 9.0);

    // subtraction
    check("5 - 3", 2.0);
    check("3 - 5", -2.0);

    // division
    check("10 / 5", 2.0);
    check("2 * 4 / 2", 4.0);

    // exponents
    check("2 ^ 3", 8.0);
    check("2 ^ (1 + 1 + 1)", 8.0);

    // exponent associativity
    check("3 * 2^2", 12.0);
    check("2^3^2", 512.0);

    // unary operators
    check("-3", -3.0);
    check("--3", 3.0);
    check("5 * -3", -15.0);
    check("10 / -1 * -2", 20.0);
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
