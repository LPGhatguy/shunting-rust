use std::io::{self, BufRead, Write};

use lexer::lex;
use parser::parse_expression;
use evaluate::evaluate;

/// Start the REPL (read-eval-print loop) to prompt the user for input and
/// attempt to process it.
pub fn start() {
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
