# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Language Specification:** Created a formal EBNF grammar and language guide as `chapter_1.md` and `chapter_2.md` in the `docs/book` directory.
- **Lexer Implementation:** Implemented a complete, production-ready lexer (`src/lexer.rs`) capable of tokenizing the full Kāra language syntax, including Unicode support and comment handling.
- Created `docs/DESIGN_RATIONALE.md` to document the *why* behind our language design choices.

### Changed
- **Finalized Core Syntax:** Solidified the Kāra language syntax based on a "Middle Way" approach, combining high-level readability with low-level expressive power.
  - Reusable logic blocks are now `Sūtras` (`Define Sūtra: ...`).
  - The internal logic of a `Sūtra` uses the Pipe Operator (`->`) for a compact, high-density data flow graph.
- **Project Renaming:** The compiler executable is now officially named `karac`.

### Removed
- The conceptual names `Mandala` and `Vajra Arrow` have been replaced by `Sūtra` and the Pipe Operator (`->`) respectively.

## [0.0.1] - 2024-05-16

### Changed
- **Project Pivot: Sutra to Kāra.** The project was re-envisioned from a procedural language to a Role-Based logic language.

### Added
- Initial `mdBook` setup, `CHANGELOG.md`, and deprecated Sutra lexer.
