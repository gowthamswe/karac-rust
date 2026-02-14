# Kāra

**Kāra is an experimental, intent-driven programming language for building robust and high-performance systems.**

Kāra combines the performance of a compiled systems language with the high-level clarity of a dataflow paradigm. Instead of writing a sequence of commands, you declare the flow of data and the transformations upon it, allowing the compiler to generate a highly optimized, parallelized execution graph.

---

## Key Concepts

-   **Intent-Driven:** You describe the "what" (the dataflow), and the compiler figures out the "how" (the execution order).
-   **Explicit Dataflow:** The `->` operator makes the high-level architecture of your program visible and unambiguous.
-   **Immutable by Default:** All data is immutable, eliminating entire classes of bugs related to state management.
-   **Pure Functions (`fn`):** Reusable, stateless building blocks for pure calculation.
-   **Impure Processes (`flow`):** The orchestration layer for managing state, I/O, and side-effects.
-   **Recursive Loops:** Iteration is handled elegantly and explicitly through recursive `flow` calls.

## Quick Example

This example counts from 0 to 4, printing each number. It showcases the hybrid syntax: the `->` operator for high-level calls (`Print`, `CounterLoop`) and the familiar `=` for low-level calculations.

```rust
// A recursive flow for iteration.
flow CounterLoop(current_count: Number) {
    // Print the current state.
    (value = current_count) -> Print;

    // Check the termination condition.
    let should_continue = current_count < 4;

    if should_continue {
        // Calculate the next state using an inline expression.
        let next_count = current_count + 1;

        // Recurse with the new state.
        (current_count = next_count) -> CounterLoop;
    }
}

// The main entry point.
flow main {
    // Start the loop with the initial state.
    (current_count = 0) -> CounterLoop;
}
```

## Project Status

The language design is largely solidified. We are now beginning the implementation of the compiler.

-   **For a detailed look at the language design, please read the [Kāra Book](docs/book/src/introduction.md).**
-   **To see the latest design decisions, please see the [CHANGELOG.md](CHANGELOG.md).**

## Getting Started

This project is currently in the compiler development phase. To explore the language design, please refer to the documentation in the `/docs` directory.
