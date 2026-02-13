mod lexer;

use lexer::{Lexer, Token};

fn main() {
    let source = r#"
Record Point {
    x: i64,
    y: i64,
}

// A comment to be ignored

flow main {
    let p1 = Point { x: 10, y: 20 };
    
    // This is the verbose way
    Action: PrintPoint
        From: p = p1;

    // This is the dense way
    p1 -> PrintPoint -> ();
}
"#;

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
