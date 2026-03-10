use std::io::Write;

use anyhow::Result;

use crate::assembly::{Function, Instruction, Operand, Program};
use crate::settings::Platform;

/// Format an operand as an x86-64 assembly string.
fn format_operand(op: &Operand) -> String {
    match op {
        Operand::Imm(val) => format!("${}", val),
        Operand::Register => "%eax".to_string(),
    }
}

/// Format a label name, prepending "_" on macOS.
fn format_label(name: &str, platform: Platform) -> String {
    match platform {
        Platform::OsX => format!("_{}", name),
        Platform::Linux => name.to_string(),
    }
}

/// Emit a single instruction to the writer.
fn emit_instruction<W: Write>(w: &mut W, instr: &Instruction) -> Result<()> {
    match instr {
        Instruction::Mov { src, dst } => {
            writeln!(w, "\tmovl {}, {}", format_operand(src), format_operand(dst))?;
        }
        Instruction::Ret => {
            writeln!(w, "\tret")?;
        }
    }
    Ok(())
}

/// Emit a function definition.
fn emit_function<W: Write>(w: &mut W, func: &Function, platform: Platform) -> Result<()> {
    let label = format_label(&func.name, platform);
    writeln!(w)?;
    writeln!(w, "\t.globl {}", label)?;
    writeln!(w, "{}:", label)?;
    for instr in &func.instructions {
        emit_instruction(w, instr)?;
    }
    Ok(())
}

/// Emit the GNU stack note (Linux only, prevents executable stack).
fn emit_stack_note<W: Write>(w: &mut W, platform: Platform) -> Result<()> {
    if platform == Platform::Linux {
        writeln!(w, "\t.section .note.GNU-stack,\"\",@progbits")?;
    }
    Ok(())
}

/// Emit the full assembly program to a file.
pub fn emit(output_path: &str, program: &Program, platform: Platform) -> Result<()> {
    let mut file = std::fs::File::create(output_path)?;
    emit_function(&mut file, &program.function, platform)?;
    emit_stack_note(&mut file, platform)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembly;

    #[test]
    fn emit_linux_return_42() {
        let program = assembly::Program {
            function: assembly::Function {
                name: "main".to_string(),
                instructions: vec![
                    assembly::Instruction::Mov {
                        src: assembly::Operand::Imm(42),
                        dst: assembly::Operand::Register,
                    },
                    assembly::Instruction::Ret,
                ],
            },
        };
        let mut buf = Vec::new();
        emit_function(&mut buf, &program.function, Platform::Linux).unwrap();
        emit_stack_note(&mut buf, Platform::Linux).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains(".globl main"));
        assert!(output.contains("main:"));
        assert!(output.contains("movl $42, %eax"));
        assert!(output.contains("ret"));
        assert!(output.contains(".note.GNU-stack"));
    }
}
