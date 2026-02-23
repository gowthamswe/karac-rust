# Chapter 3: Language Concepts

This chapter explains the core concepts of the Kāra language in a more descriptive way, moving from the formal grammar to practical understanding.

---

## 1. Program Entry Point: The `main` flow

Every Kāra executable program begins execution in a special `flow` named `main`. This is the top-level orchestration layer of your application.

```rust
flow main {
    // High-level logic goes here.
}
```

---

## 2. The Core Constructs: `record`, `fn`, and `flow`

The entire Kāra language is built upon three top-level constructs: `record`, `fn`, and `flow`.

### `record`: The Data Blueprint

Defines the static shape and memory layout of a custom data structure. `record` definitions are always top-level.

### `fn`: The Pure Calculation

Defines a pure, reusable, and stateless data transformation. A `fn` is prohibited by the compiler from having any side-effects (like I/O or printing).

### `flow`: The Impure Process

Orchestrates `fn`s and other `flow`s. A `flow` is the only place you are allowed to manage state (`let`) and perform side-effects.

---

## 3. Syntax: Dataflow (`->`)

Kāra uses a single, consistent syntax for expressing logic: the dataflow operator (`->`). It is used for calling named functions (`fn` or `flow`) and makes the high-level dependencies in your program clear and easy to see.

```rust
// Use `->` to show the flow of data between functions.
(user_id = "user-123") -> get_user_plan -> (plan);
(plan = plan) -> execute_plan -> (http_response);
```

For common arithmetic and logical operations, Kāra supports familiar inline expressions with the `let` keyword. This provides a readable syntax for low-level calculations.

However, `let` in Kāra creates an **immutable binding**, not a mutable variable. Once a name is bound, its value can never be changed.

```rust
// This is a valid binding for a new name, `dist_sq`.
let dist_sq = (dx * dx) + (dy * dy);

// This is a COMPILE-TIME ERROR because `dist_sq` already exists.
let dist_sq = dist_sq * 2.0; // ERROR: `dist_sq` cannot be redeclared.
```

This rule is fundamental. It prevents imperative-style programming and ensures that the underlying dataflow model is preserved.

---

## 4. Working with Purity

The strict separation of pure `fn`s and impure `flow`s is a core feature.

### Debugging

You cannot `Print` from within an `fn`. To debug, modify the `fn` to return the intermediate values you want to inspect, then `Print` them from the `flow`.

```rust
fn my_function(a: i64) -> (i64, i64) {
    let intermediate_value = a * 2;
    let final_result = intermediate_value + 10;
    // return (final_result, intermediate_value);
}

flow main {
    (a = 5) -> my_function -> (the_result, my_debug_val);
    (value = my_debug_val) -> Print;
}
```

### Abstracting Side-Effects: The Plan/Execute Pattern

You cannot perform I/O in an `fn`, but you can write a pure `fn` that creates a `record` describing the desired I/O. The `flow` then executes that plan.

```rust
record HttpRequestPlan {
    url: String,
    method: String
}

fn build_get_user_request(user_id: String) -> HttpRequestPlan {
    // return HttpRequestPlan {
    //     url: "/users/" + user_id,
    //     method: "GET"
    // };
}

flow main {
    (user_id = "user-123") -> build_get_user_request -> (plan);
    (plan = plan) -> execute_request -> (http_response);
}
```

---

## 5. Iteration: Looping via Recursion

Because Kāra enforces immutable bindings (`let`), traditional loop constructs like `for` and `while` do not exist. They are fundamentally based on mutation, which Kāra prohibits.

Instead, all iteration is achieved through **recursive flows**.

To create a loop, you design a `flow` that represents a single step of the iteration. This `flow` then calls itself with the *new state* for the next iteration.

### Example: A Counter

Here is how you would write a counter that prints numbers from 0 to 4:

```rust
// This flow represents one step of the counting process.
flow counter_loop(current_count: i64) {
    // Use the current state.
    (value = current_count) -> Print;

    // Check the termination condition.
    let should_continue = current_count < 4;

    // Use an `if` statement to control the recursion.
    if should_continue {
        // 1. Calculate the *next* state.
        let next_count = current_count + 1;

        // 2. Recurse: Call the flow again, passing the new state.
        (current_count = next_count) -> counter_loop;
    }
}

flow main {
    // Start the loop by calling it with the initial state.
    (current_count = 0) -> counter_loop;
}
```

This pattern makes the flow of state explicit. Instead of mutating a hidden variable, you are clearly passing data from one step of the process to the next.
