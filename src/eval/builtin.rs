//! Built-in functions to Monkey

use std::{fmt, rc::Rc};

use super::error;
use super::object;

/// Built-in function provided by Monkey.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Builtin {
    /// Return the length of an iterable Monkey object.
    Len,
    /// Return the first element of a given array.
    First,
    /// Return the last element of a given array.
    Last,
    /// Return a new array containing all the elements of the array passed as
    /// argument, except for the first one
    Rest,
    /// Allocates a new array with the same elements as the array passed as
    /// argument with the addition of the new, pushed element.
    Push,
    /// Prints the given arguments to STDOUT
    Puts,
}

impl fmt::Display for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Builtin::Len => write!(f, "len"),
            Builtin::First => write!(f, "first"),
            Builtin::Last => write!(f, "last"),
            Builtin::Rest => write!(f, "rest"),
            Builtin::Push => write!(f, "push"),
            Builtin::Puts => write!(f, "puts"),
        }
    }
}

impl Builtin {
    /// Lookup and retrieve a builtin function object by name/ identifier, if it
    /// exists.
    pub fn lookup(name: &str) -> Option<object::Object> {
        match name {
            "len" => Some(object::Object::Builtin(Builtin::Len)),
            "first" => Some(object::Object::Builtin(Builtin::First)),
            "last" => Some(object::Object::Builtin(Builtin::Last)),
            "rest" => Some(object::Object::Builtin(Builtin::Rest)),
            "push" => Some(object::Object::Builtin(Builtin::Push)),
            "puts" => Some(object::Object::Builtin(Builtin::Puts)),
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
                    object::Object::Array(arr) => {
                        Ok(Rc::new(object::Object::Integer(arr.len() as i64)))
                    }
                    other => Err(error::EvaluationError::new(format!(
                        "argument to `len` not supported, got {}",
                        other
                    ))),
                }
            }
            Builtin::First => {
                check_args_count(1, args.len())?;

                match &*args[0] {
                    object::Object::Array(arr) => match arr.first() {
                        Some(element) => Ok(Rc::clone(element)),
                        None => Ok(Rc::new(object::Object::Null)),
                    },
                    other => Err(error::EvaluationError::new(format!(
                        "argument to `first` must be ARRAY, got {}",
                        other
                    ))),
                }
            }
            Builtin::Last => {
                check_args_count(1, args.len())?;

                match &*args[0] {
                    object::Object::Array(arr) => match arr.last() {
                        Some(element) => Ok(Rc::clone(element)),
                        None => Ok(Rc::new(object::Object::Null)),
                    },
                    other => Err(error::EvaluationError::new(format!(
                        "argument to `last` must be ARRAY, got {}",
                        other
                    ))),
                }
            }
            Builtin::Rest => {
                check_args_count(1, args.len())?;

                match &*args[0] {
                    object::Object::Array(arr) => {
                        let length = arr.len();
                        if length > 0 {
                            let new_elements = arr[1..].to_vec();
                            Ok(Rc::new(object::Object::Array(new_elements)))
                        } else {
                            Ok(Rc::new(object::Object::Null))
                        }
                    }
                    other => Err(error::EvaluationError::new(format!(
                        "argument to `rest` must be ARRAY, got {}",
                        other
                    ))),
                }
            }
            Builtin::Push => {
                check_args_count(2, args.len())?;

                match &*args[0] {
                    object::Object::Array(arr) => {
                        let mut new_elements = arr.clone();
                        new_elements.push(Rc::clone(&args[1]));
                        Ok(Rc::new(object::Object::Array(new_elements)))
                    }
                    other => Err(error::EvaluationError::new(format!(
                        "argument to `push` must be ARRAY, got {}",
                        other
                    ))),
                }
            }
            Builtin::Puts => {
                args.iter().for_each(|obj| println!("{}", obj));

                // Puts returns a null value
                Ok(Rc::new(object::Object::Null))
            }
        }
    }
}

/// Verify that the number of arguments passed matches expected count.
fn check_args_count(expected: usize, actual: usize) -> Result<(), error::EvaluationError> {
    match expected == actual {
        true => Ok(()),
        false => Err(error::EvaluationError::new(format!(
            "wrong number of arguments: expected={}, got={}",
            expected, actual
        ))),
    }
}
