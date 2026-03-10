/// Code generation: TACKY IR → Assembly IR (Chapter 2).
///
/// Converts TACKY three-address code into assembly instructions,
/// using pseudo-registers that will be replaced with stack slots later.

use crate::assembly;
use crate::tacky;

/// Convert a TACKY value to an assembly operand.
fn convert_val(val: &tacky::Val) -> assembly::Operand {
    match val {
        tacky::Val::Constant(i) => assembly::Operand::Imm(*i),
        tacky::Val::Var(name) => assembly::Operand::Pseudo(name.clone()),
    }
}

/// Convert a TACKY unary operator to an assembly unary operator.
fn convert_op(op: &tacky::UnaryOp) -> assembly::UnaryOp {
    match op {
        tacky::UnaryOp::Complement => assembly::UnaryOp::Not,
        tacky::UnaryOp::Negate => assembly::UnaryOp::Neg,
    }
}

/// Convert a single TACKY instruction to one or more assembly instructions.
fn convert_instruction(instr: &tacky::Instruction) -> Vec<assembly::Instruction> {
    match instr {
        tacky::Instruction::Return(val) => {
            let asm_val = convert_val(val);
            vec![
                assembly::Instruction::Mov {
                    src: asm_val,
                    dst: assembly::Operand::Reg(assembly::Reg::AX),
                },
                assembly::Instruction::Ret,
            ]
        }
        tacky::Instruction::Unary { op, src, dst } => {
            let asm_op = convert_op(op);
            let asm_src = convert_val(src);
            let asm_dst = convert_val(dst);
            vec![
                assembly::Instruction::Mov {
                    src: asm_src,
                    dst: asm_dst.clone(),
                },
                assembly::Instruction::Unary {
                    op: asm_op,
                    dst: asm_dst,
                },
            ]
        }
    }
}

/// Convert a TACKY function to an assembly function.
fn convert_function(func: &tacky::Function) -> assembly::Function {
    let instructions: Vec<assembly::Instruction> = func
        .body
        .iter()
        .flat_map(convert_instruction)
        .collect();
    assembly::Function {
        name: func.name.clone(),
        instructions,
    }
}

/// Generate assembly IR from a TACKY program.
pub fn generate(program: &tacky::Program) -> assembly::Program {
    assembly::Program {
        function: convert_function(&program.function),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codegen_return_42() {
        let program = tacky::Program {
            function: tacky::Function {
                name: "main".to_string(),
                body: vec![tacky::Instruction::Return(tacky::Val::Constant(42))],
            },
        };
        let asm = generate(&program);
        assert_eq!(asm.function.name, "main");
        assert_eq!(
            asm.function.instructions,
            vec![
                assembly::Instruction::Mov {
                    src: assembly::Operand::Imm(42),
                    dst: assembly::Operand::Reg(assembly::Reg::AX),
                },
                assembly::Instruction::Ret,
            ]
        );
    }

    #[test]
    fn codegen_unary_negate() {
        let program = tacky::Program {
            function: tacky::Function {
                name: "main".to_string(),
                body: vec![
                    tacky::Instruction::Unary {
                        op: tacky::UnaryOp::Negate,
                        src: tacky::Val::Constant(5),
                        dst: tacky::Val::Var("tmp.0".to_string()),
                    },
                    tacky::Instruction::Return(tacky::Val::Var("tmp.0".to_string())),
                ],
            },
        };
        let asm = generate(&program);
        assert_eq!(
            asm.function.instructions,
            vec![
                assembly::Instruction::Mov {
                    src: assembly::Operand::Imm(5),
                    dst: assembly::Operand::Pseudo("tmp.0".to_string()),
                },
                assembly::Instruction::Unary {
                    op: assembly::UnaryOp::Neg,
                    dst: assembly::Operand::Pseudo("tmp.0".to_string()),
                },
                assembly::Instruction::Mov {
                    src: assembly::Operand::Pseudo("tmp.0".to_string()),
                    dst: assembly::Operand::Reg(assembly::Reg::AX),
                },
                assembly::Instruction::Ret,
            ]
        );
    }
}
