# Compiler Architecture

This document provides a high-level overview of the `karac` compiler's internal architecture. It is intended for developers working on the compiler itself.

While the source code is the ultimate source of truth, this document serves as a guide to the overall design, the flow of data through the compiler, and the responsibilities of each major component.

## 1. Compilation Pipeline

The `karac` compiler follows a traditional pipeline model. Source code is progressively transformed through a series of distinct stages, with the output of one stage becoming the input for the next.

The primary stages are:

1.  **Lexical Analysis (Lexing):** The raw source code text is converted into a linear sequence of tokens. This is handled by the `lexer` module.
2.  **Syntactic Analysis (Parsing):** The sequence of tokens is organized into a hierarchical representation of the code's structure, known as an Abstract Syntax Tree (AST). This will be handled by the `parser` module.
3.  **Semantic Analysis (Type Checking):** The AST is traversed to ensure the program is logically consistent. This involves checking types, resolving names, and enforcing language rules that can't be captured by the grammar alone. (Future stage)
4.  **Intermediate Representation (IR) Generation:** The validated AST is translated into a lower-level, machine-independent representation. (Future stage)
5.  **Code Generation (Backend):** The IR is used to generate the final executable code or target output. (Future stage)

---

## 2. Core Components

### Stage 1: The Lexer (`src/lexer.rs`)

-   **Responsibility:** To perform lexical analysis.
-   **Input:** A `String` containing the raw Kāra source code.
-   **Output:** A stream of `Token` enums.

#### Implementation Details

The lexer is implemented as a `Lexer` struct that scans the input `String`. It is designed to be fast and efficient.

-   **Unicode Support:** The lexer operates on a `Vec<char>` to correctly handle multi-byte Unicode characters, which is crucial for supporting Kāra identifiers like `Sūtra` directly in the code.
-   **State:** The lexer maintains its state through `position` and `read_position` pointers into the character vector. This allows for simple lookahead (via `peek_char`) which is necessary for distinguishing between tokens like `->` and single-character symbols.
-   **Tokenization Logic:** The core logic resides in the `next_token()` method, which forms the main loop of the lexer. It dispatches to helper methods for reading complex tokens like identifiers, numbers, and strings.
-   **Keyword Mapping:** A `lookup_ident` function is used to distinguish between user-defined identifiers and reserved keywords (e.g., `Record`, `flow`, `let`).
