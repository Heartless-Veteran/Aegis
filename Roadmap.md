# Aegis Project Roadmap (rev. 8)

This document outlines the development plan for the Aegis language. The check-marks (`[X]`) indicate features that have a working prototype in the `v0.1.0-alpha` release.

## Version 0.1: The Scribe & Architect (Lexing & Parsing) - ✅ COMPLETE
* [X] Finalize the complete Aegis grammar specification in EBNF.
* [X] Implement the Scribe (lexer).
* [X] Implement the Architect (parser) with Pratt parsing.

## Version 0.2: The Guardian (Semantic Analysis) - ✅ COMPLETE
* [X] Implement the scoped Symbol Table.
* [X] Implement name resolution.
* [X] Implement the foundational `contract` system.
* [X] Implement initial type checking for expressions.
* [X] Implement analysis of typed function definitions.

## Version 0.3: The Engine (Initial Codegen) - ✅ COMPLETE (Prototype)
* [X] Implement the full pipeline for the Android target.
* [X] Implement high-performance list rendering (`for` loops compiling to `RecyclerView`).
* [X] Implement the JavaScript Bridge for interoperability.

## Version 0.4: The Forge (Tooling) - ✅ COMPLETE (Prototype)
* [X] Implement the `aegis` CLI with analysis commands.
* [X] Implement workflow commands (`aegis new`, `aegis build`).
* [ ] Implement the test runner (`aegis test`).

## Next Up: The Grand Unification & Hardening
* [ ] Implement function calls in the Engine.
* [ ] Implement the `async/await` feature in the Engine.
* [ ] Begin work on the Language Server Protocol (LSP) for IDE integration.
