/*!
# Error

Defines the `ParserError` type, which is used to represent errors that occur
during parsing.
*/
use std::fmt;

/// An error encountered while performing parsing.
#[derive(Debug, Clone)]
pub struct ParserError(String);

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ParserError {}

impl ParserError {
    /// Construct a new parser error with the given message to display.
    pub fn new(msg: String) -> Self {
        ParserError(msg)
    }
}
