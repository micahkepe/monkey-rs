/*!
# Error

Defines the `EvaluationError` type, which is used to represent errors that occur
during evaluation.
*/
use std::fmt;

/// An error encountered while performing evaluation.
#[derive(Debug, Clone)]
pub struct EvaluationError(String);

impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for EvaluationError {}

impl EvaluationError {
    /// Construct a new parser error with the given message to display.
    pub fn new(msg: String) -> Self {
        EvaluationError(msg)
    }
}
