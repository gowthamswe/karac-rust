// src/token.rs

//! Defines the tokens that are produced by the lexer.

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    Fn,
    Flow,
    Record,
    Let,
    If,

    // Symbols
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    Arrow,        // ->
    Equal,        // =
    Colon,        // :
    Comma,        // ,
    Semicolon,    // ;
    Plus,         // +

    // Literals
    Identifier(String),
    Number(f64),
    String(String),

    // End of File
    EOF,
}
