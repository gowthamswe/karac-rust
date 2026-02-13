use crate::token::Token;

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


#[cfg(test)]
mod tests {
    use super::{Lexer, Token};

    #[test]
    fn test_full_syntax_lexing() {
        let source = r#"
// Define a complex data structure
Record User {
    id: i64,
    name: String,
    // email is optional
    email: String, 
}

// Define a simple flow
flow RegisterUser {
    let new_user = User {
        id: 101,
        name: "Kāra",
        email: "contact@kara.dev",
    };

    // Use the 'Sūtra' keyword
    Sūtra: LogUser -> ();

    new_user -> LogUser;
}
"#;

        let mut lexer = Lexer::new(source.to_string());

        let expected_tokens = vec![
            Token::Record,
            Token::Identifier("User".to_string()),
            Token::LBrace,
            Token::Identifier("id".to_string()),
            Token::Colon,
            Token::Identifier("i64".to_string()),
            Token::Comma,
            Token::Identifier("name".to_string()),
            Token::Colon,
            Token::Identifier("String".to_string()),
            Token::Comma,
            Token::Identifier("email".to_string()),
            Token::Colon,
            Token::Identifier("String".to_string()),
            Token::Comma,
            Token::RBrace,
            Token::Flow,
            Token::Identifier("RegisterUser".to_string()),
            Token::LBrace,
            Token::Let,
            Token::Identifier("new_user".to_string()),
            Token::Equal,
            Token::Identifier("User".to_string()),
            Token::LBrace,
            Token::Identifier("id".to_string()),
            Token::Colon,
            Token::Number(101.0),
            Token::Comma,
            Token::Identifier("name".to_string()),
            Token::Colon,
            Token::String("Kāra".to_string()),
            Token::Comma,
            Token::Identifier("email".to_string()),
            Token::Colon,
            Token::String("contact@kara.dev".to_string()),
            Token::Comma,
            Token::RBrace,
            Token::Semicolon,
            Token::Sutra,
            Token::Colon,
            Token::Identifier("LogUser".to_string()),
            Token::Arrow,
            Token::LParen,
            Token::RParen,
            Token::Semicolon,
            Token::Identifier("new_user".to_string()),
            Token::Arrow,
            Token::Identifier("LogUser".to_string()),
            Token::Semicolon,
            Token::RBrace,
            Token::EOF,
        ];

        for expected_token in expected_tokens {
            let actual_token = lexer.next_token();
            println!("Expected: {:?}, Got: {:?}", expected_token, actual_token);
            assert_eq!(actual_token, expected_token);
        }
    }
}
