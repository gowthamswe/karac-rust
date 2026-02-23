// src/token.rs

//! Defines the tokens that are produced by the lexer.

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    Fn,
    Flow,
    Record,
    Type,
    Let,
    If,
    True,
    False,

    // Single-character Symbols
    LeftParen,  // (
    RightParen, // )
    LeftBrace,  // {
    RightBrace, // }
    Colon,      // :
    Comma,      // ,
    Semicolon,  // ;
    Plus,       // +

    // One or two character Symbols
    Bang,               // !
    BangEqual,          // !=
    Equal,              // =
    // TODO: Add EqualEqual ==
    GreaterThan,        // >
    GreaterThanOrEqual, // >=
    LessThan,           // <
    LessThanOrEqual,    // <=
    Arrow,              // ->

    // Literals
    Identifier(String),
    Integer(i64),
    Float(f64),
    StringLiteral(String),

    // Special Tokens
    Error,
    EOF,
}
