# Kāra v2: Brainstorming — A Language for the AI Era

This document captures a from-first-principles rethinking of what Kāra could be. It sets aside all previous design decisions and asks: if we were building a language today with full freedom, what would it look like?

## Starting Assumptions

1. **AI writes most code** — optimize for machine readability, formal verifiability, and unambiguous semantics rather than human ergonomics.
2. **Compiler should extract maximum performance** — give the compiler every advantage to optimize aggressively.
3. **Silicon matters** — modern CPUs are memory-bound, not compute-bound. Cache misses, branch mispredictions, and memory layout dominate performance.
4. **Take the best from everything** — no loyalty to any paradigm.

---

## The Core Problem

The fundamental problem with every mainstream language is that **the programmer describes computation, and the compiler has to reverse-engineer intent.**

The programmer writes `for i in 0..n { result += data[i]; }` and the compiler has to figure out: can I vectorize this? Can I parallelize it? Is there aliasing? Are there dependencies between iterations? The compiler does heroic work (LLVM has millions of lines of optimization passes) to recover information that the programmer had in their head but didn't express.

**What if the language expressed intent directly, and the compiler didn't have to guess?**

---

## Feature 1: Data Layout as a First-Class Concept

The biggest performance problem in modern software isn't algorithms — it's cache misses. A `Vec<Entity>` where `Entity` has 20 fields but you only access `position` and `velocity` wastes cache lines loading 18 unused fields per entity.

The language separates **logical structure** from **physical layout**:

```
// Logical: how the programmer thinks about data
struct Entity {
    id: u64,
    name: String,
    position: Vec3,
    velocity: Vec3,
    health: f32,
    armor: f32,
    is_alive: bool,
}

// Physical: how data is actually stored in memory
// The compiler chooses, or the programmer overrides
layout entities: Collection<Entity> {
    // Store position and velocity together (hot path: physics)
    group physics { position, velocity }

    // Store health and armor together (hot path: combat)
    group combat { health, armor, is_alive }

    // Everything else is cold
    group metadata { id, name }
}
```

This is struct-of-arrays (SoA) vs array-of-structs (AoS) — the most impactful performance optimization in data-heavy systems — expressed as a language feature rather than a manual refactoring nightmare. The programmer writes code against `Entity`. The compiler generates code that accesses the correct physical layout. If you iterate over `entities` and only touch `position` and `velocity`, only the `physics` group is loaded into cache.

**Who does this today?** ECS frameworks (Unity DOTS, Bevy) do this manually. Zig lets you control memory layout. But no language makes it a first-class concept with compiler-managed mapping between logical and physical.

---

## Feature 2: Effect Types — Every Function Declares What It Does

Every function declares what it does to the world:

```
fn calculate(x: f64, y: f64) -> f64 { ... }
// Inferred effect: none — the compiler knows this is pure

fn read_config(path: String) -> Result<Config, Error>
    reads(FileSystem) { ... }
// Declared effect: reads from filesystem

fn save_user(user: User) -> Result<(), Error>
    writes(Database) { ... }
// Declared effect: writes to database

fn process(id: u64) -> Result<Report, Error>
    reads(Database), writes(FileSystem), sends(Network) { ... }
// Multiple effects — composed from callees
```

The compiler uses effects for:

- **Auto-parallelism:** Functions with non-conflicting effects run in parallel without programmer intervention.
- **Testing:** Mock any effect boundary automatically.
- **Sandboxing:** Restrict what a module can do by limiting its allowed effects.
- **Dead code elimination:** If nothing reads from a write, the write can be eliminated.

Effects are inferred when possible. If your function only calls pure functions, its effect is `none` without you saying anything. You only annotate when using FFI or built-in I/O primitives.

**Who does this today?** Koka, Eff (research languages). Nobody in mainstream.

### Why this is better than fn/flow

