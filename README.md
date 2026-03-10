# lily 🌸

A C compiler written in Rust, following [*Writing a C Compiler*](https://norasandler.com/book/) by Nora Sandler.

## Current Status

**Chapter 2 — Unary Operators**

Supports programs with integer constants, unary negation (`-`), and bitwise complement (`~`):

```c
int main(void) {
    return ~(-42);
}
```

## Building

```sh
# Debug build
cargo build

# Optimized release build
cargo build --release
```

The release binary will be at `target/release/lily`.

## Usage

```sh
# Full compilation (preprocess → lex → parse → tacky → codegen → emit → assemble → link)
lily program.c

# Stop after lexing
lily --lex program.c

# Stop after parsing
lily --parse program.c

# Stop after TACKY IR generation
lily --tacky program.c

# Stop after code generation (no .s file emitted)
lily --codegen program.c

# Stop after emitting assembly (keep .s file, don't assemble/link)
lily -S program.c

# Keep intermediate .s file for debugging
lily --debug program.c

# Target a specific platform
lily -t linux program.c
lily -t osx program.c
```

> **Tip:** You can also use `cargo run -- <args>` during development instead of the `lily` binary directly.

## Project Structure

```
src/
├── main.rs               # CLI driver & pipeline orchestration
├── settings.rs            # Platform & stage enums, platform detection
├── tokens.rs              # Token type definitions
├── lexer.rs               # Source text → Vec<Token>
├── ast.rs                 # Abstract syntax tree types
├── parser.rs              # Tokens → AST (recursive descent)
├── tacky.rs               # TACKY three-address code IR types
├── tacky_gen.rs           # AST → TACKY IR
├── unique_ids.rs          # Temporary variable name generation
├── assembly.rs            # Assembly IR types
├── codegen.rs             # TACKY → Assembly IR
├── replace_pseudos.rs     # Pseudo-registers → stack slots
├── instruction_fixup.rs   # Fix invalid instructions, add stack setup
└── emit.rs                # Assembly IR → .s file (x86-64)
```

## Architecture

```
source.c
  │
  ▼  gcc -E -P (preprocess)
source.i
  │
  ▼  Lexer
Vec<Token>
  │
  ▼  Parser
  AST
  │
  ▼  TACKY Generator
TACKY IR  (three-address code with temporaries)
  │
  ▼  Codegen
Assembly IR  (with pseudo-registers)
  │
  ▼  Replace Pseudos
Assembly IR  (with stack slots)
  │
  ▼  Instruction Fixup
Assembly IR  (valid x86-64)
  │
  ▼  Emitter
source.s
  │
  ▼  gcc (assemble + link)
executable
```

## Documentation

- [Grammar & IR Specifications](docs/grammar.md) — Token definitions, grammar rules, AST, TACKY IR, and assembly IR
- [Architecture Details](docs/architecture.md) — Per-stage pipeline walkthrough

## Running Tests

```sh
cargo test
```

## Reference

- [*Writing a C Compiler*](https://norasandler.com/book/) by Nora Sandler
