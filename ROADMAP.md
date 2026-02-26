# Kāra Compiler Roadmap

This document outlines the planned features and development milestones for the `karac` compiler.

## Core Philosophy

Kāra draws its inspiration from the Sanskrit language, where every word carries embedded context (vibhakti) so that meaning is unambiguous regardless of word order. In the same way, Kāra aims to make data self-describing: every piece of data that crosses a boundary carries semantic context that the compiler can verify and enforce.

Development proceeds in layers, focusing on building a solid foundation at each step. We prioritize correctness and a robust testing strategy. We use a **tree-walk interpreter first** approach: validate language semantics with an interpreter before investing in LLVM code generation.

**North Star:** Self-hosting — the Kāra compiler should eventually be written in Kāra itself.

---

## Phase 1: Lexer (Complete)

**Goal:** A fully compliant lexer that tokenizes Kāra source code.

- [x] **Lexer:** Recognizes all keywords, symbols, and literals defined in the language specification.
- [x] **Lexer Tests:** Unit and integration tests validating correct tokenization.

---

## Phase 2: Parser & AST

**Goal:** A parser that constructs an Abstract Syntax Tree (AST) from the token stream.

- [ ] **AST Definition:** Define Rust structs representing the language's structure (`Record`, `Flow`, `Fn`, `Statement`, `Expression`, `SemanticType`, etc.).
- [ ] **Parser:** Build a recursive-descent parser that constructs the AST from tokens.
- [ ] **Span Tracking:** Attach source location (line, column, offset) to every token and AST node for error reporting.
- [ ] **Parser Tests:** Validate correct AST construction for all language constructs.

---

## Phase 3: Semantic Analysis & Type Checking

**Goal:** A semantic analyzer that validates program correctness, with a focus on the context-at-boundaries model.

- [ ] **Symbol Table:** Track identifiers, semantic types, and scopes.
- [ ] **Basic Type Checker:** Enforce static typing rules — correct parameter types, valid assignments, return type matching.
- [ ] **Semantic Type Enforcement:** Enforce that semantic types (e.g., `type UserId i64`) are distinct at all function/flow boundaries. A `UserId` cannot be passed where a `ProductId` is expected, even though both are `i64` underneath.
- [ ] **Purity Checker:** Verify that `fn` blocks contain no side-effects (no I/O calls, no `flow` calls).
- [ ] **Immutability Enforcement:** Verify that `let` bindings are never reassigned within `fn` blocks. Verify that only `let mut` bindings can be reassigned within `flow` blocks.
- [ ] **Error Reporting:** Clear, actionable error messages with source locations.

---

## Phase 4: Tree-Walk Interpreter

**Goal:** Execute Kāra programs directly from the AST, validating language semantics without the complexity of code generation.

- [ ] **Expression Evaluator:** Recursively walk AST nodes and evaluate expressions (arithmetic, comparison, boolean logic).
- [ ] **Flow Executor:** Execute `flow` blocks, including `let`/`let mut` bindings, `if/else` statements, pipeline calls, and recursive flows.
- [ ] **Function Executor:** Execute pure `fn` blocks with return values.
- [ ] **Built-in Functions:** Implement core built-ins (`Print`, `Read`, `ToString`, `ParseI64`).
- [ ] **E2E Tests:** Run `.kara` programs through the interpreter and verify stdout output.

---

## Phase 5: Core Language Features

**Goal:** The language features required to write non-trivial programs. These are prerequisites for self-hosting and general-purpose use.

### 5.1: Enums & Pattern Matching

Required for representing variants (AST nodes, Result/Option, any "one of N" type).

- [ ] **Enum Definitions:** Top-level `enum` construct with named variants, each optionally carrying data.
  ```
  enum Token {
      Integer(i64),
      Plus,
      Identifier(String),
      EOF,
  }
  ```
- [ ] **`match` Expressions:** Exhaustive pattern matching on enums. The compiler must verify that all variants are handled.
  ```
  match token {
      Integer(n) => { ... },
      Plus => { ... },
      Identifier(name) => { ... },
      EOF => { ... },
  }
  ```
