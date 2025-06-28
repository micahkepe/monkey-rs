//! Built-in functions to Monkey

use std::{fmt, rc::Rc};

use super::error;
use super::object;

/// Built-in function provided by Monkey.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Builtin {
    /// Return the length of an iterable Monkey object.
    Len,
}

impl fmt::Display for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Builtin::Len => write!(f, "len"),
        }
    }
}

impl Builtin {
    /// Lookup and retrieve a builtin function object by name/ identifier, if it
    /// exists.
    pub fn lookup(name: &str) -> Option<object::Object> {
        match name {
            "len" => Some(object::Object::Builtin(Builtin::Len)),
            _ => None,
        }
    }

    /// Apply the builtin function on the passed arguments slice.
    pub fn apply(
        &self,
        args: &[Rc<object::Object>],
    ) -> Result<Rc<object::Object>, error::EvaluationError> {
        match self {
            Builtin::Len => {
                check_args_count(1, args.len())?;

                match &*args[0] {
                    object::Object::String(str) => {
                        Ok(Rc::new(object::Object::Integer(str.len() as i64)))
                    }
                    other => Err(error::EvaluationError::new(format!(
                        "argument to `len` not supported, got {}",
                        other
                    ))),
                }
            }
        }
    }
}

/// Verify that the number of arguments passed matches expected count.
fn check_args_count(expected: usize, actual: usize) -> Result<(), error::EvaluationError> {
    match expected == actual {
        true => Ok(()),
        false => Err(error::EvaluationError::new(format!(
            "invalid number of arguments: expected={}, got={}",
            expected, actual
        ))),
    }
}
