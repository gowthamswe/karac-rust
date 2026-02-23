# Chapter 4: Runtime, I/O, and Error Handling

This chapter discusses the practical aspects of how a Kāra program interacts with the outside world, including handling user input, output, and managing errors gracefully.

---

## 1. The Kāra Runtime and Built-in Functions

The Kāra runtime provides a set of special, globally available functions that are essential for I/O and basic data manipulation. You do not need to import them. These are part of the standard library.

Key built-in functions include:

*   **`Print`**: Prints a `String` value to the standard output.
*   **`Read`**: Reads a line of text from the standard input.
*   **`ToString`**: Converts a value of any type into its `String` representation.
*   **`ParseI64`**: Attempts to parse a `String` into an `i64`.

**Example:**

```rust
flow main {
    (value = "Please enter your name:") -> Print;

    () -> Read -> (user_name);

    (value = "Hello, " + user_name) -> Print;
}
```

---

## 2. Error Handling

In a data-flow oriented language, not all operations will succeed. A file might not exist, or a string might not be a valid number. Kāra handles this using a concept inspired by Rust's `Result` and `Option` types, combined with a `match` statement.

Many built-in functions or language operations can produce a "result" type, which can be either a success value or an error.

The `match` statement is used to safely unwrap this result and handle all possible outcomes.

### `match` with `with`

The `match` statement checks a variable. The `with` clauses define the patterns to match against.

**Example: Handling a potential parsing error.**

```rust
flow main {
    (value = "Enter a number:") -> Print;
    () -> Read -> (text_input);
    
    // ParseI64 might succeed or fail.
    (value = text_input) -> ParseI64 -> (number_result);

    // We use 'match' to handle both possibilities.
    match number_result {
        with Ok(num) {
            // This block runs on success.
            // 'num' contains the successfully parsed number.
            let product = num * 2;
            (value = product) -> ToString -> (product_str);
            (value = "The result is: " + product_str) -> Print;
        }
        with Err(error_message) {
            // This block runs on failure.
            // 'error_message' contains a string explaining the error.
            (value = "Error: " + error_message) -> Print;
        }
    }
}
```

This `match` construct forces the programmer to handle both success and failure cases, leading to more robust and reliable programs. It is a compile-time error to not handle all possible variants of a result.

---

### EBNF Grammar for `match` Statement

*(This grammar will be added to Chapter 2)*

```ebnf
statement = matchStatement | ... ;

matchStatement = 'match' IDENTIFIER '{' { withClause } '}' ;

withClause = 'with' pattern '{' { statement } '}' ;

pattern = IDENTIFIER '(' IDENTIFIER ')' | IDENTIFIER ;
```
