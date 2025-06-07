//! # Token Precedence
//!
//! Defines the precedences of tokens in the Monkey programming language.
use crate::token;

/// Defines the precedences of the Monkey programming language.
#[derive(Debug, PartialEq, Eq)]
pub enum Precdence {
    /// The lowest value precedence
    Lowest,
    /// Equality comparison operator `==`
    Equals,
    /// Strictly greater/less than operators `>` or `<`
    LessGreater,
    /// Summation operator `+`
    Sum,
    /// Multiplication operator `*`
    Product,
    /// Prefix operators, e.g., `-X` or `!X`
    Prefix,
    /// Function calls, e.g., `myFunction(X)`
    Call,
    /// Index access, e.g., `myArray[i]`
    Index,
}

/// Returns the precedence of a given `Token` value.
pub fn token_precedence(token: &token::Token) -> Precdence {
    match token {
        _ => Precdence::Lowest,
    }
}
