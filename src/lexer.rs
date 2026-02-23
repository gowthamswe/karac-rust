// src/lexer.rs

//! This module is responsible for turning a stream of characters (the source code)
//! into a stream of tokens.

use crate::token::Token;

/// The Lexer struct holds the state required to tokenize the input source code.
pub struct Lexer<'a> {
    source: &'a [u8],
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Lexer<'a> {
    /// Creates a new Lexer.
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source: source.as_bytes(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    /// Scans the next token from the source code.
    pub fn next_token(&mut self) -> Token {
        self.scan_token()
    }

    /// Scans the next token from the source code.
    fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return Token::EOF;
        }

        let c = self.advance();

        match c {
            b'(' => self.make_token(Token::LeftParen),
            b')' => self.make_token(Token::RightParen),
            b'{' => self.make_token(Token::LeftBrace),
            b'}' => self.make_token(Token::RightBrace),
            b',' => self.make_token(Token::Comma),
            b':' => self.make_token(Token::Colon),
            b';' => self.make_token(Token::Semicolon),
            b'+' => self.make_token(Token::Plus),

            b'!' => {
                let token = if self.match_char(b'=') {
                    Token::BangEqual
                } else {
                    Token::Bang
                };
                self.make_token(token)
            }
            b'=' => self.make_token(Token::Equal),
            b'<' => {
                let token = if self.match_char(b'=') {
                    Token::LessThanOrEqual
                } else {
                    Token::LessThan
                };
                self.make_token(token)
            }
            b'>' => {
                let token = if self.match_char(b'=') {
                    Token::GreaterThanOrEqual
                } else {
                    Token::GreaterThan
                };
                self.make_token(token)
            }

            b'-' => {
                if self.match_char(b'>') {
                    self.make_token(Token::Arrow)
                } else {
                    self.error_token("Unexpected character. Expected '->'.")
                }
            }
            b'"' => self.string(),
            _ if is_digit(c) => self.number(),
            _ if is_alpha(c) => self.identifier(),
            _ => self.error_token("Unexpected character."),
        }
    }
    
    fn number(&mut self) -> Token {
        while is_digit(self.peek()) { self.advance(); }

        let mut is_float = false;
        if self.peek() == b'.' && is_digit(self.peek_next()) {
            is_float = true;
            self.advance();
            while is_digit(self.peek()) { self.advance(); }
        }

        let text = std::str::from_utf8(&self.source[self.start..self.current]).unwrap();
        
        if is_float {
            text.parse::<f64>().ok().map_or_else(
                || self.error_token("Invalid float literal."),
                |v| self.make_token(Token::Float(v)),
            )
        } else {
            text.parse::<i64>().ok().map_or_else(
                || self.error_token("Invalid integer literal."),
                |v| self.make_token(Token::Integer(v)),
            )
        }
    }

    fn string(&mut self) -> Token {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' { self.line += 1; }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }

        self.advance(); // The closing quote
        let value = std::str::from_utf8(&self.source[self.start + 1..self.current - 1])
            .unwrap()
            .to_string();
        self.make_token(Token::StringLiteral(value))
    }

    fn identifier(&mut self) -> Token {
        while is_alpha(self.peek()) || is_digit(self.peek()) {
            self.advance();
        }

        let text = std::str::from_utf8(&self.source[self.start..self.current]).unwrap();
        self.make_token(self.identifier_type(text))
    }

    fn identifier_type(&self, text: &str) -> Token {
        match text {
            "fn" => Token::Fn,
            "flow" => Token::Flow,
            "record" => Token::Record,
            "type" => Token::Type,
            "let" => Token::Let,
            "if" => Token::If,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Identifier(text.to_string()),
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                b' ' | b'\r' | b'\t' => { self.advance(); }
                b'\n' => {
                    self.line += 1;
                    self.advance();
                }
                b'/' if self.peek_next() == b'/' => {
                    while self.peek() != b'\n' && !self.is_at_end() { self.advance(); }
                }
                _ => break,
            }
        }
    }

    fn match_char(&mut self, expected: u8) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn advance(&mut self) -> u8 {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() { b'\0' } else { self.source[self.current] }
    }

    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() { b'\0' } else { self.source[self.current + 1] }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn make_token(&self, token_type: Token) -> Token {
        token_type
    }

    fn error_token(&self, message: &str) -> Token {
        eprintln!("[line {}] Error at '{}': {}", self.line, std::str::from_utf8(&self.source[self.start..self.current]).unwrap_or(""), message);
        Token::Error
    }
}

fn is_alpha(c: u8) -> bool {
    (c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z') || c == b'_'
}

fn is_digit(c: u8) -> bool {
    c >= b'0' && c <= b'9'
}