- [ ] **Nested Patterns:** Match on nested structures (e.g., `Some(Ok(value))`).
- [ ] **Wildcard Pattern (`_`):** A catch-all for variants you don't need to handle individually.
- [ ] **`Result<T, E>` and `Option<T>`:** Defined as standard library enums, not special-cased in the compiler.

### 5.2: Generics

Required for type-safe collections and reusable data structures.

- [ ] **Generic Functions:** `fn identity<T>(value: T) -> T { value }`
- [ ] **Generic Records:** `record Pair<A, B> { first: A, second: B }`
- [ ] **Generic Enums:** `enum Option<T> { Some(T), None }`
- [ ] **Type Inference:** The compiler should infer generic type parameters where unambiguous.
- [ ] **Monomorphization:** Generics compiled by generating specialized code per concrete type (no runtime cost).

### 5.3: Mutable Bindings in `flow`

Required for practical stateful code (loops, accumulators, builders).

- [ ] **`let mut` in `flow`:** Allow mutable bindings inside `flow` blocks only. `fn` blocks remain fully immutable.
  ```
  flow collect_items {
      let mut items = List::new();
      // items can be reassigned here
      items = list_append(items, new_item);
  }
  ```
- [ ] **Purity enforcement:** The compiler rejects `let mut` inside `fn` blocks.

### 5.4: Iteration & Looping

Required for any non-trivial data processing.

- [ ] **`loop` / `while` in `flow`:** Traditional looping constructs, permitted only in `flow` blocks (since they require mutable state).
  ```
  flow process {
      let mut i = 0;
      while i < 10 {
          (value = i) -> Print;
          i = i + 1;
      }
  }
  ```
- [ ] **`break` and `continue`:** Control flow within loops.
- [ ] **`for..in` over collections:** Iterate over `List<T>` and other iterable types.
  ```
  for item in items {
      (value = item) -> process;
  }
  ```
- [ ] **Recursive flows:** Retained as an option. Recursive `flow` calls remain valid and are the idiomatic approach when no mutable state is desired.
- [ ] **Higher-Order Functions:** `map`, `filter`, `fold` on collections — the pure `fn` approach to iteration.
  ```
  let doubled = items.map(fn(x) { x * 2 });
  let total = items.fold(0, fn(acc, x) { acc + x });
  ```
- [ ] **Tail-Call Optimization:** Guaranteed TCO for recursive flows to prevent stack overflow.

### 5.5: Closures & Anonymous Functions

Required for higher-order functions (`map`, `filter`, `fold`, callbacks).

- [ ] **Anonymous `fn` (Lambdas):** Inline, unnamed pure functions.
  ```
  let doubled = items.map(fn(x: i64) -> i64 { x * 2 });
  ```
- [ ] **Closure Capture:** Anonymous functions can capture immutable values from their enclosing scope.
- [ ] **Closures as Parameters:** Functions and flows can accept `fn` parameters (function types).
  ```
  fn apply<T, U>(value: T, transform: fn(T) -> U) -> U {
      transform(value)
  }
  ```

### 5.6: Error Handling

Required for any program that interacts with the real world.

- [ ] **`Result<T, E>` and `Option<T>`:** Standard library enums (see 5.1).
- [ ] **`?` Operator (Error Propagation):** Syntactic sugar for early return on error.
  ```
  fn parse_config(text: String) -> Result<Config, ParseError> {
      let name = parse_name(text)?;    // returns early if Err
      let value = parse_value(text)?;  // returns early if Err
      Ok(Config { name: name, value: value })
  }
  ```
- [ ] **`return` Statement:** Early return from `fn` and `flow` blocks.

---

## Phase 6: Standard Library

**Goal:** Core types and functions required for general-purpose programming.

### 6.1: Collections

- [ ] **`List<T>`:** Dynamic-length ordered collection.
  - `List::new()`, `List::of(a, b, c)`
  - `length`, `get(index) -> Option<T>`, `first`, `last`
  - `append`, `prepend`, `concat`
  - `map`, `filter`, `fold`, `any`, `all`, `find`
  - `sort`, `reverse`, `contains`
  - `enumerate` (iteration with index)
  - `zip` (combine two lists)
  - `slice(start, end)`
