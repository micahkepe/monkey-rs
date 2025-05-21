//! # REPL
//!
//! Defines a Read-Eval-Print-Loop (REPL) for the Monkey programming language.
use std::io::{stdin, stdout, Write};

use crate::lexer;
use crate::token;

/// The prompt to display to the user
const PROMPT: &str = ">> ";

/// Runs a simple REPL for the user to run Monkey code.
pub fn start() {
    println!(
        r"
       __  ___          __
      /  |/  /__  ___  / /_____ __ __
     / /|_/ / _ \/ _ \/  '_/ -_) // /
    /_/  /_/\___/_//_/_/\_\\__/\_, /
                              /___/
        "
    );
    println!("Welcome to the Monkey programming language!");
    println!("Feel free to type in commands\n");

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
