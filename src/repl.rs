//! # REPL
//!
//! Defines a Read-Eval-Print-Loop (REPL) for the Monkey programming language.
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use std::fs;

use crate::lexer;
use crate::token;

/// The prompt to display to the user
const PROMPT: &str = ">> ";

/// Runs a simple REPL for the user to run Monkey code.
pub fn start() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    let history_path = "/tmp/.monkey-history.txt";

    match rl.load_history(history_path) {
        Ok(_) => {}
        Err(ReadlineError::Io(_)) => {
            fs::File::create(history_path)?;
        }
        Err(err) => {
            eprintln!("monkey-rs: Error loading history: {}", err);
        }
    };

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
        let readline = rl.readline(">> ");

        match readline {
            Ok(input) => {
                if input.is_empty() {
                    continue;
                }

                rl.add_history_entry(&input)?;

                let mut l = lexer::Lexer::new(&input);

                loop {
                    let tok = l.next_token();
                    if tok == token::Token::Eof {
                        break;
                    }
                    println!("{:?}", tok)
                }
            }
            Err(ReadlineError::Eof | ReadlineError::Interrupted) => {
                println!("Exiting...");
                rl.save_history(history_path)?;
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
