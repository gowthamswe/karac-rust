use karac::run_compiler;
use karac::token::Token;

#[test]
fn test_lexer_updated() {
    let source = r#"
        let x = 5;
        let y = 10.5;
        let z = "hello world";

        flow my_flow(a: i64, b: f64) {
            let result = 1;
        }
    "#;

    let tokens = run_compiler(source);

    let expected_tokens = vec![
        Token::Let,
        Token::Identifier("x".to_string()),
        Token::Equal,
        Token::Integer(5),
        Token::Semicolon,

        Token::Let,
        Token::Identifier("y".to_string()),
        Token::Equal,
        Token::Float(10.5),
        Token::Semicolon,

        Token::Let,
        Token::Identifier("z".to_string()),
        Token::Equal,
        Token::StringLiteral("hello world".to_string()),
        Token::Semicolon,

        Token::Flow,
        Token::Identifier("my_flow".to_string()),
        Token::LeftParen,
        Token::Identifier("a".to_string()),
        Token::Colon,
        Token::Identifier("i64".to_string()),
        Token::Comma,
        Token::Identifier("b".to_string()),
        Token::Colon,
        Token::Identifier("f64".to_string()),
        Token::RightParen,
        Token::LeftBrace,

        Token::Let,
        Token::Identifier("result".to_string()),
        Token::Equal,
        Token::Integer(1),
        Token::Semicolon,
        
        Token::RightBrace,
        Token::EOF,
    ];

    assert_eq!(tokens, expected_tokens);
}
