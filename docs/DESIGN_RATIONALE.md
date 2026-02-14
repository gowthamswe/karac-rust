# Kāra Design Rationale

This document records the key architectural and syntactical decisions for the Kāra language. It serves as a reference for *why* the language is designed the way it is.

## 1. The Core Philosophy: An Intent-Driven Language

Kāra is an **intent-driven language**. Unlike traditional imperative languages where the programmer provides a step-by-step sequence of commands, a Kāra programmer **declares their intent** by describing the flow of data and the transformations upon it.

The programmer describes the relationships between data and operations, not the order in which they must be executed. This allows the Kāra compiler to build a data dependency graph and optimize execution in ways that are impossible in traditional languages, including automatic parallelism, out-of-order execution, and lazy evaluation.

## 2. The Core Problem: Separating "What" from "How"

A systems language needs to express both high-level orchestration (the "what") and low-level implementation (the "how"). Kāra solves this by providing different constructs for each, forcing a separation of concerns that is critical for both readability and compiler optimization.

-   **The "What" (Process):** Describing the high-level story of a program, including state, I/O, and side-effects. This is the job of a `flow`.
-   **The "How" (Calculation):** Defining a pure, reusable, and complex algorithm. This is the job of an `fn`.

## 3. The Core Constructs: `record`, `fn`, and `flow`

Kāra's structure is built on three, and only three, top-level constructs. They cannot be nested.

### `record`: The Data Blueprint

-   **Purpose:** To define the static shape and memory layout of a data structure.
-   **Rule:** `record` definitions are strictly **top-level**.
-   **Rationale:** This allows the compiler to know the blueprint of every possible data structure before analyzing any executable code, enabling guaranteed memory layouts.

### `fn`: The Pure Calculation

-   **Purpose:** To define a pure, stateless data transformation.
-   **The Promise:** An `fn` is guaranteed to be **pure**. The output depends *only* on the input, and it has no side-effects.
-   **The Reward:** In exchange, the compiler can perform massive optimizations like automatic parallelism and memoization.

### `flow`: The Impure Process

-   **Purpose:** To orchestrate `fn`s and `flow`s, managing state (`let`) and side-effects.
-   **The Promise:** A `flow` is **impure**. The compiler knows the sequence of operations can matter and that it interacts with the outside world.
-   **The Consequence:** The compiler is more conservative in its optimizations.

## 4. Syntax: Explicitness vs. Ergonomics

Kāra's syntax is designed to balance two competing goals: giving the compiler an explicit dataflow graph for optimization, while providing the developer with an ergonomic and familiar coding experience.

### The Dataflow Operator (`->`)

The `->` operator is used for calling any named function (`fn` or `flow`). This syntax makes the high-level data dependencies in a program explicit and easy to visualize.

```rust
// Calling a pure function to build a data structure.
("user-123") -> BuildGetUserRequest -> (request_plan);

// Calling an impure flow to execute the plan.
(plan = request_plan) -> DoHttpRequest -> (http_response);
```

### Inline Expressions for Readability

For common arithmetic and logical operations, forcing every step into a separate `->` call is overly verbose. Therefore, Kāra supports familiar **inline expressions**.

```rust
// This readable expression...
let dist_sq = (dx * dx) + (dy * dy);

// ...is treated by the compiler as "syntactic sugar" for the explicit dataflow graph.
// The developer gets readability, and the compiler gets the optimization map.
```

This dual approach provides the best of both worlds: high-level dataflow is made explicit with `->`, while common low-level calculations remain compact and readable.

## 5. State: Immutable by Default

The `let` keyword is used to bind a name to the result of an expression. This binding is **immutable**. Once a name is bound, it cannot be reassigned.

```rust
// This is a valid binding.
let a = 10;

// This is a COMPILE-TIME ERROR.
let a = a + 1;
```

This rule is the key to the entire hybrid syntax. It prevents imperative-style mutation and ensures that even when using the ergonomic `=` syntax, the underlying model remains one of dataflow. A variable is a name for the result of a calculation, not a mutable bucket.

This strict immutability is what allows the compiler to safely build its dataflow graph without ambiguity.

## 6. The Purity Guarantee and Its Consequences

The separation between `fn` and `flow` is a hard, compile-time guarantee.

### Functional Purity (`fn`)

A `fn` is guaranteed by the compiler to be **pure**. This means:

1.  **Referential Transparency:** Same inputs, same output. Always.
2.  **No Side-Effects:** No I/O, clocks, printing, or random numbers.

### The "Plan/Execute" Pattern

To manage side-effects, Kāra enforces the **Plan/Execute Pattern**:

1.  **Plan (in `fn`):** A pure `fn` creates a `record` that describes the desired action.
2.  **Execute (in `flow`):** The `flow` takes the plan and passes it to a built-in, impure function that performs the action.

This cleanly separates pure business logic from impure side-effects.