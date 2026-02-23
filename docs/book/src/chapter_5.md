# Chapter 5: Defining Behavior with `fn` and `flow`

At the heart of a Kāra program are `fn` and `flow` blocks. They contain the logic that transforms data and orchestrates high-level processes.

---

## Block Expressions: The Foundation of `fn` and `flow`

Both `fn` and `flow` definitions are followed by a `blockExpression`. A block is a sequence of statements surrounded by curly braces `{}`. Crucially, a block is also an expression, meaning it evaluates to a value.

The value of a block is the value of the **last expression** in the block. If the last expression is omitted (or if the last statement ends with a semicolon), the block's value is `()` (unit), an empty tuple.

This is identical to Rust's block and return-value semantics.

```rust
// This block evaluates to the integer 42
let x = {
    let y = 20;
    y + 22 // No semicolon, so this is the block's return value
};

// This block evaluates to () because the last statement ends in a semicolon.
let z = {
    let y = 20;
    y + 22;
};
```

---

## `fn`: Pure Data Transformation

A `fn` is a **pure function**. This is a strict contract with the compiler: it can only take data, transform it, and return a new value. It cannot have side effects like printing to the console, reading files, or calling a `flow`.

The body of a `fn` is a `blockExpression`, and its return value is determined by the block's value.

### Single Value Return

Here is a function that takes two `f64` values and returns their sum.

```rust
fn add(a: f64, b: f64) -> f64 {
    a + b // The sum is the final expression, so it becomes the return value
}
```

### Record Construction and Return

You can use a `recordExpression` to construct and return a new record.

```rust
record Point { x: f64, y: f64 }

fn create_origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}
```

### Tuple Return for Multiple Values

To return multiple values from a function, you can use a `tupleExpression`. The return type must also be declared as a tuple.

```rust
fn swap(a: i64, b: i64) -> (i64, i64) {
    (b, a) // Return a tuple with the values swapped
}
```

---

## `flow`: Impure Orchestration

A `flow` is the primary tool for high-level orchestration. Unlike a `fn`, a `flow` is **impure**. It is designed to manage side effects, coordinate other `fn` and `flow` calls, and define the overall behavior of your application.

A `flow`'s body is also a `blockExpression`, but it does not have a declared return type. Its primary job is to execute a sequence of statements, typically `pipelineStatement`s.

```rust
flow process_data {
    // A pipeline is a statement. It ends with a semicolon and evaluates to ().
    (source = "raw_data.csv") -> read_file -> (raw_bytes);

    // The flow continues, orchestrating the next step.
    (bytes = raw_bytes) -> parse_csv -> (parsed_records);
    
    // ... and so on.
}
```
