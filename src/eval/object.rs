/*!
# Object

Defines the evaluation objects, e.g., the object system, of the Monkey
programming language.
*/
use std::fmt::Display;
use std::rc::Rc;

use crate::eval::environment;
use crate::parser::ast;

/// Represents objects in Monkey that can represent the values the source AST
/// represents or the values generated from evaluating the AST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Object {
    /// Represents a signed 64-bit integer value.
    Integer(i64),
    /// Represents a Boolean value.
    Boolean(bool),
    /// Represents the absence of a value.
    Null,
    /// Represents a return value object
    ReturnValue(Rc<Object>),
    /// Represents a function literal with given parameters, a body block
    /// statement, and its environment/context.
    Function(Vec<String>, ast::BlockStatement, environment::Env),
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Integer(int) => write!(f, "{}", int),
            Object::Boolean(bool) => write!(f, "{}", bool),
            Object::Null => write!(f, "null"),
            Object::ReturnValue(object) => write!(f, "{}", object),
            Object::Function(params, body, _env) => {
                let params = params.join(", ");
                write!(
                    f,
                    "fn({}) {{\n {} \n}}",
                    params,
                    ast::display_statements(body)
                )
            }
        }
    }
}
