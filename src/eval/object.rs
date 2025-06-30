/*!
# Object

Defines the evaluation objects, e.g., the object system, of the Monkey
programming language.
*/
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::rc::Rc;

use crate::eval::{environment, Builtin};
use crate::parser::ast;

/// Represents objects in Monkey that can represent the values the source AST
/// represents or the values generated from evaluating the AST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Object {
    /// Represents a signed 64-bit integer value.
    Integer(i64),
    /// Represents a Boolean value.
    Boolean(bool),
    /// Represent a string value.
    String(String),
    /// Represents the absence of a value.
    Null,
    /// Represents a return value object
    ReturnValue(Rc<Object>),
    /// Represents a function literal with given parameters, a body block
    /// statement, and its environment/context.
    Function(Vec<String>, ast::BlockStatement, environment::Env),
    /// A built-in function
    Builtin(Builtin),
    /// An array, an ordered list of elements of possibly different types.
    Array(Vec<Rc<Object>>),
    /// A hash, a collection of (key, value) pairs, where each key appears at
    /// most once.
    Hash(HashMap<Rc<HashableObject>, Rc<Object>>),
}

/// Represents objects that can be hashed to serve as keys in a hash object.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HashableObject {
    Integer(i64),
    Boolean(bool),
    String(String),
}

impl Display for HashableObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HashableObject::Integer(int) => write!(f, "{int}"),
            HashableObject::Boolean(bool) => write!(f, "{bool}"),
            HashableObject::String(str) => write!(f, "{str}"),
        }
    }
}

impl Object {
    /// Return the object as a [`HashableObject`], if possible.
    pub fn as_hashable(&self) -> Option<HashableObject> {
        match self {
            Object::Integer(int) => Some(HashableObject::Integer(*int)),
            Object::Boolean(bool) => Some(HashableObject::Boolean(*bool)),
            Object::String(str) => Some(HashableObject::String(str.clone())),
            _ => None,
        }
    }
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
            Object::String(str) => write!(f, "{}", str),
            Object::Builtin(builtin) => write!(f, "{}", builtin),
            Object::Array(objects) => write!(
                f,
                "[{}]",
                objects
                    .iter()
                    .map(|obj| obj.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Object::Hash(entries) => {
                let hash = entries
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "{{{}}}", hash)
            }
        }
    }
}
