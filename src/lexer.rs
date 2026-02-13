/// Represents a single token in the Kāra source code.
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    Record,
    Define,
    Sutra, // Note: Using Sutra to represent 'Sūtra' in the enum
    Flow,
    Let,
    Action,
    From,
    Into,
    Require,
    Return,
    As,

    // Symbols
    Colon,      // :
    Semicolon,  // ;
    Comma,      // ,
    Equal,      // =
    LParen,     // (
    RParen,     // )
    LBrace,     // {
    RBrace,     // }
    Arrow,      // ->
    Dot,        // .

    // Literals and Identifiers
    Identifier(String),
    Number(f64), // Representing all numbers as f64 for now
    String(String),

    // Special Tokens
    EOF, // End of File
    Illegal(String), // For unrecognized characters
}

/// The Lexer, responsible for turning source code into a stream of tokens.
pub struct Lexer {
    source: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    /// Creates a new Lexer instance.
    pub fn new(source: String) -> Self {
        let mut lexer = Lexer {
            source: source.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }

    /// Reads the next character and advances the lexer's position.
    fn read_char(&mut self) {
        if self.read_position >= self.source.len() {
            self.ch = '\0';
        } else {
            self.ch = self.source[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    /// Peeks at the next character without consuming it.
    fn peek_char(&self) -> char {
        if self.read_position >= self.source.len() {
            '\0'
        } else {
            self.source[self.read_position]
        }
    }

    /// Skips over any whitespace characters.
    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    /// Returns the next token from the source code.
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        // Handle comments
        if self.ch == '/' && self.peek_char() == '/' {
            while self.ch != '\n' && self.ch != '\0' {
                self.read_char();
            }
            self.skip_whitespace(); // Skip more whitespace after the comment
        }

        let tok = match self.ch {
            '=' => Token::Equal,
            ';' => Token::Semicolon,
            ':' => Token::Colon,
            ',' => Token::Comma,
            '.' => Token::Dot,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '-' => {
                if self.peek_char() == '>' {
                    self.read_char(); // consume the '-'
                    self.read_char(); // consume the '>'
                    return Token::Arrow;
                } else {
                    Token::Illegal(self.ch.to_string())
                }
            }
            '"' => {
                return Token::String(self.read_string());
            }
            '\0' => Token::EOF,
            _ => {
                if is_identifier_start(self.ch) {
                    let literal = self.read_identifier();
                    return Self::lookup_ident(&literal);
                } else if self.ch.is_digit(10) {
                    let num_str = self.read_number();
                    return Token::Number(num_str.parse().unwrap_or(0.0));
                } else {
                    Token::Illegal(self.ch.to_string())
                }
            }
        };

        self.read_char();
        tok
    }

    /// Reads a full identifier from the source.
    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_identifier_char(self.ch) {
            self.read_char();
        }
        self.source[position..self.position].iter().collect()
    }

    /// Reads a number (integer or float) from the source.
    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_digit(10) || self.ch == '.' {
            self.read_char();
        }
        self.source[position..self.position].iter().collect()
    }

    /// Reads a string literal from the source.
    fn read_string(&mut self) -> String {
        self.read_char(); // Consume the opening quote
        let position = self.position;
        while self.ch != '"' && self.ch != '\0' {
            self.read_char();
        }
        let result = self.source[position..self.position].iter().collect();
        self.read_char(); // Consume the closing quote
        result
    }

    /// Maps an identifier string to a keyword Token or an Identifier Token.
    fn lookup_ident(ident: &str) -> Token {
        match ident {
            "Record" => Token::Record,
            "Define" => Token::Define,
            "Sūtra" => Token::Sutra,
            "flow" => Token::Flow,
            "let" => Token::Let,
            "Action" => Token::Action,
            "From" => Token::From,
            "Into" => Token::Into,
            "Require" => Token::Require,
            "Return" => Token::Return,
            "as" => Token::As,
            _ => Token::Identifier(ident.to_string()),
        }
    }
}

/// Helper to check if a char can start an identifier.
fn is_identifier_start(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

/// Helper to check if a char can be part of an identifier.
fn is_identifier_char(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}
