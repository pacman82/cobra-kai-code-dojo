# Session 3 - Code Dojo

## The knights journey

A knight on a chessboard starts at the upper right corner (A1) and is moved each turn. It visits each fields on the board exactly once. Write a program which iterates over the possible solutions of the knights journey.

## Goals

While the last dojos our focus was on the way we write the code (TDD), this time we focus on how we order the code into modules (you are of course very welcome to do so in a TDD fashion ;-)). Performance is fun, but secondary.

Then cutting the problem logic into modules (not strictly modules in the Rust sense, but also other "units" seperating code, like functions, etc.) we want to focus them isolating knowledge, decisions and assumptions.

- Isolating knowledge implies that we can understand each module by itself, without understanding the entire program.
- Isolating decisions implies that we can change a descion by only changing that module and without refactoring the entire codebase.
- Isolating assumptions is closely related to we other two. "We assume the knight starts at A1" reads almost the same as "We know/decide the knight starts at A1". Therfore, unsurprisingly independently wether we would classify something as decision, knowledge or assumption these are all good candidates for isolating.

One artificial additional requirement for this code dojo: Please use some kind of backtracking algorithm to solve the problem. We want to generalize the solution in the next dojo, for other problems also solvable with backtracking (e.g. N-Queens, Sudoko, ...)

## Decisions in this Problem

Then solving the knights journey you face many design descions, all associated with tradeoffs, in terms of readability, performance, etc. rather than focusing on getting all these descions exactly right, the purpose of this exercise is to isolate them, and make them independently understandable and changable. Examples for these decisions are:

- How to represent the current position of the knight in memory (index?, coordinates?, bitmask?)
- How to keep track of already visited fields (history of moves?, boolean array?, bitmask?)
- How to generate possible moves for the knight
- Related to that: Isolating the knowledge of how a knight moves in chess.
- ...

Most importantly seperate the problem domain, from the backtracking solution we apply to it. This will be especially helpful in the next code dojo, if we want to reuse that code, on other problems.
