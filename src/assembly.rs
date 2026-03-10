/// Assembly intermediate representation (Chapter 1–2).
///
/// This mirrors the target x86-64 instructions we need.

/// A hardware register.
#[derive(Debug, Clone, PartialEq)]
pub enum Reg {
    AX,
    R10,
}

/// An assembly operand.
#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    /// An immediate integer value (e.g. $42).
    Imm(i64),
    /// A hardware register.
    Reg(Reg),
    /// A pseudo-register (temporary variable name, to be replaced later).
    Pseudo(String),
    /// A stack slot at an offset from %rbp.
    Stack(i32),
}

/// An assembly unary operator.
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
}

/// An assembly instruction.
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    /// movl src, dst
    Mov { src: Operand, dst: Operand },
    /// A unary operation (negl / notl) on a destination operand.
    Unary { op: UnaryOp, dst: Operand },
    /// Allocate stack space: subq $N, %rsp
    AllocateStack(i32),
    /// ret (with epilogue)
    Ret,
}

/// An assembly-level function definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub instructions: Vec<Instruction>,
}

/// An assembly-level program.
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub function: Function,
}
