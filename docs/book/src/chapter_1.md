# Chapter 1: Introduction

Welcome to Kāra, a programming language designed for clarity, performance, and the explicit flow of data.

## The Core Idea: Programs as a Narrative

At its heart, Kāra is built on a simple philosophy: a program should tell a story. It should describe not just *what* to do, but *how* data moves through the system from its origin to its destination. It treats data as the protagonist of the story, and actions as the events that transform it.

To achieve this, Kāra is built around a few core concepts:

1.  **`Record`**: These are the "characters" in our story. They are simple, passive data structures that define the shape of the information we will be working with.

2.  **`Sūtra`**: A "thread" of logic. A `Sūtra` is a reusable function or procedure that takes data, performs a series of actions on it, and returns a result. It contains the detailed, step-by-step implementation of a task.

3.  **`flow`**: This is the main narrative of the program. It orchestrates the high-level story, showing how data records are created and passed to different `Sūtras` to achieve the final goal.

## Two Syntaxes, One Philosophy

Kāra has a unique design that provides two different syntaxes, each tailored for a specific job:

*   **The Orchestration Syntax (`flow` blocks):** For the high-level view, the syntax is verbose and self-documenting. It reads like a set of instructions.

    ```rust
    Action: CalculateDistance
      From: p1 = origin
      From: p2 = target
      Into: final_distance
    ```

*   **The Implementation Syntax (`Sūtra` bodies):** For writing the actual logic, the syntax is dense, efficient, and uses a `->` operator to show the explicit flow of data from one operation to the next.

    ```rust
    (p1.x, p2.x) -> Subtract -> dx
    ```

Both syntaxes are built on the exact same principle: **`Source -> Action -> Destination`**. This consistency is the key to Kāra's design.

In the following chapters, we will walk you through building your first Kāra program, exploring each of these concepts in detail. You will learn how to define your own data types, write powerful, reusable logic, and compose it all into a clear and maintainable program.