The earlier `fn` (pure) vs `flow` (impure) design was binary — a function is either pure or it isn't. Effect types are a spectrum. The compiler can reason about *which specific effects* conflict and parallelize everything that doesn't conflict. Two functions that both `reads(Database)` can run in parallel. A function that `reads(UserDB)` and a function that `writes(OrderDB)` can run in parallel because they touch different resources.

---

## Feature 3: Algebraic Data Types with Layout-Aware Matching

Take Rust's enums and pattern matching (the best version of this in any mainstream language) and add layout awareness:

```
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { a: Vec2, b: Vec2, c: Vec2 },
}

// Exhaustive matching — compiler error if a variant is missed
match shape {
    Circle { radius } => pi * radius * radius,
    Rectangle { width, height } => width * height,
    Triangle { a, b, c } => triangle_area(a, b, c),
}

// Layout optimization for collections of enums
layout shapes: Collection<Shape> {
    split_by_variant  // circles stored separately from rectangles
    // Now iterating over all circles is cache-perfect
}
```

Rust's `match` semantics are already the best design. Keep them exactly as-is. The addition is `split_by_variant` in the layout system, which stores each variant contiguously in memory for cache-friendly iteration.

---

## Feature 4: Ownership Without Lifetime Annotations

Rust's ownership model is correct but lifetime annotations are the primary source of developer frustration. Take the ownership model but make lifetimes fully inferred:

- Values have single owners (like Rust).
- Borrowing is inferred by the compiler from usage patterns.
- When the compiler can't infer, it uses reference counting as a fallback (not a compile error).
- The programmer never writes `'a` or `&'a`.

```
fn longest(a: String, b: String) -> String {
    if a.len() > b.len() { a } else { b }
}
// Compiler infers: takes ownership of both, returns one.
// No lifetime annotation needed.

fn first_word(s: String) -> String {
    s.split(' ').first()
}
// Compiler infers: borrows s, returns a view into it.
// If it can't prove safety statically, it uses reference counting.
// Performance note emitted so AI/developer can optimize.
```

The tradeoff: you sometimes get unexpected clones or reference counting where Rust would force you to think about lifetimes. But since AI is writing the code, the AI can read the compiler's performance notes and optimize. Human developers no longer need to fight the borrow checker.

**Who does this today?** Swift does reference counting. Vale does generational references. Lobster does compile-time RC. Nobody combines Rust-style ownership with inferred fallback to RC.

---

## Feature 5: Auto-Concurrency via Effect Analysis

No `async/await`. No colored functions. No choosing between threads and async.

```
fn load_dashboard(user_id: u64) -> Dashboard {
    // The compiler sees these have non-conflicting effects
    // and runs them concurrently automatically
    let profile = fetch_profile(user_id);       // reads(UserDB)
    let orders = fetch_orders(user_id);         // reads(OrderDB)
    let notifications = fetch_notifs(user_id);  // reads(NotifDB)

    // This depends on all three — compiler inserts sync point
    build_dashboard(profile, orders, notifications)
}
```

No `async` keyword. No `.await`. No `join!`. No `Promise.all`. The compiler handles concurrency because it understands effects and data dependencies. The programmer writes sequential-looking code. The compiler generates concurrent execution.

This only works because of the effect system (Feature 2). Without effects, the compiler can't know that `fetch_profile` and `fetch_orders` don't conflict.

**Who does this today?** Nobody, really. This is the combination of algebraic effects + automatic parallelism that hasn't been realized in a practical language.

---

## Feature 6: Gradual Verification

Types are just the first level. The language supports increasingly strong guarantees that the programmer (or AI) can opt into:

