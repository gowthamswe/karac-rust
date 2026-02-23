
pub mod token;
pub mod lexer;

use crate::lexer::Lexer;
use crate::token::Token;

/// This is the main entry point for the Kāra compiler logic.
/// It takes the source code as input and returns a vector of tokens.
pub fn run_compiler(source: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token();
        let is_eof = token == Token::EOF;
        tokens.push(token);
        if is_eof {
            break;
        }
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*; // Make types from the parent module available
    use crate::token::Token; // Import the Token enum

    #[test]
    fn test_basic_tokenization() {
        let source = r#"
            // Define a new semantic type
            type UserId i64;

            flow PromoteUser(id: UserId) {
                let user_age = 30;
                if user_age >= 18 {
                    // This is a valid user
                }
            }
        "#;

        let tokens = run_compiler(source);

        let expected_tokens = vec![
            // type UserId i64;
            Token::Type,
            Token::Identifier("UserId".to_string()),
            Token::Identifier("i64".to_string()),
            Token::Semicolon,

            // flow PromoteUser(id: UserId) {
            Token::Flow,
            Token::Identifier("PromoteUser".to_string()),
            Token::LeftParen,
            Token::Identifier("id".to_string()),
            Token::Colon,
            Token::Identifier("UserId".to_string()),
            Token::RightParen,
            Token::LeftBrace,

            // let user_age = 30;
            Token::Let,
            Token::Identifier("user_age".to_string()),
            Token::Equal,
            Token::Integer(30),
            Token::Semicolon,

            // if user_age >= 18 {
            Token::If,
            Token::Identifier("user_age".to_string()),
            Token::GreaterThanOrEqual,
            Token::Integer(18),
            Token::LeftBrace,

            // }
            Token::RightBrace,
            // }
            Token::RightBrace,
            Token::EOF,
        ];

        assert_eq!(tokens, expected_tokens, "The token stream did not match the expected output.");
    }
}
