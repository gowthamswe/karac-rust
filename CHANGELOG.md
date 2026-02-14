# Changelog

All notable changes to the Kāra language will be documented in this file.

---

## [Unreleased] - YYYY-MM-DD

### Added

-   **Lexer Implementation:** A fully-featured lexer that tokenizes Kāra source code, including keywords, symbols, identifiers, numbers, and strings.
-   **Lexer Test Suite:** A comprehensive integration test to validate the lexer's correctness.
-   **Initial Project Structure:** Compiler, documentation, and VS Code settings.

### Changed

-   **Refined Language Design:** Solidified the roles of `record`, `fn`, and `flow`.
-   **Unified Function Call Syntax:** Standardized on the `->` operator for all named `fn` and `flow` calls, removing the separate `do` keyword.
-   **Introduced Inline Expressions:** Added support for ergonomic, C-style expressions (`let a = b + c;`) for simple arithmetic and logic, which are de-sugared by the compiler into a dataflow graph.
-   **Clarified Immutability:** Formally defined that `let` creates an immutable binding, not a mutable variable. Re-assignment is a compile-time error. This is the key enforcement mechanism for the dataflow model.
-   **Formalized Iteration:** Defined that all loops are to be implemented via recursive `flow` calls, not with traditional `for` or `while` keywords.
-   **Formalized Conditionals:** Added the `if` statement for conditional control flow.

### Next Steps

-   **Stage 2: Syntactic Analysis (Parser):** Build the parser to construct an Abstract Syntax Tree (AST) from the token stream. The AST must support nodes for our new inline expressions.
-   **Stage 3: Semantic Analysis & Graph Building:** Implement the semantic analyzer, which will walk the AST and build the final, optimized dataflow graph. This stage will include the "de-sugaring" of inline expressions.
