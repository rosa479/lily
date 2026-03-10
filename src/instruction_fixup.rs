/// Instruction fixup pass (Chapter 2).
///
/// After pseudo-register replacement, some instructions may be invalid
/// (e.g., mov from stack to stack). This pass:
/// 1. Prepends an AllocateStack instruction for the function prologue.
/// 2. Rewrites invalid mov(stack, stack) using R10 as a scratch register.

use crate::assembly::*;

/// Fix up a single instruction.
///
/// A `mov` from a stack slot to a stack slot is invalid on x86-64,
/// so we split it into two instructions using R10 as a scratch register.
fn fixup_instruction(instr: &Instruction) -> Vec<Instruction> {
    match instr {
        Instruction::Mov {
            src: src @ Operand::Stack(_),
            dst: dst @ Operand::Stack(_),
        } => vec![
            Instruction::Mov {
                src: src.clone(),
                dst: Operand::Reg(Reg::R10),
            },
            Instruction::Mov {
                src: Operand::Reg(Reg::R10),
                dst: dst.clone(),
            },
        ],
        other => vec![other.clone()],
    }
}

/// Fix up an entire function:
/// - Prepend `AllocateStack(stack_size)` to set up the stack frame.
/// - Rewrite any invalid instructions.
fn fixup_function(func: &Function, stack_size: i32) -> Function {
    let mut instructions = vec![Instruction::AllocateStack(stack_size)];
    for instr in &func.instructions {
        instructions.extend(fixup_instruction(instr));
    }
    Function {
        name: func.name.clone(),
        instructions,
    }
}

/// Fix up the entire program.
pub fn fixup_program(program: &Program, stack_size: i32) -> Program {
    Program {
        function: fixup_function(&program.function, stack_size),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixup_stack_to_stack_mov() {
        let program = Program {
            function: Function {
                name: "main".to_string(),
                instructions: vec![
                    Instruction::Mov {
                        src: Operand::Stack(-4),
                        dst: Operand::Stack(-8),
                    },
                    Instruction::Ret,
                ],
            },
        };
        let fixed = fixup_program(&program, 8);
        assert_eq!(
            fixed.function.instructions,
            vec![
                Instruction::AllocateStack(8),
                Instruction::Mov {
                    src: Operand::Stack(-4),
                    dst: Operand::Reg(Reg::R10),
                },
                Instruction::Mov {
                    src: Operand::Reg(Reg::R10),
                    dst: Operand::Stack(-8),
                },
                Instruction::Ret,
            ]
        );
    }
}