- [ ] **`Map<K, V>`:** Key-value dictionary.
  - `Map::new()`
  - `insert`, `get(key) -> Option<V>`, `contains_key`
  - `remove`, `keys`, `values`, `entries`
  - `length`
- [ ] **`Set<T>`:** Unique unordered collection.
  - `Set::new()`
  - `insert`, `contains`, `remove`
  - `union`, `intersection`, `difference`
  - `length`
- [ ] **`Stack<T>`:** LIFO collection (useful for algorithms).
  - `push`, `pop -> Option<T>`, `peek -> Option<T>`
- [ ] **`Queue<T>`:** FIFO collection.
  - `enqueue`, `dequeue -> Option<T>`, `peek -> Option<T>`

### 6.2: String Operations

- [ ] **Concatenation:** `+` operator or `string::concat`.
- [ ] **Length:** `string::length`.
- [ ] **Slicing:** `string::slice(start, end)`.
- [ ] **Character Access:** `string::char_at(index) -> Option<char>`.
- [ ] **Search:** `string::contains`, `string::index_of`, `string::starts_with`, `string::ends_with`.
- [ ] **Transformation:** `string::to_uppercase`, `string::to_lowercase`, `string::trim`, `string::replace`.
- [ ] **Splitting/Joining:** `string::split(delimiter)`, `string::join(list, delimiter)`.
- [ ] **Formatting:** String interpolation or a `format` function.
  ```
  let msg = format("Hello, {}! You have {} items.", name, count);
  ```

### 6.3: Math Operations

- [ ] **Integer Math:** `abs`, `min`, `max`, `pow`, `mod` (modulo operator `%`).
- [ ] **Float Math:** `sqrt`, `floor`, `ceil`, `round`, `sin`, `cos`, `log`.
- [ ] **Constants:** `PI`, `E`, `MAX_I64`, `MIN_I64`.
- [ ] **Bitwise Operations:** `&` (and), `|` (or), `^` (xor), `<<` (shift left), `>>` (shift right), `~` (not). Required for many algorithmic problems.

### 6.4: I/O & System

- [ ] **Console:** `Print`, `Read` (already planned).
- [ ] **File I/O:** `read_file(path) -> Result<String, Error>`, `write_file(path, content) -> Result<(), Error>`.
- [ ] **Command-Line Arguments:** `args() -> List<String>`.
- [ ] **Process Exit:** `exit(code: i64)`.
- [ ] **Environment Variables:** `env_get(key) -> Option<String>`.

### 6.5: Type Conversions

- [ ] **Numeric:** `i64_to_f64`, `f64_to_i64` (truncating), `f64_round_to_i64`.
- [ ] **String Parsing:** `parse_i64(String) -> Result<i64, Error>`, `parse_f64(String) -> Result<f64, Error>`.
- [ ] **To String:** `to_string` for all primitive types.
- [ ] **Semantic Type Conversion:** `as` keyword for crossing context boundaries (e.g., `raw as UserId`).

---

## Phase 7: Advanced Language Features

**Goal:** Features that make the language expressive enough for complex software.

### 7.1: Traits / Interfaces

Required for polymorphism, operator overloading, and generic constraints.

- [ ] **Trait Definitions:** Define shared behavior across types.
  ```
  trait Display {
      fn to_display_string(self) -> String;
  }
  ```
- [ ] **Trait Implementations:** Implement traits for records and enums.
  ```
  impl Display for Point {
      fn to_display_string(self) -> String {
          format("({}, {})", self.x, self.y)
      }
  }
  ```
- [ ] **Trait Bounds on Generics:** Constrain generic types.
  ```
  fn print_all<T: Display>(items: List<T>) { ... }
  ```
- [ ] **Operator Overloading via Traits:** `Add`, `Eq`, `Ord`, `Hash` traits that let user-defined types work with `+`, `==`, `<`, etc.

### 7.2: Module System

Required for organizing code across multiple files.

