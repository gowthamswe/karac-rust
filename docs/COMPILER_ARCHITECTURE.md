# Compiler Architecture

This document provides a high-level overview of the `karac` compiler's internal architecture. It is intended for developers working on the compiler itself.

While the source code is the ultimate source of truth, this document serves as a guide to the overall design, the flow of data through the compiler, and the responsibilities of each major component.

## 1. Compilation Pipeline

The `karac` compiler follows a traditional pipeline model. Source code is progressively transformed through a series of distinct stages, with the output of one stage becoming the input for the next.

The strategy is **interpreter first, compiler second**: we validate language semantics with a tree-walk interpreter before investing in LLVM code generation. This lets us iterate on language design rapidly.

### Phase 1: Interpreter Pipeline (Current Target)

```
Source Code (.kara)
    │
    ▼
┌──────────┐
│  Lexer   │  src/lexer.rs — converts source text to tokens
└────┬─────┘
     │ Vec<Token>
     ▼
┌──────────┐
│  Parser  │  src/parser.rs (planned) — builds AST from tokens
└────┬─────┘
     │ AST
     ▼
┌──────────────┐
│  Semantic    │  src/analyzer.rs (planned) — type checking,
│  Analyzer    │  purity enforcement, semantic type validation
└────┬─────────┘
     │ Validated AST
     ▼
┌──────────────┐
│  Tree-Walk   │  src/interpreter.rs (planned) — executes
│  Interpreter │  the AST directly
└──────────────┘
```

### Phase 2: Compiled Pipeline (Future)

```
     Validated AST
     │
     ▼
┌──────────────┐
│  LLVM IR     │  src/codegen.rs (future) — emits LLVM IR
│  Generator   │  via the inkwell crate
└────┬─────────┘
     │ LLVM IR
     ▼
┌──────────────┐
│  LLVM        │  off-the-shelf LLVM — optimization passes
│  Backend     │  and native code generation
└──────────────┘
     │
     ▼
  Native Binary
```

---

## 2. Core Components

### Stage 1: The Lexer (`src/lexer.rs`) — Complete

-   **Responsibility:** Lexical analysis — converting raw source text into a stream of tokens.
-   **Input:** A `&str` containing Kāra source code.
-   **Output:** A stream of `Token` enums (via repeated calls to `next_token()`).

#### Implementation Details

The lexer is implemented as a `Lexer` struct that scans the input source.

-   **Byte-Level Operation:** The lexer operates directly on a byte slice (`&[u8]`) of the source code. This avoids allocating a separate `Vec<char>`, making it more memory-efficient.
-   **State:** The lexer maintains its state through `start` and `current` index pointers into the byte slice, plus a `line` counter for error reporting.
-   **Tokenization Logic:** The core logic resides in `scan_token()`, which dispatches on the current byte. It uses `match_char()` for single-byte lookahead to distinguish multi-character tokens (e.g., `->`, `!=`, `==`, `<=`, `>=`).
-   **Keyword Matching:** The `identifier_type()` method maps reserved words (`fn`, `flow`, `record`, `type`, `let`, `if`, `true`, `false`, `as`) to their keyword tokens.
-   **Whitespace and Comments:** Whitespace (spaces, tabs, carriage returns, newlines) and single-line comments (`//`) are skipped in a tight loop within `skip_whitespace()`.

### Stage 2: The Parser (`src/parser.rs`) — Planned

-   **Responsibility:** Syntactic analysis — organizing tokens into an Abstract Syntax Tree (AST).
-   **Input:** A `Vec<Token>` from the lexer.
-   **Output:** An AST representing the program structure.
-   **Approach:** Recursive-descent parser, following the EBNF grammar defined in the language specification (Chapter 2).

### Stage 3: The Semantic Analyzer (`src/analyzer.rs`) — Planned

-   **Responsibility:** Validating that the AST is logically correct.
-   **Key checks:**
    -   **Semantic type enforcement:** Ensuring that semantic types (e.g., `UserId`, `ProductId`) are treated as distinct at all function/flow boundaries.
    -   **Purity checking:** Verifying that `fn` blocks contain no side-effects.
    -   **Immutability enforcement:** Verifying that `let` bindings are never reassigned.
    -   **Name resolution:** Building a symbol table, resolving identifiers to their declarations.

### Stage 4: The Interpreter (`src/interpreter.rs`) — Planned

-   **Responsibility:** Executing the validated AST directly.
-   **Approach:** Tree-walk evaluation — recursively walk AST nodes and compute results.
-   **Purpose:** Validate language semantics, enable rapid iteration on language design, provide a working Kāra execution environment before LLVM integration.

### Stage 5: LLVM Code Generation (`src/codegen.rs`) — Future

-   **Responsibility:** Translating the validated AST into LLVM IR.
-   **Approach:** Use the `inkwell` crate (Rust bindings to the LLVM C API) to emit LLVM IR, then hand off to LLVM for optimization and native code generation.
-   **Monomorphization:** Semantic types are compiled as zero-cost abstractions via monomorphization. The type context guides code generation (e.g., generating specialized function variants per semantic type) but is completely erased in the final binary.
