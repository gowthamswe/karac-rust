# Language Gaps and Design Evolution

This document summarizes the design gaps discovered while writing the initial `word_count.kara` example. It serves as a concrete set of action items for evolving the language specification.

## 1. The Need for Error Handling: `Result` and `Option`

Attempting to write a simple file-reading utility immediately revealed the lack of any mechanism to handle operations that can fail or return no value.

-   **Scenario 1 (Failure):** `read_file` cannot be guaranteed to succeed. It must be able to return an error (e.g., "file not found").
-   **Scenario 2 (Absence):** Accessing a command-line argument by index might yield no value.

### Design Decision:

Kāra will adopt first-class support for **sum types** (also known as tagged unions) to solve this. Specifically, the standard library will provide two fundamental generic enums:

-   `enum Result<T, E> { Ok(T), Error(E) }`: For operations that can succeed with a value of type `T` or fail with a value of type `E`.
-   `enum Option<T> { Some(T), None }`: For values that may be present (`Some`) or absent (`None`).

This approach is chosen over using `null` values for the following critical reason:

**The Problem with `null`:** `null` undermines the type system. If a variable of type `String` can also be `null`, the compiler cannot protect the programmer from accidentally using the `null` value, leading to runtime crashes. The burden of checking for `null` falls entirely on the programmer, which has proven to be a major source of bugs in many languages.

**The Kāra Solution:** By encoding success/failure or presence/absence directly into the type (`Result` and `Option`), the compiler can enforce that all possible cases are handled. This eliminates an entire class of runtime errors.

## 2. The Need for Pattern Matching: `match` Expressions

Having `Result` and `Option` is not enough. We need a safe and ergonomic way to access the values they contain. The placeholder syntax `let value = result as Ok;` used in the example is unsafe and clumsy.

### Design Decision:

Kāra will implement a `match` expression. This construct will be the primary way to interact with `enum` types.

```rust
// Example of the proposed `match` syntax
match file_content_result {
    Ok(content) => {
        // In this branch, `content` is a guaranteed-safe `String`.
        // The compiler has proven this for us.
        (content) -> process_text -> (word_count);
        (word_count) -> print;
    },
    Error(e) => {
        // In this branch, `e` is the bound error value.
        ("Error: ") -> print;
        (e) -> print;
    }
}
```

The `match` expression provides two key benefits:

1.  **Exhaustiveness Checking:** The compiler will ensure that all variants of the `enum` are handled. Forgetting to handle the `Error` case will be a **compile-time error**, not a runtime bug.
2.  **Safe Value Unwrapping:** It safely extracts and binds the inner value of a variant to a new variable (`content` or `e` in the example) that is only valid within that specific branch.

## 3. The Need for Generics

The `word_count` example immediately showed the need for functions that can operate on lists of any type (e.g., `count_items<T>(items: List<T>)`).

### Design Decision:

Kāra will support lightweight, C++/Rust-style generics using angle brackets (`<T>`). This allows for the definition of generic functions and data structures (like `List<T>`, `Option<T>`, and `Result<T, E>`).

## 4. The Need for a Standard Library Core

The example forced us to assume the existence of many basic functions and types.

### Design Decision:

A minimal standard library must be defined. The initial set of required components includes:

-   **Types:** `List<T>`, `String`, `Result<T, E>`, `Option<T>`.
-   **Pure Functions (`fn`):**
    -   `string::split(String, String) -> List<String>`
    -   `list::length<T>(List<T>) -> i64`
    -   `list::get<T>(List<T>, i64) -> Option<T>`
-   **Impure Processes (`flow`):**
    -   `print(String)`
    -   `read_file(String) -> Result<String, Error>`
    -   `exit(i64)`

## 5. Key-Value Data Structures: `record`

The discussion clarified that Kāra's primary key-value data structure is the `record`.

### Design Decision:

A `record` is a "static dictionary" where the keys and value types are known at compile time. This is the idiomatic way to structure data. The language will not have a dynamic, runtime-modifiable hash map in its core, favoring the safety and predictability of static `record` types.
