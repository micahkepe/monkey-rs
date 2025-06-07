//! # AST
//!
//! Defines the nodes that comprise the constructed AST from Monkey source code.

use std::fmt;

/// Defines the nodes that comprise the constructed AST from Monkey source code.
#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    /// A program node, which contains a series of statements.
    Program(Vec<Statement>),
    /// A statement node
    Stmt(Statement),
    /// An expression node
    Expr(Expression),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Program(stmts) => write!(f, "{}", display_program(stmts)),
            Node::Stmt(stmt) => write!(f, "{}", stmt),
            Node::Expr(expr) => write!(f, "{}", expr),
        }
    }
}

/// A statement doesn't produce a value, but rather performs an action or
/// defines a variable.
///
/// In Monkey, there are only three types of statements:
/// 1.  `let` statements, which define a variable with an identifier and an
///     expression.
/// 2.  `return` statements, which return an expression.
/// 3.  `expression` statements, which are expressions that don't return a value.
///
/// # Examples
///
/// ```monkey
/// let x = 5;  // let statement
/// return x;   // return statement
/// x + 1;      // expression statement
/// ```
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Statement {
    /// A let statement, which defines a variable with an identifier and an
    /// expression.
    Let(String, Expression),
    /// A return statement, which returns an expression.
    Return(Expression),
    /// An expression statement, which is an expression that doesn't return a
    /// value.
    Expr(Expression),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Let(id, expr) => write!(f, "let {} = {};", id, expr),
            Statement::Return(expr) => write!(f, "return {};", expr),
            Statement::Expr(expr) => write!(f, "{}", expr),
        }
    }
}

/// An expression is a value or a computation that produces a value.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Expression {
    /// An identifier expression, which represents a variable.
    Identifier(String),
    /// A literal expression, e.g. an integer, boolean, string, array, or hash.
    LitExpr(Literal),
    /// A prefix parse function
    PrefixParseFn,
    /// An infix parse function, which takes another expression (the "left
    /// side") as an argument
    InfixParseFn(Box<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Identifier(id) => write!(f, "{}", id),
            Expression::PrefixParseFn => todo!(),
            Expression::InfixParseFn(_expression) => todo!(),
            Expression::LitExpr(literal) => write!(f, "{}", literal),
        }
    }
}

/// A type of literal expression.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Literal {
    /// An integer literal, e.g. `5;`
    Integer(i32),
    // Add more literal variants here
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Integer(int) => write!(f, "{}", int),
        }
    }
}

/// Format program statements into a string representation.
fn display_program(stmts: &[Statement]) -> String {
    stmts
        .iter()
        .map(|stmt| stmt.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}
