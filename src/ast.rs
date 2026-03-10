/// Abstract Syntax Tree for the C compiler (Chapter 1–2).

/// A unary operator.
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Complement, // ~
    Negate,     // -
}

/// An expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Exp {
    Constant(i64),
    Unary(UnaryOp, Box<Exp>),
}

/// A statement.
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Return(Exp),
}

/// A function definition.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDefinition {
    pub name: String,
    pub body: Statement,
}

/// A complete program (a single function for now).
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub function: FunctionDefinition,
}