```
// Level 1: Basic types (always on)
fn add(a: i64, b: i64) -> i64 { a + b }

// Level 2: Constrained types (opt-in)
type PositiveInt = i64 where value > 0;
type Percentage = f64 where 0.0 <= value <= 100.0;

fn discount(price: f64, pct: Percentage) -> f64 {
    price * (1.0 - pct / 100.0)
}
// Compiler proves: result >= 0 when price >= 0

// Level 3: Pre/post conditions (opt-in)
fn binary_search(list: List<i64>, target: i64) -> Option<usize>
    requires list.is_sorted()
    ensures result.is_some() implies list[result.unwrap()] == target
{
    // ...
}

// Level 4: Full formal verification (opt-in, for critical code)
fn transfer(from: Account, to: Account, amount: Money)
    requires from.balance >= amount
    ensures from.balance == old(from.balance) - amount
    ensures to.balance == old(to.balance) + amount
    ensures from.balance + to.balance == old(from.balance) + old(to.balance)
{
    // The compiler proves these properties hold
}
```

Since AI writes the code, it can generate the verification annotations. Humans review the specifications (which are shorter and clearer than the implementation) rather than reviewing the code itself.

**Who does this today?** Dafny, F*, Liquid Haskell, Ada/SPARK. But they're all verification-first languages. Nobody offers this as a gradient where you can verify the critical 5% and leave the rest as normal typed code.

---

## Feature 7: Compilation Target Flexibility

The same language compiles to:

- **Native code** via LLVM (systems, CLI tools, game engines)
- **WebAssembly** (browser, edge computing)
- **GPU compute shaders** (data processing, ML inference)
- **FPGA bitstreams** (embedded, ultra-low-latency) — future goal

The data layout system (Feature 1) makes this natural. When targeting a GPU, `Collection<Entity>` with `group physics { position, velocity }` maps directly to a GPU buffer with the right memory layout for coalesced access.

---

## Summary

| Feature | What it does | Why it matters |
|---|---|---|
| Data layout as first-class | Logical structure separate from physical layout | Cache performance — the actual bottleneck |
| Effect types | Every function declares its effects | Enables auto-parallelism, testing, sandboxing |
| Ownership without lifetimes | Rust semantics, fully inferred, RC fallback | Memory safety without annotation burden |
| Auto-concurrency | Compiler parallelizes non-conflicting effects | No async/await, no colored functions |
| Gradual verification | Types → constraints → pre/post → formal proofs | Verify critical code, skip the rest |
| Layout-aware compilation | Same code → CPU, GPU, WASM | Write once, optimize for target hardware |

### The Pitch in One Sentence

**A language where the programmer declares *what* and *why*, and the compiler decides *how* and *where* — with the freedom to lay out memory, schedule concurrency, and target hardware without the programmer specifying any of it.**

---

## What to Build First

If picking a single most impactful feature to start with: **effect types + auto-concurrency**.

Rationale:
- No mainstream language has it.
- It solves the async/await colored-function problem that plagues every language.
- It makes the auto-parallelism story actually work (unlike the earlier fn/flow design which was too coarse-grained).
- It composes naturally with the rest of the features.
- The effect system can be built incrementally — start with a few built-in effects (reads, writes, sends) and expand over time.

---

## Design Decisions Still Open

1. **Syntax:** Should effects be declared explicitly or inferred? (Recommendation: inferred by default, explicit at module boundaries.)
2. **Effect granularity:** `reads(Database)` vs `reads(UserDB)` vs `reads(users_table)` — how fine-grained?
3. **Layout system:** Fully automatic with hints, or programmer-specified? (Recommendation: automatic with optional overrides.)
4. **Verification:** Which SMT solver to integrate? Z3 is the standard choice.
5. **Concurrency runtime:** Green threads, OS threads, or both? (Recommendation: green threads with work-stealing scheduler, like Go/Tokio.)

---

## Prior Art and Influences

- **Rust:** Ownership, enums, pattern matching, traits
- **Koka:** Algebraic effect system
- **Zig:** Memory layout control, comptime
- **Go:** Simple concurrency model, fast compilation
- **F#:** Units of measure, pipe operator
- **Dafny/F*:** Gradual verification, pre/post conditions
- **Unity DOTS/Bevy:** Data-oriented design, SoA layouts
- **Swift:** Inferred reference counting
- **Vale:** Generational references as ownership fallback
