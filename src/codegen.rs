use crate::assembly;
use crate::ast;

/// Convert an AST expression to an assembly operand.
fn convert_exp(exp: &ast::Exp) -> assembly::Operand {
    match exp {
        ast::Exp::Constant(val) => assembly::Operand::Imm(*val),
    }
}

/// Convert an AST statement to a list of assembly instructions.
fn convert_statement(stmt: &ast::Statement) -> Vec<assembly::Instruction> {
    match stmt {
        ast::Statement::Return(exp) => {
            let src = convert_exp(exp);
            vec![
                assembly::Instruction::Mov {
                    src,
                    dst: assembly::Operand::Register,
                },
                assembly::Instruction::Ret,
            ]
        }
    }
}

/// Convert an AST function definition to an assembly function.
fn convert_function(func: &ast::FunctionDefinition) -> assembly::Function {
    assembly::Function {
        name: func.name.clone(),
        instructions: convert_statement(&func.body),
    }
}

/// Generate assembly IR from an AST program.
pub fn generate(program: &ast::Program) -> assembly::Program {
    assembly::Program {
        function: convert_function(&program.function),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codegen_return_42() {
        let program = ast::Program {
            function: ast::FunctionDefinition {
                name: "main".to_string(),
                body: ast::Statement::Return(ast::Exp::Constant(42)),
            },
        };
        let asm = generate(&program);
        assert_eq!(asm.function.name, "main");
        assert_eq!(
            asm.function.instructions,
            vec![
                assembly::Instruction::Mov {
                    src: assembly::Operand::Imm(42),
                    dst: assembly::Operand::Register,
                },
                assembly::Instruction::Ret,
            ]
        );
    }
}
