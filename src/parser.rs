//! # Parser
//!
//! Parses the input token stream into an AST and performs syntactic analysis.
//! The parsing is accomplished via "top down operator precedence," also known
//! as Pratt Parsing based off Vaughan Pratt's 1973 paper ["Top Down Operator
//! Precdence"](https://dl.acm.org/doi/10.1145/512927.512931).

use crate::lexer;
use crate::token;

pub(crate) mod ast;
pub mod error;
pub mod precedence;

/// Exposed function to parse a given input into a `ast::Node::Program`.
pub fn parse(input: &str) -> Result<ast::Node, error::ParserError> {
    let mut lexer = lexer::Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program()?;
    Ok(ast::Node::Program(program))
}

/// Parses the token stream into an AST.
struct Parser<'a> {
    /// Lexer instance to read tokens from.
    lexer: &'a mut lexer::Lexer<'a>,
    /// The current token.
    current_token: Option<token::Token>,
    /// The next token.
    peek_token: Option<token::Token>,
    /// Accrued parsing errors
    errors: Vec<error::ParserError>,
}

impl<'a> Parser<'a> {
    /// Creates a new parser instance.
    pub fn new(lexer: &'a mut lexer::Lexer<'a>) -> Self {
        let mut parser = Self {
            lexer,
            current_token: None,
            peek_token: None,
            errors: Vec::new(),
        };

        // Read two token to set `current` and `peek`
        parser.next_token();
        parser.next_token();
        parser
    }

    /// Moves the current token to the `current` field and puts the next token
    /// into the `peek` field.
    fn next_token(&mut self) {
        self.current_token = self.peek_token.take();
        self.peek_token = Some(self.lexer.next_token());
    }

    /// Determine whether the current token matches the specific token variant.
    fn current_token_is(&self, t: &token::Token) -> bool {
        matches!(self.current_token.as_ref(), Some(current) if current == t)
    }

