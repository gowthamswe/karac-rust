use karac::run_compiler;

fn main() {
    // This source code is a placeholder.
    // In the future, this will be read from a file specified by the user.
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

    // Call the library's main entry point
    run_compiler(source);
}
