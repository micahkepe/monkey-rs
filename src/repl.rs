//! # REPL
//!
//! Defines a Read-Eval-Print-Loop (REPL) for the Monkey programming language.
use std::io::{stdin, stdout, Write};

use crate::lexer;
use crate::token;

const PROMPT: &str = ">> ";

/// Runs a simple REPL for the user to run Monkey code.
pub fn start() {
    loop {
        print!("{}", PROMPT);
        let _ = stdout().flush();

        let mut input = String::new();
        let _ = stdin().read_line(&mut input);

        let mut l = lexer::Lexer::new(&input);

        if input.is_empty() {
            continue;
        }

        loop {
            let tok = l.next_token();
            if tok == token::Token::Eof {
                break;
            }
            println!("{:?}", tok)
        }
    }
}
