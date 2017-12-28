# Shunting Rust
An example implementation of Edsger Dijkstra's [Shunting-yard algorithm](https://en.wikipedia.org/wiki/Shunting-yard_algorithm) in Rust.

This implementation handles:
* Binary `+`, `-`, `*`, `/`, and `^` (exponent) operators
* Unary `+` and `-` operators
* Parentheses
* Associativity (left for most operators, right for exponentiation)

Shunting Rust includes a [regex-based lexer](src/lexer.rs), the [shunting-yard parser](src/parser.rs), an [evaluator](src/evaluate.rs) for the resulting trees, and a simple [read-eval-print loop](src/repl.rs) for checking the program interactively.

It also includes a small set of tests for each component.

## Running
Shunting Rust should run on the latest stable version of Rust -- just clone the repository and use `cargo test` to run tests or `cargo run` to try out the REPL!

```sh
$ git clone https://github.com/LPGhatguy/shunting-rust.git
$ cd shunting-rust
$ cargo run
> 1 + 2
3
```

## Contributing
I started this repository to practice parser construction and idiomatic Rust. It's intended to be an educational resource, but it isn't perfect!

Shunting Rust is notably lacking:

* Proper error handling
* A proper parser test suite
* Floating-point evaluation tests

If you spot any errors, major or minor, feel free to open an issue or pull request!

## License
Shunting Rust is available under [The Unlicense](http://unlicense.org/). See [LICENSE.md](LICENSE.md) for details.