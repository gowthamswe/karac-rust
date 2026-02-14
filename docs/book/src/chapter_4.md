# Chapter 4: Runtime, I/O, and Error Handling

This chapter discusses the practical aspects of how a Kāra program interacts with the outside world, including handling user input, output, and managing errors gracefully.

---

## 1. The Kāra Runtime and Built-in Functions

The Kāra runtime provides a set of special, globally available functions (`fn`) that are essential for I/O and basic data manipulation. You do not need to import them. These are part of the standard library.

Key built-in functions include:

*   **`Print`**: Prints a `String` value to the standard output.
*   **`Read`**: Reads a line of text from the standard input.
*   **`ToString`**: Converts a value of any type into its `String` representation.
*   **`ParseNumber`**: Attempts to parse a `String` into a `Number`.

**Example:**

```rust
flow main {
    Action: Print From: value = "Please enter your name:";

    Action: Read Into: user_name;

    Action: Print From: value = "Hello, " + user_name;
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
    Action: Print From: value = "Enter a number:";
    Action: Read Into: text_input;
    
    // ParseNumber might succeed or fail.
    Action: ParseNumber From: value = text_input Into: number_result;

    // We use 'match' to handle both possibilities.
    match number_result {
        with Ok(num) {
            // This block runs on success.
            // 'num' contains the successfully parsed number.
            let product = num * 2;
            Action: Print From: value = "The result is: " + ToString(product);
        }
        with Err(error_message) {
            // This block runs on failure.
            // 'error_message' contains a string explaining the error.
            Action: Print From: value = "Error: " + error_message;
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
