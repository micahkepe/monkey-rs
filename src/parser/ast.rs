//! # AST
//!
//! Defines the nodes that comprise the constructed AST from Monkey source code.

use std::fmt;

/// Defines the nodes that comprise the constructed AST from Monkey source code.
#[derive(Debug)]
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
            Node::Program(stmts) => write!(f, "{}", format_statements(stmts)),
            Node::Stmt(stmt) => write!(f, "{}", stmt),
            Node::Expr(expr) => write!(f, "{}", expr),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Statement {
    Let(String, Expression),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Let(id, expr) => write!(f, "let {} = {};", id, expr),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Expression {
    Identifier(String),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Identifier(id) => write!(f, "{}", id),
        }
    }
}

/// Format a series of statements into a string representation.
fn format_statements(stmts: &[Statement]) -> String {
    stmts
        .iter()
        .map(|stmt| stmt.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}
