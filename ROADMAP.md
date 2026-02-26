# Kāra Compiler Roadmap

This document outlines the planned features and development milestones for the `karac` compiler.

## Core Philosophy

Kāra draws its inspiration from the Sanskrit language, where every word carries embedded context (vibhakti) so that meaning is unambiguous regardless of word order. In the same way, Kāra aims to make data self-describing: every piece of data that crosses a boundary carries semantic context that the compiler can verify and enforce.

Development proceeds in layers, focusing on building a solid foundation at each step. We prioritize correctness and a robust testing strategy. We use a **tree-walk interpreter first** approach: validate language semantics with an interpreter before investing in LLVM code generation.

---

### Phase 1: Lexer (Complete)

**Goal:** A fully compliant lexer that tokenizes Kāra source code.

- [x] **Lexer:** Recognizes all keywords, symbols, and literals defined in the language specification.
- [x] **Lexer Tests:** Unit and integration tests validating correct tokenization.

### Phase 2: Parser & AST

**Goal:** A parser that constructs an Abstract Syntax Tree (AST) from the token stream.

- [ ] **AST Definition:** Define Rust structs representing the language's structure (`Record`, `Flow`, `Fn`, `Statement`, `Expression`, `SemanticType`, etc.).
- [ ] **Parser:** Build a recursive-descent parser that constructs the AST from tokens.
- [ ] **Span Tracking:** Attach source location (line, column, offset) to every token and AST node for error reporting.
- [ ] **Parser Tests:** Validate correct AST construction for all language constructs.

### Phase 3: Semantic Analysis & Type Checking

**Goal:** A semantic analyzer that validates program correctness, with a focus on the context-at-boundaries model.

- [ ] **Symbol Table:** Track identifiers, semantic types, and scopes.
- [ ] **Basic Type Checker:** Enforce static typing rules — correct parameter types, valid assignments, return type matching.
- [ ] **Semantic Type Enforcement:** Enforce that semantic types (e.g., `type UserId i64`) are distinct at all function/flow boundaries. A `UserId` cannot be passed where a `ProductId` is expected, even though both are `i64` underneath.
- [ ] **Purity Checker:** Verify that `fn` blocks contain no side-effects (no I/O calls, no `flow` calls).
- [ ] **Immutability Enforcement:** Verify that `let` bindings are never reassigned.
- [ ] **Error Reporting:** Clear, actionable error messages with source locations.

### Phase 4: Tree-Walk Interpreter

**Goal:** Execute Kāra programs directly from the AST, validating language semantics without the complexity of code generation.

- [ ] **Expression Evaluator:** Recursively walk AST nodes and evaluate expressions (arithmetic, comparison, boolean logic).
- [ ] **Flow Executor:** Execute `flow` blocks, including `let` bindings, `if` statements, pipeline calls, and recursive flows.
- [ ] **Function Executor:** Execute pure `fn` blocks.
- [ ] **Built-in Functions:** Implement core built-ins (`Print`, `Read`, `ToString`, `ParseI64`).
- [ ] **E2E Tests:** Run `.kara` programs through the interpreter and verify stdout output.

### Phase 5: LLVM Code Generation

**Goal:** Replace the tree-walk interpreter with LLVM IR generation for compiled, high-performance output.

- [ ] **LLVM Bridge:** Integrate the `inkwell` crate to emit LLVM IR from the validated AST.
- [ ] **Variable Emitter:** Emit LLVM IR for variable declarations and lookups.
- [ ] **Function Emitter:** Translate `fn` and `flow` blocks into LLVM functions.
- [ ] **Pipeline Emitter:** Translate `->` pipeline calls into LLVM call instructions.
- [ ] **Semantic Type Monomorphization (Project Sutra):** Compile semantic types as zero-cost abstractions via monomorphization — context guides code generation but is erased in the final binary.

### Phase 6: End-to-End Testing & CLI

**Goal:** A complete toolchain from `.kara` source file to running binary.

- [ ] **CLI (`karac`):** A command-line binary that takes a `.kara` file, compiles it, and produces an executable.
- [ ] **E2E Test Runner:** A `cargo test` harness that compiles `.kara` files, runs the output, and asserts on stdout.
- [ ] **Error Formatting:** Polished, rustc-style error messages with source snippets and suggestions.

### Future Goals

- **`match` Expressions:** Pattern matching on `Result`, `Option`, and user-defined enums.
- **Generics:** Lightweight `<T>` generics for functions and data structures.
- **Standard Library:** Core types (`List<T>`, `Result<T, E>`, `Option<T>`) and functions (`string::split`, `list::length`, etc.).
- **Automatic Parallelism:** Compiler-driven parallel execution of independent `fn` calls within a `flow`.
- **Tail-Call Optimization:** Guaranteed TCO for recursive flows to prevent stack overflow.
- **Memory Management:** Full memory model as described in the language philosophy.
