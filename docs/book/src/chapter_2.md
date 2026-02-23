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

A `type` definition creates a new, **semantically distinct type** that is represented by an underlying primitive.

```rust
// Syntax: type <NewTypeName> <UnderlyingType>;
type OrderId i64;
```

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
recordDefinition ::= 'record' IDENTIFIER '{' [ fieldDefinition { ',' fieldDefinition } [','] ] '}';
fieldDefinition ::= IDENTIFIER ':' IDENTIFIER;

typeDefinition ::= 'type' IDENTIFIER IDENTIFIER ';';

flowDefinition ::= 'flow' IDENTIFIER '(' [paramList] ')' blockExpression;
functionDefinition ::= 'fn' IDENTIFIER '(' [paramList] ')' [ '->' returnType ] blockExpression;

paramList ::= IDENTIFIER ':' IDENTIFIER { ',' IDENTIFIER ':' IDENTIFIER } [','];
returnType ::= IDENTIFIER | '(' IDENTIFIER { ',' IDENTIFIER } [','] ')';

(* Block & Statements *)
blockExpression ::= '{' { statement } [ expression ] '}'; (* Final expression is the return value *)
statement ::= letBinding | pipelineStatement | ifStatement | expressionStatement;

letBinding ::= 'let' IDENTIFIER '=' expression ';';
expressionStatement ::= expression ';';

ifStatement ::= 'if' expression blockExpression [ 'else' ( ifStatement | blockExpression ) ];

pipelineStatement ::= ( argumentList | expression ) '->' IDENTIFIER { '->' IDENTIFIER } [ '->' outputBinding ] ';';
argumentList ::= '(' [ namedArgument { ',' namedArgument } [','] ] ')';
namedArgument ::= IDENTIFIER '=' expression;
outputBinding ::= IDENTIFIER | '(' IDENTIFIER { ',' IDENTIFIER } [','] ')';

(* Expressions & Literals *)
expression     ::= comparison ;
comparison     ::= term ( ( '==' | '!=' | '<' | '<=' | '>' | '>=' ) term )* ;
term           ::= factor ( ( '+' | '-' ) factor )* ;
factor         ::= unary ( ( '*' | '/' ) unary )* ;
unary          ::= ( '!' | '-' ) unary | call ;
call           ::= primary ( '.' IDENTIFIER )* ; (* Field access, e.g., a.b.c *)

primary        ::= IDENTIFIER
               | literal
               | recordExpression
               | tupleExpression
               | '(' expression ')' (* Grouped expression *)
               ;

recordExpression ::= IDENTIFIER '{' [ namedArgument { ',' namedArgument } [','] ] '}';
tupleExpression ::= '(' expression ',' [ expression { ',' expression } [','] ] ')'; (* e.g., (a,) or (a,b,c) *)

literal ::= STRING_LITERAL | INTEGER_LITERAL | FLOAT_LITERAL | 'true' | 'false';

(* Lexical Tokens *)
IDENTIFIER ::= /* e.g., my_variable, MyType */
STRING_LITERAL ::= /* e.g., "hello world" */
INTEGER_LITERAL ::= /* e.g., 101, 42 */
FLOAT_LITERAL ::= /* e.g., 3.14, 0.5 */

```
