# Grammar & IR Specifications

This document defines the formal grammar, token set, and intermediate representations used by the lily compiler at each stage.

---

## Tokens

The lexer produces the following token types:

| Token | Pattern | Example |
|-------|---------|---------|
| `Identifier(String)` | `[A-Za-z_][A-Za-z0-9_]*` | `main`, `foo` |
| `Constant(i64)` | `[0-9]+` | `42`, `0` |
| `KWInt` | `int` | |
| `KWReturn` | `return` | |
| `KWVoid` | `void` | |
| `OpenParen` | `(` | |
| `CloseParen` | `)` | |
| `OpenBrace` | `{` | |
| `CloseBrace` | `}` | |
| `Semicolon` | `;` | |
| `Hyphen` | `-` | |
| `DoubleHyphen` | `--` | |
| `Tilde` | `~` | |

**Lexing rules:**
- Whitespace is skipped.
- Keywords take priority over identifiers (e.g., `int` is `KWInt`, not `Identifier("int")`).
- `--` is matched before `-` (longest match).
- A number immediately followed by an identifier character (e.g., `123abc`) is a lex error.

---

## Grammar

The parser implements a recursive descent parser for the following grammar (BNF):

### Chapter 1 — Return an Integer

```
<program>   ::= <function>
<function>  ::= "int" <identifier> "(" "void" ")" "{" <statement> "}"
<statement> ::= "return" <exp> ";"
<exp>       ::= <int>
```

### Chapter 2 — Unary Operators

```
<program>   ::= <function>
<function>  ::= "int" <identifier> "(" "void" ")" "{" <statement> "}"
<statement> ::= "return" <exp> ";"
<exp>       ::= <int> | <unop> <exp> | "(" <exp> ")"
<unop>      ::= "-" | "~"
```

**Precedence:** Unary operators are right-associative and bind tighter than any binary operator (none yet). Parentheses override precedence.

**Examples:**
```c
return 42;          // Constant
return -5;          // Negate(Constant(5))
return ~0;          // Complement(Constant(0))
return ~(-42);      // Complement(Negate(Constant(42)))
return -(-10);      // Negate(Negate(Constant(10)))
```

---

## Abstract Syntax Tree (AST)

The parser produces the following AST types:

```
Program
 └─ FunctionDefinition { name: String, body: Statement }

Statement
 └─ Return(Exp)

Exp
 ├─ Constant(i64)
 └─ Unary(UnaryOp, Box<Exp>)

UnaryOp
 ├─ Complement    (~)
 └─ Negate        (-)
```

### Example

Source: `int main(void) { return ~(-42); }`

```
Program
 └─ Function "main"
     └─ Return
         └─ Unary(Complement)
             └─ Unary(Negate)
                 └─ Constant(42)
```

---

## TACKY IR (Three-Address Code)

The TACKY generator flattens the nested AST into a linear sequence of instructions, each operating on at most one operation with explicit temporaries.

### Types

```
Val
 ├─ Constant(i64)
 └─ Var(String)         // e.g., "tmp.0", "tmp.1"

Instruction
 ├─ Return(Val)
 └─ Unary { op: UnaryOp, src: Val, dst: Val }

UnaryOp
 ├─ Complement
 └─ Negate

Function { name: String, body: Vec<Instruction> }
Program  { function: Function }
```

### Example

Source: `return ~(-42);`

```
Unary { op: Negate,     src: Constant(42),  dst: Var("tmp.0") }
Unary { op: Complement, src: Var("tmp.0"),  dst: Var("tmp.1") }
Return(Var("tmp.1"))
```

Each nested sub-expression gets its own temporary variable. The result is a flat list that's straightforward to translate to assembly.

---

## Assembly IR

The assembly IR goes through three phases:

### Phase 1: After Codegen (with pseudo-registers)

```
Operand
 ├─ Imm(i64)           // immediate: $42
 ├─ Reg(AX)            // %eax
 ├─ Reg(R10)           // %r10d (scratch register)
 └─ Pseudo(String)     // pseudo-register: "tmp.0"

Instruction
 ├─ Mov { src, dst }
 ├─ Unary { op, dst }
 └─ Ret

UnaryOp
 ├─ Neg                // negl
 └─ Not                // notl
```

### Phase 2: After Replace Pseudos (with stack slots)

Pseudo-registers are replaced with stack offsets:

```
Operand
 ├─ Imm(i64)           // $42
 ├─ Reg(AX)            // %eax
 ├─ Reg(R10)           // %r10d
 └─ Stack(i32)         // -4(%rbp), -8(%rbp), ...
```

Each unique pseudo gets a 4-byte slot growing downward from `%rbp`.

### Phase 3: After Instruction Fixup (valid x86-64)

Two fixes are applied:

1. **`AllocateStack(N)`** is prepended → `subq $N, %rsp`
2. **`Mov(Stack, Stack)`** is split into two instructions using `R10` as a scratch register:
   ```
   Mov(Stack(a), Stack(b))
   →
   Mov(Stack(a), Reg(R10))
   Mov(Reg(R10), Stack(b))
   ```

### Example

Source: `return -5;`

**After Codegen:**
```
Mov(Imm(5), Pseudo("tmp.0"))
Unary(Neg, Pseudo("tmp.0"))
Mov(Pseudo("tmp.0"), Reg(AX))
Ret
```

**After Replace Pseudos:**
```
Mov(Imm(5), Stack(-4))
Unary(Neg, Stack(-4))
Mov(Stack(-4), Reg(AX))
Ret
```

**After Fixup:**
```
AllocateStack(4)
Mov(Imm(5), Stack(-4))
Unary(Neg, Stack(-4))
Mov(Stack(-4), Reg(AX))
Ret
```

**Emitted x86-64:**
```asm
    .globl main
main:
    pushq %rbp
    movq %rsp, %rbp
    subq $4, %rsp
    movl $5, -4(%rbp)
    negl -4(%rbp)
    movl -4(%rbp), %eax

    movq %rbp, %rsp
    popq %rbp
    ret
```
