#[derive(Debug, PartialEq, Clone)]
pub struct Location {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    Program,
    Begin,
    End,
    Print,

    // Identifiers
    Identifier(String),

    // Literals
    String(String),

    // Symbols
    Semicolon,
    LParen,
    RParen,

    // End of File
    Eof,

    // Illegal token
    Illegal,
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: u8,               // current char under examination
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0; // ASCII code for "NUL"
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            b';' => Token::Semicolon,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'"' => self.read_string(),
            0 => Token::Eof,
            _ => {
                if is_letter(self.ch) {
                    let ident = self.read_identifier();
                    return match ident.as_str() {
                        "program" => Token::Program,
                        "begin" => Token::Begin,
                        "end" => Token::End,
                        "print" => Token::Print,
                        _ => Token::Identifier(ident),
                    };
                } else {
                    Token::Illegal
                }
            }
        };

        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn read_string(&mut self) -> Token {
        let position = self.position + 1;
        loop {
            self.read_char();
            if self.ch == b'"' || self.ch == 0 {
                break;
            }
        }
        Token::String(self.input[position..self.position].to_string())
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
}

fn is_letter(ch: u8) -> bool {
    ch.is_ascii_alphabetic() || ch == b'_'
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "program main;\nbegin\n  print(\"Hello, World!\");\nend";

        let tests = vec![
            Token::Program,
            Token::Identifier("main".to_string()),
            Token::Semicolon,
            Token::Begin,
            Token::Print,
            Token::LParen,
            Token::String("Hello, World!".to_string()),
            Token::RParen,
            Token::Semicolon,
            Token::End,
            Token::Eof,
        ];

        let mut l = Lexer::new(input);

        for t in tests {
            let tok = l.next_token();
            assert_eq!(tok, t);
        }
    }
}
