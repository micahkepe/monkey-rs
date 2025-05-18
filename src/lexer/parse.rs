use crate::token;

/// The lexer to convert source code into tokens representing the source code.
#[derive(Debug)]
struct Lexer<'a> {
    /// the input source code to tokenize
    input: &'a str,
    /// current position in input (points to current char)
    position: usize,
    /// current reading position in input (after current char)
    read_position: usize,
    /// current char under examination
    ch: Option<char>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        // put the lexer in an initial working state referencing the first
        // character
        lexer.read_char();
        lexer
    }

    /// Update the lexer state to reflect the next character in the input, if
    /// any, and advance the position in the input.
    fn read_char(&mut self) {
        // check if we have reached end of the input
        if self.read_position >= self.input.len() {
            self.ch = None
        } else {
            let remainder = &self.input[self.read_position..];
            if let Some((_, c)) = remainder.char_indices().next() {
                self.ch = Some(c);
                self.position = self.read_position;
                // advance the read position to be a character ahead of the
                // current character position
                self.read_position += c.len_utf8();
                return;
            }
        }
        // reached EOF
        self.ch = None;
        self.position = self.read_position;
    }

    /// Determine and return the next token in the input from the current
    /// character position.
    fn next_token(&mut self) -> token::Token {
        // consume character(s) until no whitespace
        while matches!(self.ch, Some(c) if c.is_whitespace()) {
            self.read_char();
        }

        let token = match self.ch {
            // Single character tokens
            Some('+') => token::Token::Plus,
            Some('-') => token::Token::Minus,
            Some('/') => token::Token::Slash,
            Some('*') => token::Token::Asterisk,
            Some('<') => token::Token::Lt,
            Some('>') => token::Token::Gt,
            Some(';') => token::Token::Semicolon,
            Some('(') => token::Token::LParen,
            Some(')') => token::Token::RParen,
            Some(',') => token::Token::Comma,
            Some('{') => token::Token::LBrace,
            Some('}') => token::Token::RBrace,

            // Multi-character tokens (e.g., identifier, integer, etc.)
            Some(c) if c.is_ascii_alphabetic() => {
                let ident = self.read_indentifier();
                return token::lookup_ident(&ident);
            }
            Some(c) if c.is_ascii_digit() => {
                let literal = self.read_number();
                return token::Token::Int(literal);
            }
            Some('=') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    self.read_char();
                    return token::Token::Eq;
                }
                self.read_char();
                return token::Token::Assign;
            }
            Some('!') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    self.read_char();
                    return token::Token::NotEq;
                } else {
                    self.read_char();
                    return token::Token::Bang;
                }
            }

            // Unknown single character
            Some(_) => token::Token::Illegal,

            // Reached EOF
            None => token::Token::Eof,
        };

        // advance past the consumed character
        self.read_char();
        token
    }

    /// Reads in an identifier and advances the lexer's position until it
    /// encounters a non-letter character
    fn read_indentifier(&mut self) -> String {
        let start = self.position;
        // identifiers can be alphanumeric separated by underscores
        while matches!(self.ch, Some(c) if c.is_ascii_alphanumeric() || c == '_') {
            self.read_char();
        }
        self.input[start..self.position].to_string()
    }

    /// Reads in a number and advances the lexer's position until it encounters
    /// a non-numeric character. Only supports integer values.
    fn read_number(&mut self) -> i32 {
        let start = self.position;
        while matches!(self.ch, Some(c) if c.is_ascii_digit()) {
            self.read_char();
        }
        self.input[start..self.position]
            .parse()
            .expect("Invalid number encountered")
    }

    /// Peeks the next character from the current position of the lexer.
    fn peek_char(&self) -> Option<char> {
        self.input[self.read_position..].chars().next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verify_expected_next_token(expected: &[token::Token], lexer: &mut Lexer) {
        for (i, expected_tk) in expected.iter().enumerate() {
            let token: token::Token = lexer.next_token();
            assert_eq!(
                token,
                expected_tk.clone(),
                "expected[{}] - wrong token. expected={:?}, actual={:?}",
                i,
                expected_tk,
                token
            );
        }
    }

    #[test]
    fn test_unichar_next_token() {
        let input = "=+(){},;";
        let mut l = Lexer::new(input);

        let expected: Vec<token::Token> = vec![
            token::Token::Assign,
            token::Token::Plus,
            token::Token::LParen,
            token::Token::RParen,
            token::Token::LBrace,
            token::Token::RBrace,
            token::Token::Comma,
            token::Token::Semicolon,
            token::Token::Eof,
        ];

        verify_expected_next_token(&expected, &mut l);
    }

    #[test]
    fn mixed_chars() {
        let input = "let five = 5; \
                     let ten = 10; \
                     \
                     let add = fn(x, y) { \
                         x + y; \
                     }; \
            \
            let result = add(five, ten); \
            !-/*5; \
            5 < 10 > 5;
            \
            if (5 < 10) { \
                return true; \
            } else { \
                return false; \
            } \
            \
            10 == 10; \
            10 != 9;";

        let mut l = Lexer::new(input);
        let expected: Vec<token::Token> = vec![
            token::Token::Let,
            token::Token::Ident("five".to_string()),
            token::Token::Assign,
            token::Token::Int(5),
            token::Token::Semicolon,
            token::Token::Let,
            token::Token::Ident("ten".to_string()),
            token::Token::Assign,
            token::Token::Int(10),
            token::Token::Semicolon,
            token::Token::Let,
            token::Token::Ident("add".to_string()),
            token::Token::Assign,
            token::Token::Function,
            token::Token::LParen,
            token::Token::Ident("x".to_string()),
            token::Token::Comma,
            token::Token::Ident("y".to_string()),
            token::Token::RParen,
            token::Token::LBrace,
            token::Token::Ident("x".to_string()),
            token::Token::Plus,
            token::Token::Ident("y".to_string()),
            token::Token::Semicolon,
            token::Token::RBrace,
            token::Token::Semicolon,
            token::Token::Let,
            token::Token::Ident("result".to_string()),
            token::Token::Assign,
            token::Token::Ident("add".to_string()),
            token::Token::LParen,
            token::Token::Ident("five".to_string()),
            token::Token::Comma,
            token::Token::Ident("ten".to_string()),
            token::Token::RParen,
            token::Token::Semicolon,
            token::Token::Bang,
            token::Token::Minus,
            token::Token::Slash,
            token::Token::Asterisk,
            token::Token::Int(5),
            token::Token::Semicolon,
            token::Token::Int(5),
            token::Token::Lt,
            token::Token::Int(10),
            token::Token::Gt,
            token::Token::Int(5),
            token::Token::Semicolon,
            token::Token::If,
            token::Token::LParen,
            token::Token::Int(5),
            token::Token::Lt,
            token::Token::Int(10),
            token::Token::RParen,
            token::Token::LBrace,
            token::Token::Return,
            token::Token::True,
            token::Token::Semicolon,
            token::Token::RBrace,
            token::Token::Else,
            token::Token::LBrace,
            token::Token::Return,
            token::Token::False,
            token::Token::Semicolon,
            token::Token::RBrace,
            token::Token::Int(10),
            token::Token::Eq,
            token::Token::Int(10),
            token::Token::Semicolon,
            token::Token::Int(10),
            token::Token::NotEq,
            token::Token::Int(9),
            token::Token::Semicolon,
            token::Token::Eof,
        ];

        verify_expected_next_token(&expected, &mut l);
    }
}
