# Chapter 2: Language Specification

This chapter provides the official reference grammar for the Kāra language. This is the source of truth for the language's syntax.

We use a form of Extended Backus-Naur Form (EBNF) to describe the syntax:

*   `UPPERCASE` denotes a terminal token from the lexer (e.g., `IDENTIFIER`, `LET`).
*   `camelCase` denotes a production rule (a non-terminal).
*   `'text'` denotes a literal string.
*   `|` denotes an alternative.
*   `[]` denotes an optional part.
*   `{}` denotes zero or more repetitions.

---

## 1. Program Structure

A Kāra program is a collection of top-level definitions.

```ebnf
program = { topLevelDefinition } ;

topLevelDefinition = recordDefinition | sutraDefinition | flowDefinition ;
```

---

## 2. Lexical Elements

(Details on comments and identifiers as before)

---

## 3. `Record` Definitions

(Details on Record definitions as before)

---

## 4. `Sūtra` Definitions

(Details on Sūtra definitions as before)

---

## 5. `flow` Definitions

(Details on flow definitions as before)

---

## 6. Statements and Expressions

Statements are the units of execution within `Sūtra` and `flow` bodies.

### Grammar

```ebnf
statement = letBinding | actionCall ; // For now
```

### Let Bindings

A `let` binding introduces a new variable.

```ebnf
letBinding = 'let' IDENTIFIER '=' expression ';' ;
```

### Expressions

An expression is a piece of code that evaluates to a value.

```ebnf
expression = literal | recordLiteral | variableAccess ; 
```

#### Literals

`literal = NUMBER | STRING ;`

#### Record Literals

`recordLiteral = IDENTIFIER '{' [ fieldInitializer { ',' fieldInitializer } [ ',' ] ] '}' ;`

`fieldInitializer = IDENTIFIER ':' expression ;`

#### Variable and Field Access

This expression is used to access the value of a variable or a field within a `Record`.

```ebnf
variableAccess = IDENTIFIER [ '.' IDENTIFIER ] ;
```

**Examples:**
```rust
let a = my_variable;      // Access a variable
let b = my_record.x;    // Access field 'x' of a record
```

### Action Calls

An `actionCall` invokes a `Sūtra` to perform a transformation. There are two syntactic forms.

#### Verbose Syntax

This form is self-documenting and primarily used for orchestration in `flow` blocks.

```ebnf
verboseActionCall = 'Action' ':' IDENTIFIER
                    { fromClause }
                    [ intoClause ] ';' ;

fromClause = 'From' ':' IDENTIFIER '=' expression ;

intoClause = 'Into' ':' IDENTIFIER ;
```

- It begins with `Action:`, followed by the name of the `Sūtra` to call.
- `From:` clauses map local variables to the required inputs of the `Sūtra`.
- The optional `Into:` clause specifies the variable to store the `Sūtra`'s output.
- The entire statement ends with a semicolon `;`.

**Example:**
```rust
// Assuming a Sūtra 'CalculateDistance' that takes p1 and p2
Action: CalculateDistance
  From: p1 = origin_point
  From: p2 = destination_point
  Into: distance_result;
```

#### Pipe Syntax

This form is dense and designed for chaining operations within `Sūtra` bodies.

```ebnf
pipeActionCall = source '->' IDENTIFIER '->' destination ';' ;

source = IDENTIFIER | tuple ;
destination = IDENTIFIER | tuple ;

tuple = '(' IDENTIFIER { ',' IDENTIFIER } ')' ;
```

- It represents the flow of data from a `source`, through an `action`, to a `destination`.
- The `source` and `destination` can be a single variable or a parenthesized, comma-separated tuple of variables.
- The `action` is the name of the `Sūtra` to execute.
- The statement ends with a semicolon `;`.

**Example:**

```rust
// Single source and destination
input -> Square -> squared_output;

// Multiple sources and destinations (hypothetical)
(a, b) -> AddAndSubtract -> (sum, difference);
```