- [ ] **Module Declarations:** `mod my_module;` to include another file.
- [ ] **Imports:** `use my_module::MyType;` to bring names into scope.
- [ ] **Visibility:** `pub` keyword for public items; private by default.
- [ ] **Nested Modules:** Modules can contain sub-modules.

### 7.3: Tuple Types

- [ ] **Tuple Literals:** `(1, "hello", true)`.
- [ ] **Tuple Types:** `(i64, String, bool)`.
- [ ] **Destructuring:** `let (x, y, z) = my_tuple;`.
- [ ] **Tuple Indexing:** `my_tuple.0`, `my_tuple.1`.

### 7.4: Range & Slice Types

- [ ] **Range Expressions:** `0..10`, `0..=10` (inclusive).
- [ ] **Range in `for` loops:** `for i in 0..10 { ... }`.
- [ ] **Slice from Range:** `list.slice(2..5)`, `string.slice(0..3)`.

### 7.5: Destructuring & Advanced Patterns

- [ ] **Record Destructuring:** `let Point { x, y } = my_point;`.
- [ ] **Tuple Destructuring:** `let (a, b) = my_tuple;`.
- [ ] **Nested Destructuring:** `let Pair { first: Point { x, y }, second } = my_pair;`.
- [ ] **`if let` shorthand:** `if let Some(value) = my_option { ... }`.
- [ ] **`while let`:** `while let Some(item) = stack.pop() { ... }`.

### 7.6: String Escape Sequences

- [ ] **Standard Escapes:** `\n` (newline), `\t` (tab), `\\` (backslash), `\"` (quote).
- [ ] **Unicode Escapes:** `\u{1F600}` for Unicode code points.

### 7.7: Comments

- [ ] **Single-line:** `//` (already implemented in lexer).
- [ ] **Multi-line / Block:** `/* ... */` with nesting support.
- [ ] **Doc Comments:** `///` for documentation generation.

### 7.8: Constants

- [ ] **Top-level Constants:** `const MAX_SIZE: i64 = 1024;`.
- [ ] **Compile-time Evaluation:** Constants must be evaluable at compile time.

---

## Phase 8: LLVM Code Generation

**Goal:** Replace the tree-walk interpreter with LLVM IR generation for compiled, high-performance output.

- [ ] **LLVM Bridge:** Integrate the `inkwell` crate to emit LLVM IR from the validated AST.
- [ ] **Variable Emitter:** Emit LLVM IR for variable declarations and lookups.
- [ ] **Function Emitter:** Translate `fn` and `flow` blocks into LLVM functions.
- [ ] **Pipeline Emitter:** Translate `->` pipeline calls into LLVM call instructions.
- [ ] **Enum Compilation:** Compile enums as tagged unions.
- [ ] **Closure Compilation:** Compile closures as function pointers + captured environment.
- [ ] **Semantic Type Monomorphization:** Compile semantic types as zero-cost abstractions via monomorphization — context guides code generation but is erased in the final binary.
- [ ] **Generic Monomorphization:** Generate specialized code per concrete type for all generic functions and types.

---

## Phase 9: End-to-End Testing & CLI

**Goal:** A complete toolchain from `.kara` source file to running binary.

- [ ] **CLI (`karac`):** A command-line binary that takes a `.kara` file, compiles it, and produces an executable.
  - `karac build <file.kara>` — compile to binary.
  - `karac run <file.kara>` — compile and run (or interpret).
  - `karac check <file.kara>` — type-check without executing.
- [ ] **E2E Test Runner:** A `cargo test` harness that compiles `.kara` files, runs the output, and asserts on stdout.
- [ ] **Error Formatting:** Polished, rustc-style error messages with source snippets and suggestions.
- [ ] **REPL (Optional):** Interactive interpreter for experimenting with the language.

---

## Phase 10: Memory Management

**Goal:** A memory model that provides safety without garbage collection overhead.

- [ ] **Ownership Model:** Define ownership and borrowing rules for heap-allocated data (records, enums, strings, collections).
- [ ] **Reference Counting or Borrow Checking:** Choose and implement a strategy — reference counting (simpler, like Swift) or borrow checking (zero-cost, like Rust).
- [ ] **Stack vs Heap:** Primitives and small records on the stack; large/dynamic data on the heap.
- [ ] **Automatic Memory Cleanup:** Memory is freed when the owner goes out of scope (RAII).

