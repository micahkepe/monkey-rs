//! # Token
//!
//! `token` defines the tokens accepted from a Monkey source file.
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Illegal,
    Eof,

    /// Identifiers and literals
    Ident(String), // add, foobar, x, y, ...
    Int(i32), // [0-9]

    // Operators
    Assign, // =
    Plus,   // +

    // Delimiters
    Comma,     // ,
    Semicolon, // ;
    LParen,    // (
    RParen,    // )
    LBrace,    // {
    RBrace,    // }

    // Keywords
    Function, // fn
    Let,      // let
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Illegal => write!(f, "ILLEGAL"),
            Token::Eof => write!(f, "EOF"),
            Token::Ident(id) => write!(f, "{}", id),
            Token::Int(i) => write!(f, "{}", i),
            Token::Assign => write!(f, "="),
            Token::Plus => write!(f, "+"),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"), // escape
            Token::RBrace => write!(f, "}}"), // escape
            Token::Function => write!(f, "FUNCTION"),
            Token::Let => write!(f, "LET"),
        }
    }
}

/// Map a raw identifier to either a keyword token or an `Ident`
pub fn lookup_ident(ident: &str) -> Token {
    match ident {
        "fn" => Token::Function,
        "let" => Token::Let,

        // user-defined identifier
        _ => Token::Ident(ident.to_string()),
    }
}
