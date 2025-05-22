//! # Parser
//!
//! Parses the input token stream into an AST and performs syntactic analysis.

use crate::lexer;
use crate::token;

pub mod ast;
pub mod error;

/// Parses the token stream into an AST.
pub struct Parser<'a> {
    /// Lexer instance to read tokens from.
    lexer: &'a mut lexer::Lexer<'a>,
    /// The current token.
    current: Option<token::Token>,
    /// The next token.
    peek: Option<token::Token>,
    /// Accrued parsing errors
    errors: Vec<error::ParserError>,
}

impl<'a> Parser<'a> {
    /// Creates a new parser instance.
    pub fn new(lexer: &'a mut lexer::Lexer<'a>) -> Self {
        let mut parser = Self {
            lexer,
            current: None,
            peek: None,
            errors: Vec::new(),
        };

        // Read two token so set `current` and `peek`
        parser.next_token();
        parser.next_token();
        parser
    }

    /// Moves the current token to the `current` field and puts the next token
    /// into the `peek` field.
    fn next_token(&mut self) {
        self.current = self.peek.take();
        self.peek = Some(self.lexer.next_token());
    }

    /// Determine whether the current token matches the specific token variant.
    fn current_token_is(&self, t: &token::Token) -> bool {
        matches!(self.current.as_ref(), Some(current) if current == t)
    }

    /// Determine whether the peek token matches the specific token variant.
    fn peek_token_is(&self, t: &token::Token) -> bool {
        matches!(self.peek.as_ref(), Some(peek) if peek == t)
    }

    /// Assertion function to check if the type of the next token matches its
    /// expected type, and only then advancing the tokens.
    fn expect_peek_token(&mut self, t: &token::Token) -> Result<(), error::ParserError> {
        if self.peek_token_is(t) {
            self.next_token();
            Ok(())
        } else {
            Err(error::ParserError::new(format!(
                "Expected next token to be {:?}, received {:?}",
                t, self.peek
            )))
        }
    }

    /// Parses a statement, returning an AST node if successful, else a
    /// `ParserError`.
    fn parse_statement(&mut self) -> Result<ast::Statement, error::ParserError> {
        match self.current {
            Some(token::Token::Let) => self.parse_let_statement(),
            _ => Err(error::ParserError::new("Unexpected token".to_string())),
        }
    }

    /// Parses a let statement, returning an AST node if successful, else a
    /// `ParserError`.
    fn parse_let_statement(&mut self) -> Result<ast::Statement, error::ParserError> {
        if let Some(token) = &self.current {
            if token != &token::Token::Let {
                return Err(error::ParserError::new(format!(
                    "Expected 'let' token, got {:?}",
                    token
                )));
            }
        }

        let ident = match &self.peek {
            Some(token::Token::Ident(ident)) => ident.clone(),
            _ => {
                return Err(error::ParserError::new(
                    "Expected identifier after 'let'".to_string(),
                ))
            }
        };

        // Consume the identifier
        self.next_token();

        // Check that the next token is an assignment
        self.expect_peek_token(&token::Token::Assign)?;

        // TODO: skipping expressions until encounter a semicolon for now
        // need to handle expressions
        while !matches!(self.current, Some(token::Token::Semicolon)) {
            self.next_token();
        }

        // TODO: replace placeholder expression
        Ok(ast::Statement::Let(
            ident,
            ast::Expression::Identifier("5".to_string()),
        ))
    }

    /// Parse the input token into a program AST (a series of statements).
    fn parse_program(&mut self) -> Result<Vec<ast::Statement>, error::ParserError> {
        let mut statements: Vec<ast::Statement> = Vec::new();

        while let Some(current) = self.current.as_ref() {
            // reached end of file
            if *current == token::Token::Eof {
                break;
            }

            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(err) => {
                    self.errors.push(err);
                }
            }
            // Advance tokens
            self.next_token();
        }

        // Return a parsing error if any errors were encountered.
        if !self.errors.is_empty() {
            // collect errors for display
            let error_messages: Vec<String> = self.errors.iter().map(|e| e.to_string()).collect();
            return Err(error::ParserError::new(format!(
                "Encountered {} errors while parsing:\n{}",
                self.errors.len(),
                error_messages.join("\n")
            )));
        }

        Ok(statements)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_let_statement() {
        let input = "let x = 5; \
                                   let y = 10; \
                                   let foobar = 838383;";

        let mut l = lexer::Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        assert!(program.is_ok());
        let program = program.unwrap();
        if program.len() != 3 {
            panic!(
                "program does not contain 3 statements. got={}",
                program.len()
            );
        }

        let expected = [
            ast::Statement::Let(
                "x".to_string(),
                ast::Expression::Identifier("5".to_string()),
            ),
            ast::Statement::Let(
                "y".to_string(),
                ast::Expression::Identifier("10".to_string()),
            ),
            ast::Statement::Let(
                "foobar".to_string(),
                ast::Expression::Identifier("838383".to_string()),
            ),
        ];

        // TODO: compare expression as well once implemented
        for (i, expected_stmt) in expected.iter().enumerate() {
            let ast::Statement::Let(expected_name, _) = &expected_stmt;
            let ast::Statement::Let(actual_name, _) = &program[i];
            assert_eq!(
                expected_name, actual_name,
                "statement {} does not match the name, expected={:?}, got={:?}",
                i, expected_name, actual_name
            );
        }
    }

    #[test]
    fn test_invalid_let_statement() {
        let input = "let x 5;";
        let mut l = lexer::Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        assert!(&program.is_err());
    }
}
