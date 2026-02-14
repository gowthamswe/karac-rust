use karac::run_compiler;
use karac::token::Token;

#[test]
fn test_lexer() {
    let source = r#"
        let x = 5;
        let y = 10.5;
        let add = fn(a, b) -> {
            a + b;
        };
        let result = add(x, y);
        "hello world"
    "#;

    let tokens = run_compiler(source);

    let expected_tokens = vec![
        Token::Let,
        Token::Identifier("x".to_string()),
        Token::Equal,
        Token::Number(5.0),
        Token::Semicolon,
        Token::Let,
        Token::Identifier("y".to_string()),
        Token::Equal,
        Token::Number(10.5),
        Token::Semicolon,
        Token::Let,
        Token::Identifier("add".to_string()),
        Token::Equal,
        Token::Fn,
        Token::LeftParen,
        Token::Identifier("a".to_string()),
        Token::Comma,
        Token::Identifier("b".to_string()),
        Token::RightParen,
        Token::Arrow,
        Token::LeftBrace,
        Token::Identifier("a".to_string()),
        Token::Plus,
        Token::Identifier("b".to_string()),
        Token::Semicolon,
        Token::RightBrace,
        Token::Semicolon,
        Token::Let,
        Token::Identifier("result".to_string()),
        Token::Equal,
        Token::Identifier("add".to_string()),
        Token::LeftParen,
        Token::Identifier("x".to_string()),
        Token::Comma,
        Token::Identifier("y".to_string()),
        Token::RightParen,
        Token::Semicolon,
        Token::String("hello world".to_string()),
        Token::EOF,
    ];

    assert_eq!(tokens, expected_tokens);
}
