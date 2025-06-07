//! # Parser
//!
//! Parses the input token stream into an AST and performs syntactic analysis.

use crate::lexer;
use crate::token;

pub(crate) mod ast;
pub mod error;
pub mod precedence;

/// Parses the token stream into an AST.
struct Parser<'a> {
    /// Lexer instance to read tokens from.
    lexer: &'a mut lexer::Lexer<'a>,
    /// The current token.
    current_token: Option<token::Token>,
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
            current_token: None,
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
        self.current_token = self.peek.take();
        self.peek = Some(self.lexer.next_token());
    }

    /// Determine whether the current token matches the specific token variant.
    fn current_token_is(&self, t: &token::Token) -> bool {
        matches!(self.current_token.as_ref(), Some(current) if current == t)
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
        match self.current_token {
            Some(token::Token::Let) => self.parse_let_statement(),
            Some(token::Token::Return) => self.parse_return_statement(),
            // Otherwise, default to parsing an expression statement.
            _ => self.parse_expression_statement(),
        }
    }

    /// Parses a let statement, returning an AST node if successful, else a
    /// `ParserError`.
    fn parse_let_statement(&mut self) -> Result<ast::Statement, error::ParserError> {
        if let Some(token) = &self.current_token {
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
        while !matches!(self.current_token, Some(token::Token::Semicolon)) {
            self.next_token();
        }

        // TODO: replace placeholder expression
        Ok(ast::Statement::Let(
            ident,
            ast::Expression::Identifier("let_expr".to_string()),
        ))
    }

    /// Parses a return statement, returning an AST node if successful, else a
    /// `ParserError`.
    fn parse_return_statement(&mut self) -> Result<ast::Statement, error::ParserError> {
        if let Some(token) = &self.current_token {
            if token != &token::Token::Return {
                return Err(error::ParserError::new(format!(
                    "Expected 'return' token, got {:?}",
                    token
                )));
            }
        }

        // Consume the return token
        self.next_token();

        // TODO: skipping the expressions until we encounter a semicolon
        while !matches!(self.current_token, Some(token::Token::Semicolon)) {
            self.next_token();
        }

        // Placeholder expression
        let expr = ast::Expression::Identifier("return_value".to_string());

        Ok(ast::Statement::Return(expr))
    }

    /// Parse a given expression statement.
    fn parse_expression_statement(&mut self) -> Result<ast::Statement, error::ParserError> {
        // Pass an initial lowest precedence since we haven't parse the rest of
        // the expression.
        let expr = self.parse_expression(precedence::Precdence::Lowest)?;

        // Check for optional semicolon, advancing past the semicolon
        // The semicolon is optional to allow expression statements such as
        // `5 + 5` easier to type in the REPL
        if self.peek_token_is(&token::Token::Semicolon) {
            self.next_token();
        }

        Ok(ast::Statement::Expr(expr))
    }

    /// Parse the input token into a program AST (a series of statements).
    fn parse_program(&mut self) -> Result<Vec<ast::Statement>, error::ParserError> {
        let mut statements: Vec<ast::Statement> = Vec::new();

        while let Some(current) = self.current_token.as_ref() {
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

    /// Parses the current token as an identifier expression, else returns a
    /// parse error.
    fn parse_identifier(&self) -> Result<ast::Expression, error::ParserError> {
        match &self.current_token {
            Some(token::Token::Ident(ident)) => Ok(ast::Expression::Identifier(ident.to_string())),
            _ => Err(error::ParserError::new("Expected identifier".to_string())),
        }
    }

    /// Attempts to parse the current token as an integer literal expression.
    fn parse_integer_literal(&self) -> Result<ast::Expression, error::ParserError> {
        match &self.current_token {
            Some(token::Token::Int(int)) => {
                Ok(ast::Expression::LitExpr(ast::Literal::Integer(*int)))
            }
            _ => Err(error::ParserError::new("Expected integer".to_string())),
        }
    }

    /// Parses the current expression based on precedence rules.
    fn parse_expression(
        &self,
        precedence: precedence::Precdence,
    ) -> Result<ast::Expression, error::ParserError> {
        match self.current_token {
            Some(token::Token::Ident(_)) => self.parse_identifier(),
            Some(token::Token::Int(_)) => self.parse_integer_literal(),
            _ => Err(error::ParserError::new(format!(
                "No prefix parse function for {:?}",
                self.current_token
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to compare `Let` statements by their variants, ignoring
    /// expressions for now.
    /// TODO: Handle expressions in the future.
    fn assert_let_statement(expected: &ast::Statement, actual: &ast::Statement, index: usize) {
        match (expected, actual) {
            (ast::Statement::Let(expected_name, _), ast::Statement::Let(actual_name, _)) => {
                assert_eq!(
                    expected_name, actual_name,
                    "statement {} does not match the name, expected={}, got={}",
                    index, expected_name, actual_name
                );
            }
            _ => panic!(
                "statement {} is not a 'Let' statement, expected={:?}, got={:?}",
                index, expected, actual
            ),
        }
    }

    /// Helper function to compare `Return` statements by their variants,
    /// ignoring expressions for now.
    /// TODO: Handle expressions in the future.
    fn assert_return_statement(expected: &ast::Statement, actual: &ast::Statement, index: usize) {
        match (expected, actual) {
            (ast::Statement::Return(_), ast::Statement::Return(_)) => {
                assert_eq!(
                    expected.to_string(),
                    actual.to_string(),
                    "statement {} does not match the expression, expected_expr={}, got={}",
                    index,
                    expected,
                    actual
                );
            }
            _ => panic!(
                "statement {} is not a 'Return' statement, expected={:?}, got={:?}",
                index, expected, actual
            ),
        }
    }

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

        for (i, expected_stmt) in expected.iter().enumerate() {
            assert_let_statement(expected_stmt, &program[i], i);
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

    #[test]
    fn test_return_statements() {
        let input = "return 5; \
                     return 10; \
                     return 993322;";
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
            ast::Statement::Return(ast::Expression::Identifier("5".to_string())),
            ast::Statement::Return(ast::Expression::Identifier("10".to_string())),
            ast::Statement::Return(ast::Expression::Identifier("993322".to_string())),
        ];

        for (i, expected_stmt) in expected.iter().enumerate() {
            assert_return_statement(expected_stmt, &program[i], i);
        }
    }

    #[test]
    fn test_simple_program_display() {
        let input = "let myVar = anotherVar;";
        let mut l = lexer::Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        assert!(program.is_ok());
        let program = ast::Node::Program(program.unwrap());
        let expected = ast::Node::Program(vec![ast::Statement::Let(
            "myVar".to_string(),
            ast::Expression::Identifier("anotherVar".to_string()),
        )]);
        assert_eq!(expected, program);
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";
        let mut l = lexer::Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.len(), 1);
        let expected = vec![ast::Statement::Expr(ast::Expression::Identifier(
            "foobar".to_string(),
        ))];
        assert_eq!(expected, program);
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";
        let mut l = lexer::Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.len(), 1);
        let expected = vec![ast::Statement::Expr(ast::Expression::LitExpr(
            ast::Literal::Integer(5),
        ))];
        assert_eq!(expected, program);
    }
}
