use monkey_rs::repl;
use rustyline::Result;

/// Starts a Monkey REPL session to run Monkey code.
fn main() -> Result<()> {
    repl::start()?;

    Ok(())
}
