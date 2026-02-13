pub mod token;
pub mod lexer;

use crate::lexer::Lexer;
use crate::token::Token;

/// This is the main entry point for the Kāra compiler logic.
/// It takes the source code as input and will eventually return a result
/// indicating success or failure. For now, it just tokenizes and prints.
pub fn run_compiler(source: &str) {
    let mut lexer = Lexer::new(source.to_string());

    println!("--- Lexer Output ---");
    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if token == Token::EOF {
            break;
        }
    }
    println!("--- End Lexer Output ---");
}
