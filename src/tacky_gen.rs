/// TACKY generation: AST → TACKY IR (Chapter 2).
///
/// Converts nested AST expressions into a flat sequence of three-address
/// instructions using temporary variables.

use crate::ast;
use crate::tacky;
use crate::unique_ids;

/// Convert an AST unary operator to a TACKY unary operator.
fn convert_op(op: &ast::UnaryOp) -> tacky::UnaryOp {
    match op {
        ast::UnaryOp::Complement => tacky::UnaryOp::Complement,
        ast::UnaryOp::Negate => tacky::UnaryOp::Negate,
    }
}

/// Emit TACKY instructions for an expression.
///
/// Returns the list of instructions needed to evaluate the expression,
/// and the TACKY value holding the result.
fn emit_tacky_for_exp(exp: &ast::Exp) -> (Vec<tacky::Instruction>, tacky::Val) {
    match exp {
        // A constant needs no instructions — it's already a value.
        ast::Exp::Constant(c) => (vec![], tacky::Val::Constant(*c)),

        // A unary expression:
        //   1. Evaluate the inner expression
        //   2. Create a fresh temporary for the result
        //   3. Emit a Unary instruction
        ast::Exp::Unary(op, inner) => {
            let (mut instructions, src) = emit_tacky_for_exp(inner);
            let dst_name = unique_ids::make_temporary();
            let dst = tacky::Val::Var(dst_name);
            let tacky_op = convert_op(op);
            instructions.push(tacky::Instruction::Unary {
                op: tacky_op,
                src,
                dst: dst.clone(),
            });
            (instructions, dst)
        }
    }
}

/// Emit TACKY instructions for a statement.
fn emit_tacky_for_statement(stmt: &ast::Statement) -> Vec<tacky::Instruction> {
    match stmt {
        ast::Statement::Return(exp) => {
            let (mut instructions, val) = emit_tacky_for_exp(exp);
            instructions.push(tacky::Instruction::Return(val));
            instructions
        }
    }
}

/// Emit TACKY for a function definition.
fn emit_tacky_for_function(func: &ast::FunctionDefinition) -> tacky::Function {
    let instructions = emit_tacky_for_statement(&func.body);
    tacky::Function {
        name: func.name.clone(),
        body: instructions,
    }
}

/// Generate TACKY IR from an AST program.
pub fn generate(program: &ast::Program) -> tacky::Program {
    tacky::Program {
        function: emit_tacky_for_function(&program.function),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tacky_gen_nested_unary() {
        unique_ids::reset();
        // ~(-42)
        let program = ast::Program {
            function: ast::FunctionDefinition {
                name: "main".to_string(),
                body: ast::Statement::Return(ast::Exp::Unary(
                    ast::UnaryOp::Complement,
                    Box::new(ast::Exp::Unary(
                        ast::UnaryOp::Negate,
                        Box::new(ast::Exp::Constant(42)),
                    )),
                )),
            },
        };
        let tacky_prog = generate(&program);
        assert_eq!(
            tacky_prog.function.body,
            vec![
                tacky::Instruction::Unary {
                    op: tacky::UnaryOp::Negate,
                    src: tacky::Val::Constant(42),
                    dst: tacky::Val::Var("tmp.0".to_string()),
                },
                tacky::Instruction::Unary {
                    op: tacky::UnaryOp::Complement,
                    src: tacky::Val::Var("tmp.0".to_string()),
                    dst: tacky::Val::Var("tmp.1".to_string()),
                },
                tacky::Instruction::Return(tacky::Val::Var("tmp.1".to_string())),
            ]
        );
    }
}
