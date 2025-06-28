/*!
# AST

Defines the nodes that comprise the constructed AST from Monkey source code.
*/

use std::fmt;

use crate::token;

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
            Node::Program(stmts) => write!(f, "{}", display_statements(stmts)),
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

/// Represents the series of statements enclosed within an opening `{{` and a
/// closing `}}`.
pub type BlockStatement = Vec<Statement>;

/// An expression is a value or a computation that produces a value.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Expression {
    /// An identifier expression, which represents a variable.
    Identifier(String),

    /// A literal expression, e.g. an integer, boolean, string, array, or hash.
    Lit(Literal),

    /// A prefix parse function
    Prefix(token::Token, Box<Expression>),

    /// An infix parse function, which takes another expression (the "left
    /// side") as an argument
    Infix(token::Token, Box<Expression>, Box<Expression>),

    /// An if expression, where the produced value is the last evaluated line.
    /// An if expression can be defined by the following grammar:
    /// ```ebnf
    /// if (<condition>) <consequence> else <alternative>
    /// ```
    /// where `consequence` and `alternative` are block statements.
    If(Box<Expression>, BlockStatement, Option<BlockStatement>),

    /// A function literal. Abstractly, function literals can be represented
    /// as follows:
    /// ```ebnf
    /// fn <parameters> <block statement>
    /// ```
    /// where parameters is an optionally-empty comma-separated list of
    /// identifiers surrounded by parentheses:
    /// ```ebnf
    /// (<parameter one>, <parameter two>, <parameter three>, ...)
    /// ```
    Fn(Vec<String>, BlockStatement),

    /// A function call expression, which can be represented abstractly as:
    /// ```ebnf
    /// <expression>(<comma-separated expressions>)
    /// ```
    Call(Box<Expression>, Vec<Expression>),

    /// Index expression. Abstractly, an index expression can be represented as
    /// follows:
    /// ```ebnf
    /// <expression>\[<expression>\]
    /// ```
    ///
    /// The left expression is the object being accessed, and the right index
    /// expression is an expression that semantic must produce an integer.
    Index(Box<Expression>, Box<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Identifier(id) => write!(f, "{}", id),
            Expression::Prefix(op, right) => write!(f, "({}{})", op, right),
            Expression::Infix(op, left, right) => write!(f, "({} {} {})", left, op, right),
            Expression::Lit(literal) => write!(f, "{}", literal),
            Expression::If(condition, consequence, alternative) => {
                if let Some(alternative) = alternative {
                    write!(
                        f,
                        "if {} {{ {} }} else {{ {} }}",
                        condition,
                        display_statements(consequence),
                        display_statements(alternative),
                    )
                } else {
                    write!(
                        f,
                        "if {} {{ {} }}",
                        condition,
                        display_statements(consequence),
                    )
                }
            }
            Expression::Fn(parameters, body) => {
                write!(
                    f,
                    "fn({}) {{ {} }}",
                    parameters.join(", "),
                    display_statements(body)
                )
            }
            Expression::Call(function_expr, arguments) => {
                write!(f, "{}({})", function_expr, display_expressions(arguments))
            }
            Expression::Index(left, index) => write!(f, "({}[{}])", left, index),
        }
    }
}

/// A type of literal expression.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Literal {
    /// An integer literal, e.g. `5;`
    Integer(i32),
    /// A Boolean literal, e.g. `true` or `false`
    Boolean(bool),
    /// A string literal, e.g. `\"Hello world!\"`
    String(String),
    /// An array literal, e.g. `\[1, 2, 3 + 3, fn(x) { x }, add(2, 2)\]`
    Array(Vec<Expression>),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Integer(int) => write!(f, "{}", int),
            Literal::Boolean(bool) => write!(f, "{}", bool),
            Literal::String(str) => write!(f, "\"{}\"", str),
            Literal::Array(expressions) => write!(f, "[{}]", display_expressions(expressions)),
        }
    }
}

/// Format program statements into a string representation delimited by an
/// empty string.
pub fn display_statements(stmts: &[Statement]) -> String {
    stmts
        .iter()
        .map(|stmt| stmt.to_string())
        .collect::<Vec<String>>()
        .join("")
}

/// Format expressions array into a comma-separated string representation.
pub fn display_expressions(expressions: &[Expression]) -> String {
    expressions
        .iter()
        .map(|expr| expr.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}
