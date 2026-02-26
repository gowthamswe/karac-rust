# Chapter 1: Introduction

Welcome to Kāra, a programming language where data carries its own meaning.

## The Inspiration: Sanskrit

Kāra is inspired by the Sanskrit language. In Sanskrit, every noun carries embedded context through its morphological inflection (vibhakti). The case endings tell you what role a word plays — subject, object, instrument, location — regardless of where the word appears in the sentence. You can reorder words freely because meaning is encoded in the word itself.

Most programming languages work the opposite way. An `i64` is just a number. Whether it represents a user ID, a product count, or a temperature is known only to the programmer. The compiler sees no difference between them and will let you pass one where the other is expected without complaint.

Kāra asks: **what if data carried its own semantic context, the way Sanskrit words carry their own grammatical role?**

## Context at Boundaries

Not all data needs annotation. A loop counter, an intermediate arithmetic result, or a temporary variable — these are obvious from context. Forcing the programmer to annotate everything would be busywork.

But data that **crosses a boundary** — function parameters, return values, record fields — is where semantic errors happen. A `UserId` gets confused with a `ProductId`. A price in cents gets treated as a price in dollars.

Kāra's rule: **context is mandatory at boundaries, inferred locally.**

```rust
type UserId i64;
type ProductId i64;

// At the boundary: UserId and ProductId are distinct types.
// Mixing them is a compile error.
fn lookup_product(user: UserId, product: ProductId) -> PriceInCents {
    // Inside the body: just numbers. No ceremony.
    let base = get_base_price(product);
    let discount = base * discount_rate;
    base - discount
}
```

This is analogous to Rust's lifetime inference: the compiler figures out most lifetimes automatically and only asks you to annotate when it can't.

## Intent-Driven Dataflow

Beyond semantic safety, Kāra is an **intent-driven language**. You describe the relationships between data and operations, not the order in which they execute.

In a traditional language, you might write:

```
let user = get_user_from_db(user_id);
let template = load_email_template("welcome");
send_email(user, template);
```

You are defining a strict, sequential order. But `get_user_from_db` and `load_email_template` have no dependency on each other. You'd have to manually introduce threads or async logic to run them in parallel.

In Kāra, you declare the data flow:

```rust
flow process_new_user {
    // These two pipelines are independent. The compiler understands this
    // and is free to execute them in parallel.
    (user_id = "user-123") -> get_user_from_db -> (user_record);
    (template_name = "welcome") -> load_email_template -> (email_template);

    // This depends on the results of the first two.
    // It will only run after both have completed.
    (user = user_record, template = email_template) -> send_email;
}
```

Because Kāra understands the dependency graph, it can automatically optimize for parallelism. You get multi-core performance without writing `thread.spawn()` or `async/await`.

## The Building Blocks

Kāra provides three core constructs:

1.  **`record`**: Simple, passive data structures that define the shape of your information.

2.  **`fn`**: A pure, side-effect-free data transformation. The compiler guarantees purity. Same inputs, same output, always.

3.  **`flow`**: The orchestration layer where you compose `fn`s and other `flow`s, manage state (`let`), and perform side-effects (I/O).

The strict separation of `fn` (pure) and `flow` (impure) is a compile-time guarantee. It is what enables the compiler to safely parallelize independent operations and reason about program behavior.

In the following chapters, we will walk through the language specification, core concepts, and how to build your first Kāra program.
