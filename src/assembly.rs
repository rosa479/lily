/// Assembly intermediate representation (Chapter 1).
///
/// This mirrors the target x86-64 instructions we need for chapter 1.

/// An assembly operand.
#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    /// An immediate integer value (e.g. $42).
    Imm(i64),
    /// The %eax register.
    Register,
}

/// An assembly instruction.
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    /// movl src, dst
    Mov { src: Operand, dst: Operand },
    /// ret
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
