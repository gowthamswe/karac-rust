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
