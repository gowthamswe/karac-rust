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
