mod assembly;
mod ast;
mod codegen;
mod emit;
mod instruction_fixup;
mod lexer;
mod parser;
mod replace_pseudos;
mod settings;
mod tacky;
mod tacky_gen;
mod tokens;
mod unique_ids;

use std::path::Path;
use std::process::Command;

use anyhow::{bail, Result};
use clap::Parser;

use settings::{Platform, Stage};

/// lily — a C compiler written in Rust
#[derive(Parser, Debug)]
#[command(name = "lily", version, about)]
struct Cli {
    /// The C source file to compile
    source: String,

    /// Run the lexer only
    #[arg(long)]
    lex: bool,

    /// Run the lexer and parser only
    #[arg(long)]
    parse: bool,

    /// Run through TACKY generation but stop before codegen
    #[arg(long)]
    tacky: bool,

    /// Run through code generation but stop before emitting assembly
    #[arg(long)]
    codegen: bool,

    /// Stop before assembling (keep .s file)
    #[arg(short = 'S')]
    assembly: bool,

    /// Target platform
    #[arg(short = 't', long = "target", value_enum)]
    target: Option<Platform>,

    /// Keep intermediate files for debugging
    #[arg(long)]
    debug: bool,
}

impl Cli {
    /// Determine which stage to stop after based on the flags.
    fn stage(&self) -> Stage {
        if self.lex {
            Stage::Lex
        } else if self.parse {
            Stage::Parse
        } else if self.tacky {
            Stage::Tacky
        } else if self.codegen {
            Stage::Codegen
        } else if self.assembly {
            Stage::Assembly
        } else {
            Stage::Executable
        }
    }
}

/// Run an external command; bail on failure.
fn run_command(program: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(program).args(args).status()?;
    if !status.success() {
        bail!("Command failed: {} {:?}", program, args);
    }
    Ok(())
}

/// Replace the extension of a file path.
fn replace_extension(path: &str, new_ext: &str) -> String {
    let stem = Path::new(path).file_stem().unwrap().to_str().unwrap();
    let parent = Path::new(path)
        .parent()
        .map(|p| p.to_str().unwrap())
        .unwrap_or("");
    if parent.is_empty() {
        format!("{}{}", stem, new_ext)
    } else {
        format!("{}/{}{}", parent, stem, new_ext)
    }
}

/// Step 1: Preprocess the C source file using gcc -E -P.
fn preprocess(src: &str) -> Result<String> {
    let ext = Path::new(src)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    if ext != "c" && ext != "h" {
        bail!("Expected a C source file with .c or .h extension");
    }
    let output = replace_extension(src, ".i");
    run_command("gcc", &["-E", "-P", src, "-o", &output])?;
    Ok(output)
}

/// Step 2: Run the compiler pipeline up to the requested stage.
fn compile(stage: Stage, preprocessed_src: &str, platform: Platform) -> Result<()> {
    let source = std::fs::read_to_string(preprocessed_src)?;

    // Lex
    let tokens = lexer::lex(&source)?;
    if stage == Stage::Lex {
        return Ok(());
    }

    // Parse
    let ast = parser::parse(tokens)?;
    if stage == Stage::Parse {
        return Ok(());
    }

    // TACKY generation (AST → TACKY IR)
    let tacky_prog = tacky_gen::generate(&ast);
    if stage == Stage::Tacky {
        return Ok(());
    }

    // Code generation (TACKY → Assembly IR with pseudo-registers)
    let asm_ast = codegen::generate(&tacky_prog);

    // Replace pseudo-registers with stack slots
    let (asm_ast, stack_size) = replace_pseudos::replace_pseudos(&asm_ast);

    // Fix up invalid instructions and prepend AllocateStack
    let asm_ast = instruction_fixup::fixup_program(&asm_ast, stack_size);

    if stage == Stage::Codegen {
        return Ok(());
    }

    // Emit assembly to .s file
    let asm_filename = replace_extension(preprocessed_src, ".s");
    emit::emit(&asm_filename, &asm_ast, platform)?;

    Ok(())
}

/// Step 3: Assemble and link the .s file into an executable using gcc.
fn assemble_and_link(src: &str, cleanup: bool) -> Result<()> {
    let assembly_file = replace_extension(src, ".s");
    let output_file = Path::new(src)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let output_path = if let Some(parent) = Path::new(src).parent() {
        if parent.to_str().unwrap_or("").is_empty() {
            output_file
        } else {
            format!("{}/{}", parent.display(), output_file)
        }
    } else {
        output_file
    };
    run_command("gcc", &[&assembly_file, "-o", &output_path])?;
    if cleanup {
        std::fs::remove_file(&assembly_file)?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let platform = cli.target.unwrap_or_else(settings::detect_platform);
    let stage = cli.stage();

    // Preprocess
    let preprocessed = preprocess(&cli.source)?;

    // Compile (lex → parse → codegen → emit)
    compile(stage, &preprocessed, platform)?;

    // Clean up preprocessed file
    let _ = std::fs::remove_file(&preprocessed);

    // Assemble and link if going all the way
    if stage == Stage::Executable {
        assemble_and_link(&cli.source, !cli.debug)?;
    }

    Ok(())
}
