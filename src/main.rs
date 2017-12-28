/// Nothing interesting happpens here, everything is broken apart into modules!

#[macro_use]
extern crate lazy_static;
extern crate regex;

// The lexer takes strings and produces lists of tokens
mod lexer;

// The parser takes list of tokens and produces an abstract syntax tree
mod parser;

// The evaluator uses the abstract syntax tree to compute numeric results
mod evaluate;

// The REPL is a small module that takes user input and processes it fully
mod repl;

// Call out to the REPL!
fn main() {
    repl::start();
}
