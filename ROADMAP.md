# Kāra Compiler Roadmap

This document outlines the planned features and development milestones for the `karac` compiler.

## Core Philosophy

Development will proceed in layers, focusing on building a solid foundation at each step. We will prioritize correctness and a robust testing strategy.

---

### Version 0.1.0: Core Parser & Lexer (Current)

**Goal:** To have a fully compliant lexer and a foundational parser that can read and understand the complete Kāra syntax.

- [x] **Lexer:** The lexer is complete and recognizes all keywords, symbols, and literals defined in the `DESIGN_RATIONALE.md`.
- [ ] **AST (Abstract Syntax Tree):** Define the core Rust structs that will represent the Kāra language's structure in memory (e.g., `Sutra`, `Record`, `Flow`, `Statement`, `Expression`, etc.).
- [ ] **Parser:** Build a parser to construct an Abstract Syntax Tree (AST) that represents `Record` definitions, `Sūtra` definitions, and `flow` blocks.
- [ ] **AST Validation:** Ensure the parser correctly builds AST nodes for our new, complete syntax.

### Version 0.2.0: Semantic Analysis & Type Checking

**Goal:** To build a semantic analyzer that can validate the correctness of the AST.

- [ ] **Symbol Table:** Implement a symbol table to track identifiers, types, and scopes.
- [ ] **Type Checker:** Walk the AST and enforce Kāra's static typing rules. Ensure that actions are called with the correct types and that variable assignments are valid.
- [ ] **Error Reporting:** Implement a robust error reporting system to give clear, actionable feedback to the developer about type mismatches or undefined variables.

### Version 0.3.0: LLVM IR Generation

**Goal:** To translate the validated AST into LLVM Intermediate Representation (IR).

- [ ] **LLVM Bridge:** Create the initial bridge to transpile the AST into basic LLVM IR. The first goal is to compile a simple `Sūtra` that performs integer arithmetic.
- [ ] **Variable Emitter:** Implement the logic to emit LLVM IR for variable declarations, assignments, and lookups.
- [ ] **Action Emitter:** Translate Kāra's data-flow actions (`->`) into LLVM instructions.

### Version 0.4.0: End-to-End (E2E) Testing Framework

**Goal:** To create a test harness that compiles `.kara` files and verifies their output, proving the compiler works from source to execution.

- [ ] **Test Runner:** Build a test runner within `cargo test` that can discover and execute `.kara` files from a dedicated test directory.
- [ ] **`karac` Execution:** The runner will execute the `karac` binary on a source `.kara` file.
- [ ] **Assertion:** The runner will compile the resulting LLVM IR, execute the binary, and assert that its output (e.g., stdout) matches the expected output.

### Future Goals

- **Conditional Logic:** Introduce declarative conditionals.
- **Standard Library:** Build a core library of essential `Sūtras` (e.g., `Print`, `Sqrt`, `Compare`).
- **Memory Management:** Implement the full memory model described in the language philosophy.