---

## Phase 11: Automatic Parallelism

**Goal:** Compiler-driven parallel execution of independent operations.

- [ ] **Dependency Analysis:** The compiler analyzes `flow` blocks to identify independent operations (no data dependencies between them).
- [ ] **Parallel Execution of Pure Functions:** Independent `fn` calls within a `flow` are executed on separate threads/tasks.
- [ ] **Synchronization:** The compiler inserts synchronization points where dependent operations need results from parallel ones.
- [ ] **Runtime Thread Pool:** A lightweight runtime that manages worker threads for parallel execution.

---

## Phase 12: Self-Hosting (North Star)

**Goal:** Rewrite the Kāra compiler in Kāra.

- [ ] **Lexer in Kāra:** Rewrite `src/lexer.rs` in Kāra.
- [ ] **Parser in Kāra:** Rewrite `src/parser.rs` in Kāra.
- [ ] **Type Checker in Kāra:** Rewrite `src/analyzer.rs` in Kāra.
- [ ] **Interpreter or Code Generator in Kāra:** Rewrite the execution backend.
- [ ] **Bootstrap:** The Kāra compiler compiles itself, producing an identical binary.

---

## Feature Checklist: General-Purpose Readiness

A quick reference of all features needed for Kāra to be a general-purpose language capable of solving arbitrary algorithmic problems.

### Data Types
- [x] Integers (`i64`)
- [x] Floats (`f64`)
- [x] Strings (`String`)
- [x] Booleans (`bool`)
- [x] Records (structs)
- [ ] Enums (sum types / tagged unions)
- [ ] Tuples
- [ ] Generics (`<T>`)
- [ ] Semantic types (`type X i64`)
- [ ] `Option<T>`, `Result<T, E>`

### Control Flow
- [x] `if` statements
- [ ] `if/else` and `else if`
- [ ] `match` (exhaustive pattern matching)
- [ ] `loop` / `while` (in `flow` only)
- [ ] `for..in` (over collections and ranges)
- [ ] `break` / `continue`
- [ ] `return` (early return)
- [ ] `?` operator (error propagation)
- [x] Recursive flows

### Functions & Closures
- [x] Named `fn` (pure)
- [x] Named `flow` (impure)
- [ ] Anonymous functions / lambdas
- [ ] Closures (capture from enclosing scope)
- [ ] Higher-order functions (fn as parameter)
- [ ] `map`, `filter`, `fold` on collections

### Data Structures (Stdlib)
- [ ] `List<T>` (dynamic array)
- [ ] `Map<K, V>` (hash map)
- [ ] `Set<T>` (hash set)
- [ ] `Stack<T>`
- [ ] `Queue<T>`

### Operators
- [x] Arithmetic: `+`, `-`, `*`, `/`
- [ ] Modulo: `%`
- [x] Comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`
- [ ] Logical: `&&`, `||`, `!`
- [ ] Bitwise: `&`, `|`, `^`, `<<`, `>>`, `~`
- [ ] Assignment: `=` (and `let mut` reassignment in `flow`)
- [ ] Range: `..`, `..=`

### String Operations
- [ ] Concatenation, length, slicing
- [ ] Search, replace, split, join
- [ ] Character access
- [ ] String formatting / interpolation

### Math
- [ ] `abs`, `min`, `max`, `pow`, `mod`
- [ ] `sqrt`, `floor`, `ceil`, `round`
- [ ] Trigonometric, logarithmic functions

### Type System
- [ ] Traits / interfaces
- [ ] Trait bounds on generics
- [ ] Operator overloading via traits

### Organization
- [ ] Module system (`mod`, `use`, `pub`)
- [ ] Constants (`const`)
- [ ] Doc comments (`///`)

### Memory
- [ ] Ownership / borrowing model
- [ ] Automatic memory cleanup (RAII)

### Tooling
- [ ] CLI (`karac build`, `karac run`, `karac check`)
- [ ] Error messages with source snippets
- [ ] REPL (optional)
