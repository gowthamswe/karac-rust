# Compiler Architecture: Choosing a Path

## The Origin: Data That Carries Its Own Meaning

Kāra's design is inspired by the Sanskrit language. In Sanskrit, every noun carries embedded context through morphological inflection (vibhakti). The case endings tell you what role a word plays — subject, object, instrument, location — regardless of where the word appears in the sentence. Word order is free because meaning is encoded in the word itself.

In most programming languages, data is context-free. An `i64` is just a number. Whether it represents a user ID, a price, or a temperature is known only to the programmer. The compiler cannot distinguish between them.

Kāra asks: **what if data carried its own semantic context?** If the compiler always knows what a piece of data *means*, it can prevent logical bugs, optimize more aggressively, and reason about program correctness at a deeper level than traditional type systems allow.

This vision — data as self-describing, context-carrying values — drives every architectural decision in the compiler.

---

## The Compilation Strategy

A core design decision in any new programming language is its compilation strategy. For Kāra, this decision determines how we realize the vision of semantic, context-carrying data without sacrificing performance.

We evaluated three architectures and chose a phased approach: **tree-walk interpreter first** for rapid iteration on language semantics, then **Project Sutra** (data-centric AOT compilation) for production performance.

---

## The Lexer

The first stage of the Kāra compiler is the lexer (or scanner), which converts raw source code into a sequence of tokens.

-   **Implementation:** The lexer operates directly on a byte slice (`&[u8]`) of the source code. This avoids allocating a separate `Vec<char>`, making it more memory-efficient. It uses `start` and `current` index pointers into the byte slice to track progress.
-   **Tokenization:** The lexer recognizes keywords, identifiers, literals (strings, integers, floats), and symbols. It uses a `match` statement on the current byte to dispatch to the correct tokenizing logic.
-   **Whitespace and Comments:** Whitespace and single-line comments (`//`) are skipped in a tight loop.

---

## Option 1: The Standard AOT Path (Rejected)

This is the traditional model used by languages like Rust, C++, and Go. The compiler translates source code into native machine code before the program runs.

-   **How it works:** The compiler produces a self-contained, optimized executable. All type information and high-level abstractions are erased and compiled down to raw machine instructions.
-   **The Problem for Kāra:** Semantic types would have to be represented as runtime data (e.g., `{ payload, tag }`), requiring defensive tag checks at every operation. This destroys CPU pipelining and vectorization, negating any performance benefit from the `fn`/`flow` purity split.
-   **Verdict:** Rejected. A standard AOT model respects Kāra's *syntax* but fails to deliver on the *promise* of zero-cost semantic safety.

---

## Option 2: The Standard JIT Path (Rejected)

This is the model used by V8 (JavaScript), the JVM (Java), and BEAM (Erlang). The system starts by interpreting code and compiles "hot" paths at runtime.

-   **How it works:** A VM observes the program as it runs and generates optimized machine code based on actual data types and values.
-   **The Problem for Kāra:** A JIT requires a complex runtime and VM, making it impossible to use for systems programming (kernels, bootloaders, device drivers).
-   **Verdict:** Rejected. While viable for the semantic context vision, it abandons the systems programming use case.

---

## Option 3: Project Sutra (Chosen)

Named after the Sanskrit word for "thread" or "rule" (सूत्र), Project Sutra is our chosen compilation strategy. It combines AOT performance with the expressive power of semantic types by making them a **zero-cost, compile-time abstraction.**

### How It Works: Data-Centric Compilation

Inspired by Rust's implementation of generics, Project Sutra uses monomorphization to compile semantic types away entirely:

1.  **Context in the Type System:** A piece of data's semantic context is not a runtime value — it is part of its **type**. A price is not an `f64`; it is a `PriceInCents`. This information exists only for the compiler.

2.  **Context at Boundaries:** The compiler enforces semantic types at function/flow boundaries — parameters, return values, record fields. Inside function bodies, local computation uses raw primitives with no overhead. This is analogous to Sanskrit's treatment of indeclinable particles (avyaya): words whose role is obvious from context need no inflection.

3.  **Monomorphization:** When a function is compiled, the compiler generates specialized machine code for the specific semantic types being used. The context information guides correct code generation, then is completely erased. Zero runtime cost.

### The Result

-   **Semantic safety without overhead:** The compiler catches logical bugs (mixing `UserId` with `ProductId`) at compile time, with no runtime penalty.
-   **Bare-metal performance:** The output is a standard native binary, suitable for systems programming.
-   **Automatic optimization:** Because the compiler understands the dataflow graph (from `flow` blocks) and knows which functions are pure (from `fn` declarations), it can parallelize and reorder operations that a traditional compiler cannot.

### Development Approach

To reach Project Sutra incrementally, we take a phased approach:

1.  **Phase 1 (Current): Tree-Walk Interpreter.** Execute Kāra programs by walking the AST directly. This validates language semantics — semantic type enforcement, purity checking, immutability — without the complexity of code generation. The interpreter is slow but correct, and allows rapid iteration on language design.

2.  **Phase 2 (Future): LLVM Code Generation.** Replace the interpreter with LLVM IR generation via the `inkwell` crate. This is where Project Sutra's monomorphization is implemented and the zero-cost abstraction promise is fulfilled.

The lexer, parser, and semantic analyzer are shared between both phases. Only the execution backend changes.

---

## The Kāra Pitch: Why Not Just Use Rust?

Kāra's goal is not to be "faster Rust." It is to offer a fundamentally different programming model.

### What Rust Gives You: Memory Safety

Rust eliminates entire classes of bugs related to pointers, data races, and memory management. Its ownership system is a compile-time guarantee that your program is memory-safe.

### What Kāra Gives You: Semantic Safety

Rust will not stop you from accidentally using a `user_id` where a `product_id` was expected. Kāra will. By making semantic context a first-class part of the type system — enforced at every boundary — Kāra prevents logical errors that no mainstream language catches.

This is the same relationship that Rust has to C++:

-   C++ gives you performance. Rust gives you performance **plus memory safety**.
-   Rust gives you memory safety. Kāra gives you memory safety **plus semantic safety**.

### The Sanskrit Analogy

In Sanskrit, you can reorder words freely because every word carries its own grammatical context. Meaning is unambiguous regardless of structure.

In Kāra, you can restructure your program — reorder operations, parallelize independent computations, refactor data pipelines — because every piece of data at a boundary carries its own semantic context. Correctness is guaranteed regardless of how the code is organized.

You choose Rust when you want fearless concurrency.

You choose Kāra when you want **fearless architecture** — the confidence that your program's high-level logic is as sound as its memory management.
