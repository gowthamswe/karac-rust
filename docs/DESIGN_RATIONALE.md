# Kāra Design Rationale

This document records the key architectural and syntactical decisions for the Kāra language. It serves as a reference for *why* the language is designed the way it is.

## 1. The Core Problem: Expressing Both "What" and "How"

A systems language needs to be good at two things: high-level orchestration (the "what") and low-level implementation (the "how"). Kāra's design challenge was to solve both of these within a single, consistent philosophical model.

- **The "What" (Orchestration):** Describing the high-level flow of a program. This should be as readable as possible.
- **The "How" (Implementation):** Writing the complex, granular logic for algorithms. This needs to be powerful and expressive.

### The Problem with a Single Syntax

The purely verbose `Action: From: Into:` syntax, while excellent for readability in high-level orchestration, becomes unwieldy for complex calculations. For example, `(a * b) + (c * d)` would be:

```
// This is too verbose for implementation.
Action: Multiply
  From: a
  From: b
  Into: product_ab

Action: Multiply
  From: c
  From: d
  Into: product_cd

Action: Add
  From: product_ab
  From: product_cd
  Into: final_result
```

This is not a scalable way to write algorithms.

## 2. The Solution: A Multi-Layered, Consistent Syntax

Kāra solves this by providing different syntactical "layers" for orchestration and implementation, but both layers are built on the exact same core principle: **Source -> Action -> Destination**.

### Component 1: `Record` - Defining the Participants

- **Purpose:** To define custom data structures, the *Kārakas* (the participants in an action).
- **Syntax:**
  ```rust
  Record Point {
    x: i64,
    y: i64,
  }
  ```

### Component 2: `Sūtra` - Defining a Reusable Logic Thread

- **Purpose:** To define a reusable, named "thread of logic." This is the equivalent of a function.
- **Why `Sūtra`?** "Sūtra" translates to "thread," perfectly capturing the idea of guiding data through a series of actions.
- **Syntax:** The signature is verbose, defining a clear "contract."
  ```rust
  Define Sūtra: CalculateDistance
    Require From: p1 as Point
    Require From: p2 as Point
    Return: distance as f64
  ```

### Component 3: The Pipe Operator `->` - The High-Density Sūtra Body

- **Purpose:** To define the implementation logic inside a `Sūtra` using a compact, graph-like syntax.
- **Why `->`?** It's a compact version of `From -> Action -> Into`. It's conceptually a "pipe," a familiar concept for developers.
- **Syntax:**
  ```rust
  // Inside the CalculateDistance Sūtra...
  {
    // (source1, source2) -> Action -> destination
    (p1.x, p2.x) -> Subtract -> dx
    (p1.y, p2.y) -> Subtract -> dy
    (dx, dx) -> Multiply -> dx_squared
    (dy, dy) -> Multiply -> dy_squared
    (dx_squared, dy_squared) -> Add -> sum_squares
    Return: (sum_squares) -> Sqrt
  }
  ```

### Component 4: `flow` - High-Level Orchestration

- **Purpose:** The main entry point of the program. It uses the verbose, highly-readable `Action: From: Into:` syntax to describe the program's overall story.
- **Type Inference:** While `Sūtra` and `Record` definitions require explicit types (the "contract"), the `flow` block uses type inference. For example, the type of `final_distance` is automatically inferred from the `Return` signature of the `CalculateDistance` `Sūtra`, keeping the orchestration layer clean and readable.
- **Syntax:**
  ```rust
  flow main {
    let origin = Point { x: 0, y: 0 }
    let target = Point { x: 10, y: 20 }

    // Invoke the Sūtra using the clean, self-documenting syntax.
    Action: CalculateDistance
      From: p1 = origin
      From: p2 = target
      Into: final_distance

    Action: Print
      From: "The final distance is:"
      From: final_distance
  }
  ```

## 3. A Complete Example

This is what a complete `.kara` file looks like, demonstrating how the components work in harmony.

```rust
// 1. Define the data structures.
Record Point {
  x: i64,
  y: i64,
}

// 2. Define a reusable logic thread (the 'how').
Define Sūtra: CalculateDistance
  Require From: p1 as Point
  Require From: p2 as Point
  Return: distance as f64
{
  // Use the compact pipe operator for the implementation graph.
  (p1.x, p2.x) -> Subtract -> dx
  (p1.y, p2.y) -> Subtract -> dy
  (dx, dx) -> Multiply -> dx_squared
  (dy, dy) -> Multiply -> dy_squared
  (dx_squared, dy_squared) -> Add -> sum_squares
  Return: (sum_squares) -> Sqrt
}

// 3. Orchestrate the high-level program flow (the 'what').
flow main {
  let origin = Point { x: 0, y: 0 }
  let target = Point { x: 10, y: 20 }

  Action: CalculateDistance
    From: p1 = origin
    From: p2 = target
    Into: final_distance

  Action: Print
    From: "The calculated distance is:"
    From: final_distance
}
```
