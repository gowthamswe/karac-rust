# Chapter 2: Language Specification

This chapter provides the formal specification of the Kāra language grammar. It is intended for compiler developers and language enthusiasts.

---

## 1. Top-Level Statements

A Kāra source file is composed of a series of top-level statements. These are the only constructs that can appear at the top level of a file.

-   **Record Definition:** `record <RecordName> { <field>: <Type>, ... }`
-   **Function Definition:** `fn <FunctionName>(<param>: <Type>, ...) -> (<return_name>: <Type>, ...) { ... }`
-   **Flow Definition:** `flow <FlowName>(<param>: <Type>, ...) { ... }`

## 2. `record` Definition

Defines a new composite data type.

```rust
// Defines a record with two fields.
record Point {
    x: Number,
    y: Number
}
```

## 3. `fn` (Pure Function) Definition

Defines a stateless, pure data transformation.

```rust
// Defines a function that takes a Point and returns a Number.
fn CalculateDistance(p: Point) -> (dist: Number) {
    // Function body composed of dataflow statements
    // or inline expressions.
}
```

## 4. `flow` (Impure Process) Definition

Defines a stateful process that can contain side-effects.

```rust
// Defines a flow that takes a String and performs an action.
flow PrintMessage(message: String) {
    // Flow body composed of dataflow statements,
    // inline expressions, conditionals, and recursion.
}
```

## 5. Statements within `fn` and `flow`

### Dataflow Statement

Calls a named `fn` or `flow`.

-   **Syntax:** `(<source_vars>) -> <FunctionName> -> (<dest_vars>);`

```rust
(x, y) -> Add -> z;
```

### Let Statement (Immutable Binding)

Binds the result of a simple expression to an immutable name.

-   **Syntax:** `let <name> = <expression>;`

```rust
let a = b * c + d;
```

## 6. Control Flow

Control flow is managed through conditional statements and recursive `flow` calls.

### Conditional (`if`) Statement

The `if` statement executes a block of code if a condition is true. It does not have an `else` clause.

-   **Syntax:** `if <boolean_expression> { ... }`

```rust
let is_valid = x > 10;
if is_valid {
    (value = "Valid") -> Print;
}
```

### Iteration (Looping)

Loops are not a built-in syntax but are implemented via **recursive `flow` calls**. A `flow` calls itself to continue an iteration, passing new state as parameters.

```rust
flow Countdown(n: Number) {
    (value = n) -> Print;

    let should_continue = n > 0;
    if should_continue {
        let next_n = n - 1;
        (n = next_n) -> Countdown; // Recursion performs the loop.
    }
}
```