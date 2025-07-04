/*!
  Monkey entry program.
*/
use clap::Parser;
use monkey_rs::{
    eval::{self, environment::Env},
    parser, repl,
};
use rustyline::Result;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

/// Runs and evaluates the Monkey source file (`*.monkey`), if provided, else
/// starts a Monkey REPL session to run Monkey code.
#[derive(Parser, Debug)]
struct Args {
    /// Path to a Monkey source file to execute (must have `.monkey` extension).
    #[arg(value_name = "FILE")]
    input: Option<PathBuf>,
}

/// Runs the Monkey interpreter, either executing a source file or starting a
/// REPL session.
///
/// This function parses command-line arguments to determine whether to process
/// a `.monkey` file or launch an interactive REPL session. If a file is
/// provided, it validates the file extension, reads the file contents, parses
/// and evaluates the Monkey code, and outputs the result. If no file is
/// provided, it starts the REPL for interactive code execution.
///
/// # Returns
///
/// - `Ok(())` on successful execution or if an error is handled
/// gracefully (e.g., invalid file extension).
///
/// - `Err(e)` if file reading, parsing, or REPL operations encounter an
/// unrecoverable error.
///
/// # Errors
///
/// - Returns an error if the input file cannot be read (e.g., file not found).
///
/// - Returns an error if the REPL encounters an issue (e.g., interrupted
/// input).
///
/// - Prints an error message and exits gracefully if the file lacks a `.monkey`
/// extension or has no extension.
fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(file) = args.input {
        // Check file extension, if it exists
        if let Some(ext) = file.extension() {
            if ext.to_string_lossy().to_lowercase() != "monkey" {
                eprint!("Error: File must have a .monkey extension");
                return Ok(());
            }
        } else {
            eprintln!("Error: File has no extension");
            return Ok(());
        }

        // Run file contents
        let input = std::fs::read_to_string(file)?;
        let env: Env = Rc::new(RefCell::new(Default::default()));

        // NOTE: only `puts(...)` statements the last executed statement will be
        // emitted to STDOUT
        match parser::parse(&input) {
            Ok(program) => match eval::eval(program, &Rc::clone(&env)) {
                Ok(evaluated) => println!("{}", evaluated),
                Err(e) => eprintln!("{}", e),
            },
            Err(e) => eprintln!("{}", e),
        }
    } else {
        // Start interactive REPL session
        repl::start()?;
    }

    Ok(())
}
