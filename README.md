# lily 🌸

A C compiler written in Rust

## Current Status

**Chapter 1** — Compiles programs of the form:
```c
int main(void) {
    return <integer>;
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
# Full compilation (preprocess → lex → parse → codegen → emit → assemble → link)
lily program.c

# Stop after lexing
lily --lex program.c

# Stop after parsing
lily --parse program.c

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
├── main.rs        # CLI driver & pipeline orchestration
├── settings.rs    # Platform & stage enums, platform detection
├── tokens.rs      # Token type definitions
├── lexer.rs       # Source text → tokens
├── ast.rs         # Abstract syntax tree types
├── parser.rs      # Tokens → AST (recursive descent)
├── assembly.rs    # Assembly IR types
├── codegen.rs     # AST → Assembly IR
└── emit.rs        # Assembly IR → .s file (x86-64)
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
  ▼  Codegen
Assembly IR
  │
  ▼  Emitter
source.s
  │
  ▼  gcc (assemble + link)
executable
```

## Running Tests

```sh
cargo test
```

## Reference

- [*Writing a C Compiler*](https://norasandler.com/book/) by Nora Sandler
