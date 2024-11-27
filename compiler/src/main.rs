use std::fs;

use compiler::lexer::{lex_source, SlicedToken};

fn main() {
    let source = fs::read_to_string("examples/test.rf").unwrap();
    let lexer = lex_source(&source);
    let tokens: Vec<SlicedToken> = lexer
        .into_iter()
        .map(|token| match token {
            Ok(t) => t,
            Err(e) => panic!("{}", e),
        })
        .collect();
    for token in tokens {
        print!("{} ", token)
    }
}
