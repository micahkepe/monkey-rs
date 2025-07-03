/*!
# REPL

Defines a Read-Eval-Print-Loop (REPL) for the Monkey programming language.
*/
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

use crate::eval;
use crate::eval::environment::Env;
use crate::parser;

/// Runs a simple Read-Eval-Print-Loop (REPL) for the user to run Monkey code.
pub fn start() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    let env: Env = Rc::new(RefCell::new(Default::default()));
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
        let mut input = String::new();

        match readline {
            Ok(mut line) => {
                while line.ends_with(' ') {
                    line.pop();
                }
                if line.is_empty() {
                    continue;
                }

                loop {
                    if line.as_bytes().ends_with(b"\\") {
                        // Strip final backslash and add to current input
                        line.pop();
                        input += &line;

                        // Re-prompt for additional lines
                        match rl.readline(".. ") {
                            Ok(next) => {
                                line = next;
                                while line.ends_with(' ') {
                                    line.pop();
                                }
                            }
                            Err(ReadlineError::Eof | ReadlineError::Interrupted) => {
                                println!("Exiting...");
                                rl.save_history(history_path)?;
                                return Ok(());
                            }
                            Err(err) => {
                                println!("Error: {:?}", err);
                                return Err(err);
                            }
                        }
                    } else {
                        // Final line
                        input += &line;
                        break;
                    }
                }

                rl.add_history_entry(&input)?;

                match parser::parse(&input) {
                    Ok(program) => match eval::eval(program, &Rc::clone(&env)) {
                        Ok(evaluated) => println!("{}", evaluated),
                        Err(e) => eprintln!("{}", e),
                    },
                    Err(e) => eprintln!("{}", e),
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
