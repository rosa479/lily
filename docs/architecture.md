# Architecture

This document describes each stage of the lily compilation pipeline in detail.

---

## Pipeline Overview

```
source.c
  │
  ▼  Preprocess        gcc -E -P
source.i
  │
  ▼  Lex               lexer.rs
Vec<Token>
  │
  ▼  Parse             parser.rs
AST
  │
  ▼  TACKY Gen         tacky_gen.rs
TACKY IR
  │
  ▼  Codegen           codegen.rs
Assembly IR (pseudo-registers)
  │
  ▼  Replace Pseudos   replace_pseudos.rs
Assembly IR (stack slots)
  │
  ▼  Instruction Fixup instruction_fixup.rs
Assembly IR (valid x86-64)
  │
  ▼  Emit              emit.rs
source.s
  │
  ▼  Assemble & Link   gcc
executable
```

The driver (`main.rs`) orchestrates this pipeline and supports stopping at any stage via command-line flags (`--lex`, `--parse`, `--tacky`, `--codegen`, `-S`).

---

## Stage 1: Preprocessing

**File:** External (`gcc -E -P`)

The C preprocessor expands `#include` directives, macros, and conditional compilation. The `-P` flag suppresses `#line` markers. The output is written to a `.i` file.

Not implemented in lily — delegated to `gcc`.

---

## Stage 2: Lexing

**File:** `src/lexer.rs`

Converts a source string into a flat `Vec<Token>`.

**Approach:** Character-by-character scanning with no regex. The lexer:
1. Skips whitespace.
2. Matches identifiers/keywords: `[A-Za-z_][A-Za-z0-9_]*`, then checks a keyword table.
3. Matches integer constants: `[0-9]+`, rejecting numbers followed by identifier characters (e.g., `123abc`).
4. Matches multi-character operators using longest match (`--` before `-`).
5. Matches single-character punctuation.

**Error handling:** Returns `anyhow::Result` with descriptive `LexError` messages including position.

---

## Stage 3: Parsing

**File:** `src/parser.rs`

Converts `Vec<Token>` into an AST using recursive descent.

**Key design:**
- A `TokenStream` wrapper provides `peek()`, `take()`, and `expect()` operations over the token vector.
- Each grammar rule maps to one function (e.g., `parse_exp`, `parse_statement`, `parse_function`).
- `peek()` is used to decide which production to follow (e.g., constant vs. unary operator vs. parenthesized expression).
- `expect()` consumes a token and verifies it matches, producing clear error messages on mismatch.

**Error handling:** Returns `anyhow::Result` with `ParseError` messages like "Expected `;` but found `}`".

---

## Stage 4: TACKY Generation

**Files:** `src/tacky_gen.rs`, `src/tacky.rs`, `src/unique_ids.rs`

Converts the nested AST into a flat three-address code IR called TACKY.

**Why TACKY?** The AST has arbitrarily nested expressions (e.g., `~(-42)`). TACKY flattens them into a linear instruction sequence where each instruction does one thing, making code generation straightforward.

**How it works:**
- `emit_tacky_for_exp` recursively walks the AST expression tree.
- For each sub-expression, it returns:
  - The instructions needed to evaluate it.
  - The `Val` (constant or temporary) holding the result.
- Each unary operation creates a fresh temporary via `unique_ids::make_temporary()` (producing names like `tmp.0`, `tmp.1`).
- The dot in `tmp.N` ensures no collision with real C identifiers.

---

## Stage 5: Code Generation

**File:** `src/codegen.rs`

Converts TACKY IR into Assembly IR with pseudo-registers.

**Translation rules:**

| TACKY Instruction | Assembly Instructions |
|---|---|
| `Return(val)` | `Mov(val, Reg(AX))`, `Ret` |
| `Unary { op, src, dst }` | `Mov(src, dst)`, `Unary(op, dst)` |

- TACKY `Constant(n)` → Assembly `Imm(n)`
- TACKY `Var(name)` → Assembly `Pseudo(name)`
- TACKY `Complement` → Assembly `Not`
- TACKY `Negate` → Assembly `Neg`

At this point, the output uses pseudo-registers (unbounded named temporaries) rather than real hardware registers or stack slots.

---

## Stage 6: Replace Pseudo-Registers

**File:** `src/replace_pseudos.rs`

Assigns each pseudo-register a concrete stack slot.

**Algorithm:**
- Maintains a `HashMap<String, i32>` mapping pseudo names to stack offsets.
- Stack grows downward from `%rbp`: first pseudo gets `-4(%rbp)`, second gets `-8(%rbp)`, etc.
- Each operand in every instruction is examined and replaced if it's a `Pseudo`.
- Returns the modified program and the total stack space needed.

---

## Stage 7: Instruction Fixup

**File:** `src/instruction_fixup.rs`

Makes the Assembly IR valid for x86-64.

**Two fixes:**

1. **Stack allocation:** Prepends `AllocateStack(N)` to each function, which will emit `subq $N, %rsp`.

2. **Invalid `mov` rewrite:** x86-64 doesn't allow `movl` from memory to memory. If both operands are `Stack`, the instruction is split using `R10` as a scratch register:
   ```
   Mov(Stack(a), Stack(b))  →  Mov(Stack(a), Reg(R10)); Mov(Reg(R10), Stack(b))
   ```

---

## Stage 8: Emission

**File:** `src/emit.rs`

Writes Assembly IR as x86-64 AT&T syntax to a `.s` file.

**Function structure:**
```asm
    .globl <label>
<label>:
    pushq %rbp          # save old base pointer
    movq %rsp, %rbp     # set up new stack frame
    <instructions>
```

**Instruction mapping:**

| Assembly IR | x86-64 |
|---|---|
| `Mov { src, dst }` | `movl <src>, <dst>` |
| `Unary { Neg, dst }` | `negl <dst>` |
| `Unary { Not, dst }` | `notl <dst>` |
| `AllocateStack(N)` | `subq $N, %rsp` |
| `Ret` | `movq %rbp, %rsp; popq %rbp; ret` |

**Operand formatting:**

| Operand | AT&T Syntax |
|---|---|
| `Imm(42)` | `$42` |
| `Reg(AX)` | `%eax` |
| `Reg(R10)` | `%r10d` |
| `Stack(-4)` | `-4(%rbp)` |

**Platform differences:**
- **macOS:** Function labels are prefixed with `_` (e.g., `_main`).
- **Linux:** Labels are used as-is; a `.note.GNU-stack` section is emitted to prevent executable stack.

---

## Stage 9: Assembly & Linking

**External:** `gcc`

The emitted `.s` file is assembled and linked by `gcc` to produce the final executable. The `.s` file is cleaned up unless `--debug` is passed.
