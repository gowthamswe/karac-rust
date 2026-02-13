# Kāra Language Roadmap

## 1. The Core Vision

Kāra is a high-performance systems language designed to provide C-level speed with a radically different mental model. It rejects both the manual memory management overhead of C and the "Noun-centric" (Object-Oriented) complexity of modern languages.

## 2. The Philosophy: Kāraka Logic

The language is built on the ancient Sanskrit Kāraka framework, which focuses on **Roles** and **Actions** rather than "Objects." Code should read like a logical statement of intent.

- **No OOP:** There are no classes or inheritance. Instead, there are **Actions** (Verbs).
- **Role-Based Data:** Data is not just a "parameter"; it fulfills a specific role in an action (e.g., the *Source* of an operation, the *Destination* of a result).

## 3. The Architecture

- **Host & Backend:** The compiler is written in Rust (for safety) and targets LLVM IR (for hardware-agnostic performance).
- **Memory Model:** We will implement **Self-Cleaning Arenas**. Memory is tied to the scope of an `Action`. When the action finishes, its memory arena is wiped clean instantly. This is more performant than a GC and far simpler than Rust's borrow checker.
- **Logic Registry:** A centralized compiler component that maps `Action` names (e.g., `Add`) to specific hardware instructions or more complex operations.

## 4. Milestones

### Version 0.1.0: Core Engine

- [ ] **Lexer:** Re-implement the lexer for the Kāra syntax (`Action:`, `From:`, `Into:`).
- [ ] **Parser:** Build a recursive descent parser to create a structured `KaraAction` AST (Abstract Syntax Tree).
- [ ] **Stack Registry:** Implement the core logic for variable chaining, allowing the `Into` of one action to be used as a `From` in another.
- [ ] **Compiler Backend:** Implement the "Vajra Bridge" to transpile the AST into basic LLVM IR for integer arithmetic.

### Version 0.2.0: Foundational Library & Modularity

- [ ] **Modularization:** Split the monolithic compiler source into `lexer.rs`, `parser.rs`, and `compiler.rs`.
- [ ] **Arena Allocator:** Implement the first version of the Self-Cleaning Arena memory model.
- [ ] **Logic Registry Expansion:** Expand the registry to include `Multiply`, `Divide`, and basic Boolean logic.

### Future Goals (Post v0.2.0)

- **Conditional Logic:** Introduce `Observe` and `Case` keywords for branching.
- **Expanded Types:** Move beyond integers to support floats, strings, and custom data structures (`Records`).
- **Standard Library:** Begin building a "Sūtra Library" of core actions.
