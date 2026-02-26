# Kāra Design Rationale

This document records the key architectural and syntactical decisions for the Kāra language. It serves as a reference for *why* the language is designed the way it is.

## 1. The Origin: Inspiration from Sanskrit

Kāra's design is inspired by the Sanskrit language. In Sanskrit, every noun carries embedded context through its morphological inflection (vibhakti). The case endings tell you what role a word plays in a sentence — subject, object, instrument, location — regardless of where the word appears. Word order is free because the meaning is encoded in the word itself.

Most programming languages work the opposite way: data is raw and context-free. An `i64` is just a number. Whether it represents a user ID, a product count, or a temperature is known only to the programmer. The compiler sees no difference between them and will happily let you add a user ID to a temperature. This is a class of logical bug that no mainstream language prevents.

Kāra asks: **what if data carried its own semantic context, the way Sanskrit words carry their own grammatical role?** If the compiler always knows what a piece of data *means* — not just what it *is* — it can prevent an entire category of errors that other languages cannot catch.

The compiler project is internally codenamed **Project Sutra** (from the Sanskrit सूत्र, meaning "thread" or "rule").

## 2. The Core Principle: Context at Boundaries

Not all data needs explicit context. A loop counter, an intermediate arithmetic result, or a local scratch variable — these are like Sanskrit's indeclinable particles (avyaya). Their role is obvious from usage. Forcing the programmer to annotate every local variable with semantic meaning would be busywork.

But data that **crosses a boundary** — function parameters, return values, record fields, I/O — is where semantic errors happen. This is where a `UserId` gets confused with a `ProductId`, where a price in cents gets treated as a price in dollars.

Kāra's rule is: **context is mandatory at boundaries, inferred locally.**

```
// At the boundary: parameters and return type carry semantic context.
// UserId and ProductId are both i64 underneath, but the compiler
// treats them as distinct types. Mixing them is a compile error.
fn lookup_product(user: UserId, product: ProductId) -> PriceInCents {
    // Inside the body: local computation is context-free.
    // These are just numbers. No annotation required.
    let base = get_base_price(product);
    let discount = base * discount_rate;
    let final_price = base - discount;

    // At the return boundary: the compiler checks that final_price
    // can satisfy the declared return type PriceInCents.
    final_price
}
```

This gives us:

- **Enforcement where it matters:** Boundaries are where semantic bugs happen.
- **No busywork:** Local variables, counters, and intermediates need no annotation.
- **Gradual adoption:** Start with a few semantic types and add more as your domain model matures.

This is analogous to Rust's approach to lifetimes: the compiler infers most lifetimes automatically and only asks the programmer to annotate when it can't figure things out. Kāra does the same for semantic context.

## 3. Semantic Types: How Context Works

A semantic type is defined with the `type` keyword. It creates a new, distinct type backed by a primitive:

```
type UserId i64;
type ProductId i64;
type PriceInCents i64;
type Temperature f64;
```

These are all primitives underneath, but the compiler treats them as incompatible types at every boundary. You cannot pass a `UserId` where a `ProductId` is expected.

### Context Conversion

When raw data needs to gain semantic context (e.g., parsing user input into a `UserId`), this must happen through an explicit conversion — a function that validates and transforms:

```
fn parse_user_id(raw: i64) -> Result<UserId, ValidationError> {
    if raw > 0 {
        Ok(raw as UserId)
    } else {
        Err(ValidationError { message: "User ID must be positive" })
    }
}
```

Explicit conversion functions are the **only** way to cross context boundaries. This makes every boundary crossing visible, auditable, and testable. The `as` keyword marks the point where raw data gains semantic meaning.

## 4. The Core Philosophy: An Intent-Driven Language

Kāra is an **intent-driven language**. Unlike traditional imperative languages where the programmer provides a step-by-step sequence of commands, a Kāra programmer **declares their intent** by describing the flow of data and the transformations upon it.

The programmer describes the relationships between data and operations, not the order in which they must be executed. This allows the Kāra compiler to build a data dependency graph and optimize execution in ways that are impossible in traditional languages, including automatic parallelism, out-of-order execution, and lazy evaluation.

## 5. The Core Problem: Separating "What" from "How"

A systems language needs to express both high-level orchestration (the "what") and low-level implementation (the "how"). Kāra solves this by providing different constructs for each, forcing a separation of concerns that is critical for both readability and compiler optimization.

-   **The "What" (Process):** Describing the high-level story of a program, including state, I/O, and side-effects. This is the job of a `flow`.
-   **The "How" (Calculation):** Defining a pure, reusable, and complex algorithm. This is the job of an `fn`.

## 6. The Core Constructs: `record`, `fn`, and `flow`

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

## 7. Syntax: Explicitness vs. Ergonomics

Kāra's syntax is designed to balance two competing goals: giving the compiler an explicit dataflow graph for optimization, while providing the developer with an ergonomic and familiar coding experience.

### The Dataflow Operator (`->`)

The `->` operator is used for calling any named function (`fn` or `flow`). This syntax makes the high-level data dependencies in a program explicit and easy to visualize.

```
// Calling a pure function to build a data structure.
("user-123") -> BuildGetUserRequest -> (request_plan);

// Calling an impure flow to execute the plan.
(plan = request_plan) -> DoHttpRequest -> (http_response);
```

### Inline Expressions for Readability

For common arithmetic and logical operations, forcing every step into a separate `->` call is overly verbose. Therefore, Kāra supports familiar **inline expressions**.

```
// This readable expression...
let dist_sq = (dx * dx) + (dy * dy);

// ...is treated by the compiler as "syntactic sugar" for the explicit dataflow graph.
// The developer gets readability, and the compiler gets the optimization map.
```

This dual approach provides the best of both worlds: high-level dataflow is made explicit with `->`, while common low-level calculations remain compact and readable.

## 8. State: Immutable by Default

The `let` keyword is used to bind a name to the result of an expression. This binding is **immutable**. Once a name is bound, it cannot be reassigned.

```
// This is a valid binding.
let a = 10;

// This is a COMPILE-TIME ERROR.
let a = a + 1;
```

This rule is the key to the entire hybrid syntax. It prevents imperative-style mutation and ensures that even when using the ergonomic `=` syntax, the underlying model remains one of dataflow. A variable is a name for the result of a calculation, not a mutable bucket.

This strict immutability is what allows the compiler to safely build its dataflow graph without ambiguity.

## 9. The Purity Guarantee and Its Consequences

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