    /// Determine whether the peek token matches the specific token variant.
    fn peek_token_is(&self, t: &token::Token) -> bool {
        matches!(self.peek_token.as_ref(), Some(peek) if peek == t)
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
                t, self.peek_token
            )))
        }
    }

    /// Returns the precedence of the next token `self.peek`. If the next token
    /// does not exist, then defaults to `Precdence::Lowest`. The returned
    /// precedence value corresponds to the left-binding power of the next
    /// token/operator in the token stream.
    fn peek_precedence(&self) -> precedence::Precdence {
        match &self.peek_token {
            Some(token) => precedence::token_precedence(token),
            None => precedence::Precdence::Lowest,
        }
    }

    /// Returns the precedence of the current token `self.current_token`. If the
    /// current token does not exist, then defaults to `Precdence::Lowest`.
    fn curr_precedence(&self) -> precedence::Precdence {
        match &self.current_token {
            Some(token) => precedence::token_precedence(token),
            None => precedence::Precdence::Lowest,
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

        let ident = match &self.peek_token {
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
        self.next_token();

        // Parse expression
        let expr = self.parse_expression(precedence::Precdence::Lowest)?;

        // Advance parser past the optional semicolon, if it exists
        if self.peek_token_is(&token::Token::Semicolon) {
            self.next_token();
        }

        Ok(ast::Statement::Let(ident, expr))
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

        // Consume the `return`
        self.next_token();

        // Parse expression
        let expr = self.parse_expression(precedence::Precdence::Lowest)?;

        // Place parser after the semicolon, if it exists
        if self.peek_token_is(&token::Token::Semicolon) {
            self.next_token();
        }

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
            Some(token::Token::Int(int)) => Ok(ast::Expression::Lit(ast::Literal::Integer(*int))),
            _ => Err(error::ParserError::new("Expected integer".to_string())),
        }
    }

    /// Attempts to parse the current token as a Boolean literal expression.
    fn parse_boolean(&self) -> Result<ast::Expression, error::ParserError> {
        match &self.current_token {
            Some(token::Token::True) => Ok(ast::Expression::Lit(ast::Literal::Boolean(true))),
            Some(token::Token::False) => Ok(ast::Expression::Lit(ast::Literal::Boolean(false))),
            _ => Err(error::ParserError::new("Expected boolean".to_string())),
        }
    }

    /// Attempts to parse a group expression, starting from the opening
    /// `token::Token::LParen` token.
    fn parse_grouped_expression(&mut self) -> Result<ast::Expression, error::ParserError> {
        // Advance past the opening left parenthesis
        self.next_token();

        let expr = self.parse_expression(precedence::Precdence::Lowest)?;
        self.expect_peek_token(&token::Token::RParen)?;

        Ok(expr)
    }

    /// Parses the if-else expression from the current token, returning an
    /// `ast::Expression::If(...)` node of the condition, consequence, and
    /// optional alternative expressions and block statements, respectively.
    fn parse_if_expression(&mut self) -> Result<ast::Expression, error::ParserError> {
        self.expect_peek_token(&token::Token::LParen)?;
        self.next_token();

        // Parse condition expression
        let condition = self.parse_expression(precedence::Precdence::Lowest)?;
        self.expect_peek_token(&token::Token::RParen)?;

        // Parse consequence expression
        self.expect_peek_token(&token::Token::LBrace)?;
        let consequence = self.parse_block_statement()?;

        // Parse alternative expression, if it exists
        let alternative = if self.peek_token_is(&token::Token::Else) {
            self.next_token();
            self.expect_peek_token(&token::Token::LBrace)?;
            Some(self.parse_block_statement()?)
        } else {
            None
        };

        Ok(ast::Expression::If(
            Box::new(condition),
            consequence,
            alternative,
        ))
    }

    /// Parses the block statement from the current token, which should be on
    /// the opening curly left brace.
    fn parse_block_statement(&mut self) -> Result<ast::BlockStatement, error::ParserError> {
        // Advance past the opening curly brace
        self.next_token();

        let mut block_statement = Vec::new();

        // Continue to parse statement until we either reach the end of the
        // block statement or EOF.
        while !self.current_token_is(&token::Token::RBrace)
            && !self.current_token_is(&token::Token::Eof)
        {
            if let Ok(stmt) = self.parse_statement() {
                block_statement.push(stmt);
            }
            self.next_token();
        }

        Ok(block_statement)
    }

    /// Parses the function literal from the current token.
    fn parse_function_literal(&mut self) -> Result<ast::Expression, error::ParserError> {
        self.expect_peek_token(&token::Token::LParen)?;

        // Parse the parameters of the function
        let parameters = self.parse_function_parameters()?;

        self.expect_peek_token(&token::Token::LBrace)?;

        let body = self.parse_block_statement()?;
        Ok(ast::Expression::Fn(parameters, body))
    }

    /// Parses the parameters of a function literal expression.
    fn parse_function_parameters(&mut self) -> Result<Vec<String>, error::ParserError> {
        let mut identifiers = Vec::new();

        // Early exit in the case of no parameters/ empty list
        if self.peek_token_is(&token::Token::RParen) {
            self.next_token();
            return Ok(identifiers);
        }

        // Advance past the opening left parenthesis
        self.next_token();

        // Add the current identifier
        match &self.current_token {
            Some(token::Token::Ident(ref param)) => identifiers.push(param.clone()),
            Some(token) => {
                return Err(error::ParserError::new(format!(
                    "Expected a parameter identifer, got {}",
                    token
                )))
            }
            None => {
                return Err(error::ParserError::new(
                    "Expected a parameter identifer, received None".to_string(),
                ))
            }
        }

        // Add parameter identifier(s) following the first parameter, if they
        // exist
        while self.peek_token_is(&token::Token::Comma) {
            self.next_token();
            self.next_token();
            match &self.current_token {
                Some(token::Token::Ident(ref param)) => identifiers.push(param.clone()),
                Some(token) => {
                    return Err(error::ParserError::new(format!(
                        "Expected a parameter identifer, got {}",
                        token
                    )))
                }
                None => {
                    return Err(error::ParserError::new(
                        "Expected a parameter identifer, received None".to_string(),
                    ))
                }
            }
        }

        self.expect_peek_token(&token::Token::RParen)?;

        Ok(identifiers)
    }

    /// Parse the function call expression from the current token.
    fn parse_call_expression(
        &mut self,
        expr: ast::Expression,
    ) -> Result<ast::Expression, error::ParserError> {
        let args = self.parse_call_arguments()?;
        Ok(ast::Expression::Call(Box::new(expr), args))
    }

    /// Parse the function call arguments from the current token.
    fn parse_call_arguments(&mut self) -> Result<Vec<ast::Expression>, error::ParserError> {
        let mut args = Vec::new();

        // Early exit in case of empty arguments list
        if self.peek_token_is(&token::Token::RParen) {
            self.next_token();
            return Ok(args);
        }

        // Advance past opening left parenthesis
        self.next_token();

        // Parse first argument expression
        args.push(self.parse_expression(precedence::Precdence::Lowest)?);

        while self.peek_token_is(&token::Token::Comma) {
            self.next_token();
            self.next_token();
            args.push(self.parse_expression(precedence::Precdence::Lowest)?);
        }

        // Check for closing right parenthesis
        self.expect_peek_token(&token::Token::RParen)?;

        Ok(args)
    }

    /// Attempts to parse the current token as a prefix expression.
    fn parse_prefix_expression(&mut self) -> Result<ast::Expression, error::ParserError> {
        let prefix = self.current_token.clone();

        // advance the parser
        self.next_token();

        let expr = self.parse_expression(precedence::Precdence::Prefix)?;

        Ok(ast::Expression::Prefix(
            prefix.expect("Expected a prefix token"),
            Box::new(expr),
        ))
    }

    /// Attempts to parse an infix expression, given the left-hand side to the
    /// infix expression.
    fn parse_infix_expression(
        &mut self,
        left: ast::Expression,
    ) -> Result<ast::Expression, error::ParserError> {
        // Handle the infix operator
        let operator = self.current_token.clone();
        let precedence = self.curr_precedence();
        self.next_token();

        // Parse the right expression
        let right = self.parse_expression(precedence)?;

        Ok(ast::Expression::Infix(
            operator.expect("Expected infix operator"),
            Box::new(left),
            Box::new(right),
        ))
    }

    /// Parses the current expression based on precedence rules. The passed
    /// value for `precedence` signifies the current right-binding power of the
    /// invocation.
    fn parse_expression(
        &mut self,
        precedence: precedence::Precdence,
    ) -> Result<ast::Expression, error::ParserError> {
        let mut left_expr = match self.current_token {
            Some(token::Token::True) | Some(token::Token::False) => self.parse_boolean(),
            Some(token::Token::Ident(_)) => self.parse_identifier(),
            Some(token::Token::Int(_)) => self.parse_integer_literal(),
            Some(token::Token::Bang) | Some(token::Token::Minus) => self.parse_prefix_expression(),
            Some(token::Token::LParen) => self.parse_grouped_expression(),
            Some(token::Token::If) => self.parse_if_expression(),
            Some(token::Token::Function) => self.parse_function_literal(),
            _ => Err(error::ParserError::new(format!(
                "No prefix parse function for {:?}",
                self.current_token
            ))),
        };

        // Try to parse the infix expression, if it exists. Checks if the
        // left-binding power of the next operator/token is higher than the
        // current right-binding power.
        //
        // NOTE: The check for the peek token being a semicolon is not strictly
        // necessary since the `peek_precedence` method will default to
        // returning `Precdence::Lowest`. However, this explicitly sets the
        // semantic behavior of semicolons and expression-ending delimiters.
        while !self.peek_token_is(&token::Token::Semicolon) && precedence < self.peek_precedence() {
            match self.peek_token {
                Some(token::Token::Plus)
                | Some(token::Token::Minus)
                | Some(token::Token::Slash)
                | Some(token::Token::Asterisk)
                | Some(token::Token::Eq)
                | Some(token::Token::NotEq)
                | Some(token::Token::Lt)
                | Some(token::Token::Gt) => {
                    self.next_token();
                    let left = left_expr.unwrap();
                    left_expr = self.parse_infix_expression(left)
                }
                Some(token::Token::LParen) => {
                    self.next_token();
                    let expr = left_expr.unwrap();
                    left_expr = self.parse_call_expression(expr)
                }
                Some(_) => {
                    return Err(error::ParserError::new(format!(
                        "No infix parse function for {:?}",
                        &self.peek_token
                    )))
                }
                None => return left_expr,
            }
        }

        left_expr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks the output of parsing an input program string against the
    /// expected serialized display output for the parsed program AST.
    fn check_parse_test_cases(cases: &[(&str, &str)]) {
        for (input, expected) in cases {
            let mut l = lexer::Lexer::new(input);
            let mut p = Parser::new(&mut l);
            match p.parse_program() {
                Ok(stmts) => {
                    let program = ast::Node::Program(stmts);
                    assert_eq!(expected, &format!("{}", program))
                }
                Err(e) => panic!("Parsing error: {}", e),
            }
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

        let expected = vec![
            ast::Statement::Let(
                "x".to_string(),
                ast::Expression::Lit(ast::Literal::Integer(5)),
            ),
            ast::Statement::Let(
                "y".to_string(),
                ast::Expression::Lit(ast::Literal::Integer(10)),
            ),
            ast::Statement::Let(
                "foobar".to_string(),
                ast::Expression::Lit(ast::Literal::Integer(838383)),
            ),
        ];
        assert_eq!(expected, program)
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
        let expected = vec![
            ast::Statement::Return(ast::Expression::Lit(ast::Literal::Integer(5))),
            ast::Statement::Return(ast::Expression::Lit(ast::Literal::Integer(10))),
            ast::Statement::Return(ast::Expression::Lit(ast::Literal::Integer(993322))),
        ];
        assert_eq!(expected, program)
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
        let expected = vec![ast::Statement::Expr(ast::Expression::Lit(
            ast::Literal::Integer(5),
        ))];
        assert_eq!(expected, program);
    }

    #[test]
    fn test_boolean_expressions() {
        let bool_tests = [("true", "true"), ("false", "false")];
        check_parse_test_cases(&bool_tests);
    }

    #[test]
    fn test_parsing_prefix_expressions() {
        // test_case[i] = (input str, expected formatted parsed representation)
        let prefix_cases = [
            ("!5;", "(!5)"),
            ("-15;", "(-15)"),
            ("!true", "(!true)"),
            ("!false", "(!false)"),
        ];
        check_parse_test_cases(&prefix_cases);
    }

    #[test]
    fn test_parsing_infix_expressions() {
        let infix_tests = [
            ("5 + 5;", "(5 + 5)"),
            ("5 - 5;", "(5 - 5)"),
            ("5 * 5;", "(5 * 5)"),
            ("5 / 5;", "(5 / 5)"),
            ("5 > 5;", "(5 > 5)"),
            ("5 < 5;", "(5 < 5)"),
            ("5 == 5;", "(5 == 5)"),
            ("5 != 5;", "(5 != 5)"),
            ("true == true", "(true == true)"),
            ("true != false", "(true != false)"),
            ("false == false", "(false == false)"),
        ];
        check_parse_test_cases(&infix_tests);
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let precedence_tests = [
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a + b - c", "((a + b) - c)"),
            ("a * b * c", "((a * b) * c)"),
            ("a * b / c", "((a * b) / c)"),
            ("a + b / c", "(a + (b / c))"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
            ("true", "true"),
            ("false", "false"),
            ("3 > 5 == false", "((3 > 5) == false)"),
            ("3 < 5 == true", "((3 < 5) == true)"),
            ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
            ("(5 + 5) * 2", "((5 + 5) * 2)"),
            ("2 / (5 + 5)", "(2 / (5 + 5))"),
            ("-(5 + 5)", "(-(5 + 5))"),
            ("!(true == true)", "(!(true == true))"),
            ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
            (
                "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
                "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
            ),
            (
                "add(a + b + c * d / f + g)",
                "add((((a + b) + ((c * d) / f)) + g))",
            ),
        ];
        check_parse_test_cases(&precedence_tests);
    }

    #[test]
    fn test_if_expression() {
        let if_case = [("if (x < y) { x }", "if (x < y) { x }")];
        check_parse_test_cases(&if_case);
    }

    #[test]
    fn test_ifelse_expression() {
        let ifelse_case = [("if (x < y) { x } else { y }", "if (x < y) { x } else { y }")];
        check_parse_test_cases(&ifelse_case);
    }

    #[test]
    fn test_function_literal_parsing() {
        let fn_literal_case = [("fn(x, y) { x + y; }", "fn(x, y) { (x + y) }")];
        check_parse_test_cases(&fn_literal_case);
    }

    #[test]
    fn test_function_parameter_parsing() {
        let fn_params_cases = [
            ("fn() {};", "fn() {  }"),
            ("fn(x) {};", "fn(x) {  }"),
            ("fn(x, y, z) {};", "fn(x, y, z) {  }"),
        ];
        check_parse_test_cases(&fn_params_cases);
    }

    #[test]
    fn test_call_expression_parsing() {
        let fn_call_cases = [("add(1, 2 * 3, 4 + 5)", "add(1, (2 * 3), (4 + 5))")];
        check_parse_test_cases(&fn_call_cases);
    }
}
