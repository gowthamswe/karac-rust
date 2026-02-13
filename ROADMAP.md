# Sutra Compiler Roadmap

This document outlines the high-level goals and future direction for the Sutra compiler project.

## Vision

To create a robust, efficient, and well-documented compiler for the Sutra language, focusing on clear error messages, performance, and ease of use.

## Milestones

### Version 0.1.0: Foundational Parser

- [ ] Define the core syntax of the Sutra language.
- [ ] Implement a basic lexer to tokenize the source code.
- [ ] Build a parser that can process a simple "Hello, World!" equivalent in Sutra.
- [ ] Establish the initial Abstract Syntax Tree (AST) structure.
- [ ] Set up continuous integration (CI) to run tests automatically.

### Version 0.2.0: Semantic Analysis

- [ ] Implement a symbol table to track variable bindings and types.
- [ ] Perform basic type checking.
- [ ] Provide meaningful error messages for semantic issues (e.g., undefined variables).

### Future Goals (Post v0.2.0)

- Code generation to a target intermediate representation (IR) or assembly.
- Standard library implementation.
- Advanced type system features.
- Concurrency support.
