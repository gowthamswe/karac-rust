# Kāra

**Kāra is an experimental, intent-driven programming language where data carries its own semantic context.**

Inspired by the Sanskrit language — where every word carries embedded grammatical context (vibhakti) so that meaning is unambiguous regardless of word order — Kāra makes semantic context a first-class part of the type system. The compiler knows not just what your data *is*, but what it *means*, catching an entire class of logical bugs that no mainstream language prevents.

---

## The Core Idea

In most languages, an `i64` is just a number. Whether it's a user ID, a product count, or a temperature is known only to the programmer. The compiler sees no difference and will happily let you pass one where the other is expected.

In Kāra, data that crosses a boundary — function parameters, return values, record fields — carries semantic context that the compiler enforces:

```rust
type UserId i64;
type ProductId i64;

// The compiler treats UserId and ProductId as distinct types.
// Passing a UserId where a ProductId is expected is a compile error,
// even though both are i64 underneath.
fn lookup_product(user: UserId, product: ProductId) -> PriceInCents {
    let base = get_base_price(product);
    let discount = base * discount_rate;
    base - discount
}
```

Inside function bodies, local computation is context-free — no annotation required. Context is mandatory at boundaries, inferred locally. This is analogous to how Rust infers most lifetimes and only asks for annotations when it can't figure things out.

## Key Concepts

-   **Sanskrit-Inspired:** Data carries semantic context, the way Sanskrit words carry grammatical role. Meaning is unambiguous at every boundary.
-   **Context at Boundaries:** Semantic types are enforced at function/flow boundaries. Local computation is annotation-free.
-   **Intent-Driven:** You describe the "what" (the dataflow), and the compiler figures out the "how" (the execution order).
-   **Immutable by Default:** All data is immutable, eliminating entire classes of bugs related to state management.
-   **Pure Functions (`fn`):** Reusable, stateless building blocks for pure calculation.
-   **Impure Processes (`flow`):** The orchestration layer for managing state, I/O, and side-effects.
-   **Recursive Iteration:** All loops are expressed as recursive `flow` calls, preserving immutability.

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

The compiler is in early development. The lexer is complete; the parser and type checker are next.

We are taking a **tree-walk interpreter first** approach: validate language semantics with an interpreter before building LLVM code generation. See the [Roadmap](ROADMAP.md) for details.

-   **For the language design, read the [Kāra Book](docs/book/src/chapter_1.md).**
-   **For design decisions, see the [Design Rationale](docs/DESIGN_RATIONALE.md).**
-   **For the compilation strategy, see [Project Sutra](docs/book/src/00_compiler_architecture_decision.md).**

## Getting Started

This project is currently in the compiler development phase. To explore the language design, refer to the documentation in the `/docs` directory.
