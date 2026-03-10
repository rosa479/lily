/// Replace pseudo-registers with stack slots (Chapter 2).
///
/// After codegen, instructions reference pseudo-registers (temporary variable
/// names). This pass assigns each pseudo a unique stack slot relative to %rbp.

use std::collections::HashMap;

use crate::assembly::*;

/// State tracking stack slot assignments.
struct ReplacementState {
    /// The last used stack offset (grows negative from 0).
    current_offset: i32,
    /// Map from pseudo-register name to its stack offset.
    offset_map: HashMap<String, i32>,
}

impl ReplacementState {
    fn new() -> Self {
        Self {
            current_offset: 0,
            offset_map: HashMap::new(),
        }
    }

    /// Replace a pseudo-register with a stack slot, assigning a new slot if needed.
    fn replace_operand(&mut self, operand: &Operand) -> Operand {
        match operand {
            Operand::Pseudo(name) => {
                if let Some(&offset) = self.offset_map.get(name) {
                    Operand::Stack(offset)
                } else {
                    self.current_offset -= 4;
                    let offset = self.current_offset;
                    self.offset_map.insert(name.clone(), offset);
                    Operand::Stack(offset)
                }
            }
            other => other.clone(),
        }
    }

    /// Replace pseudos in a single instruction.
    fn replace_in_instruction(&mut self, instr: &Instruction) -> Instruction {
        match instr {
            Instruction::Mov { src, dst } => {
                let new_src = self.replace_operand(src);
                let new_dst = self.replace_operand(dst);
                Instruction::Mov {
                    src: new_src,
                    dst: new_dst,
                }
            }
            Instruction::Unary { op, dst } => {
                let new_dst = self.replace_operand(dst);
                Instruction::Unary {
                    op: op.clone(),
                    dst: new_dst,
                }
            }
            Instruction::Ret => Instruction::Ret,
            Instruction::AllocateStack(_) => {
                panic!("Internal error: AllocateStack shouldn't be present at this point")
            }
        }
    }
}

/// Replace all pseudo-registers in a function with stack slots.
///
/// Returns the updated function and the total stack size needed (as a positive value).
fn replace_pseudos_in_function(func: &Function) -> (Function, i32) {
    let mut state = ReplacementState::new();
    let instructions: Vec<Instruction> = func
        .instructions
        .iter()
        .map(|instr| state.replace_in_instruction(instr))
        .collect();
    let fixed_func = Function {
        name: func.name.clone(),
        instructions,
    };
    // current_offset is negative; negate it to get the total stack size
    (fixed_func, -state.current_offset)
}

/// Replace pseudo-registers in an entire program.
///
/// Returns the updated program and the stack size needed.
pub fn replace_pseudos(program: &Program) -> (Program, i32) {
    let (fixed_func, stack_size) = replace_pseudos_in_function(&program.function);
    (
        Program {
            function: fixed_func,
        },
        stack_size,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_pseudos_basic() {
        let program = Program {
            function: Function {
                name: "main".to_string(),
                instructions: vec![
                    Instruction::Mov {
                        src: Operand::Imm(5),
                        dst: Operand::Pseudo("tmp.0".to_string()),
                    },
                    Instruction::Unary {
                        op: UnaryOp::Neg,
                        dst: Operand::Pseudo("tmp.0".to_string()),
                    },
                    Instruction::Mov {
                        src: Operand::Pseudo("tmp.0".to_string()),
                        dst: Operand::Reg(Reg::AX),
                    },
                    Instruction::Ret,
                ],
            },
        };
        let (fixed, stack_size) = replace_pseudos(&program);
        assert_eq!(stack_size, 4);
        assert_eq!(
            fixed.function.instructions,
            vec![
                Instruction::Mov {
                    src: Operand::Imm(5),
                    dst: Operand::Stack(-4),
                },
                Instruction::Unary {
                    op: UnaryOp::Neg,
                    dst: Operand::Stack(-4),
                },
                Instruction::Mov {
                    src: Operand::Stack(-4),
                    dst: Operand::Reg(Reg::AX),
                },
                Instruction::Ret,
            ]
        );
    }
}
