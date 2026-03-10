/// TACKY intermediate representation (Chapter 2).
///
/// A three-address code IR that sits between the AST and assembly.
/// Each instruction operates on at most one operation with explicit
/// source and destination temporaries.

/// A unary operator in TACKY (same names as AST, but separate type).
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Complement,
    Negate,
}

/// A TACKY value — either an integer constant or a temporary variable.
#[derive(Debug, Clone, PartialEq)]
pub enum Val {
    Constant(i64),
    Var(String),
}

/// A TACKY instruction.
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Return(Val),
    Unary {
        op: UnaryOp,
        src: Val,
        dst: Val,
    },
}

/// A TACKY function definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub body: Vec<Instruction>,
}

/// A TACKY program.
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub function: Function,
}
