# Kāra Compiler Changelog

All notable changes to this project will be documented in this file.

## [Unreleased] - Current Progress

### Completed

- **Project Structure Refactoring**:
  - Separated the core compiler logic into a distinct library crate (`src/lib.rs`).
  - Maintained the executable binary as a thin wrapper (`src/main.rs`) around the library.
- **Lexer Module (`src/lexer.rs`)**:
  - The lexer is feature-complete for the initial language syntax.
  - Added a comprehensive unit test suite inside the lexer module to validate tokenization of a full source code snippet.
- **Token Module (`src/token.rs`)**:
  - Defined all necessary tokens for the language syntax.
- **Version Control**:
  - Successfully committed and pushed all recent changes to the remote repository.

### Next Step

- **Implement the Parser and Abstract Syntax Tree (AST)**:
  - **1. Define AST:** Create `src/ast.rs` to define the data structures (structs and enums) that will represent the hierarchical structure of the code (e.g., `Record`, `Flow`, `Statement`, `Expression`).
  - **2. Create Parser:** Create `src/parser.rs` to house the `Parser`, which will consume the token stream from the Lexer and produce the AST.
