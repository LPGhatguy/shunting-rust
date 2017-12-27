#[macro_use]
extern crate lazy_static;
extern crate regex;

mod lexer;
mod parser;
mod evaluate;

use std::io::{self, BufRead, Write};

use lexer::lex;
use parser::parse_expression;
use evaluate::evaluate;

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
