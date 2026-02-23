# Chapter 2: Language Specification

This chapter provides the formal specification of the Kāra language. It is the single source of truth for the language's syntax and semantics.

---

## 1. Type System

Kāra's type system is designed to be simple, explicit, and powerful, enabling both developer ergonomics and deep compiler optimizations.

### 1.1. Primitive Types

The language provides a set of built-in primitive types:

-   `i64`: A 64-bit signed integer.
-   `f64`: A 64-bit floating-point number.
-   `String`: A UTF-8 encoded string.
-   `bool`: A boolean value (`true` or `false`).

### 1.2. Semantic Types (Roles)

A `type` definition creates a new, **semantically distinct type** that is represented by an underlying primitive. This is the core mechanism for providing "Semantic Safety" and giving optimization hints to the compiler.

```rust
// Syntax: type <NewTypeName> <UnderlyingType>;

type OrderId i64;
type CustomerId i64;
type PriceUSD f64;
```

Operations between a semantic type and its underlying primitive are permitted and result in the semantic type. Operations between two different semantic types are a compile-time error. Explicit conversion is done via the `as` keyword.

### 1.3. Record Types

A `record` defines a composite, structural data type. 

```rust
// Syntax: record <RecordName> { <field>: <Type>, ... }

record Point {
    x: f64,
    y: f64,
}
```

---

## 2. Grammar (EBNF)

This section defines the complete Extended Backus-Naur Form (EBNF) grammar for the Kāra language.

```ebnf
(* Top Level *)
file ::= { topLevelDefinition };
topLevelDefinition ::= recordDefinition | typeDefinition | flowDefinition | functionDefinition;

(* Definitions *)
recordDefinition ::= 'record' IDENTIFIER '{' { fieldDefinition } '}';
fieldDefinition ::= IDENTIFIER ':' IDENTIFIER ';';

typeDefinition ::= 'type' IDENTIFIER IDENTIFIER ';';

flowDefinition ::= 'flow' IDENTIFIER '(' [paramList] ')' '{' { statement } '}';
functionDefinition ::= 'fn' IDENTIFIER '(' [paramList] ')' [ '->' '(' [paramList] ')' ] '{' { statement } '}';

paramList ::= IDENTIFIER ':' IDENTIFIER { ',' IDENTIFIER ':' IDENTIFIER };

(* Statements *)
statement ::= letBinding | pipelineStatement;

letBinding ::= 'let' IDENTIFIER '=' expression ';';

pipelineStatement ::= expression '->' IDENTIFIER { '->' IDENTIFIER } ';';

(* Expressions & Literals *)
expression ::= IDENTIFIER | literal;
literal ::= STRING_LITERAL | INTEGER_LITERAL | FLOAT_LITERAL | 'true' | 'false';

(* Lexical Tokens *)
IDENTIFIER ::= /* e.g., my_variable, MyType */
STRING_LITERAL ::= /* e.g., "hello world" */
INTEGER_LITERAL ::= /* e.g., 101, -42 */
FLOAT_LITERAL ::= /* e.g., 3.14, -0.5 */

```
