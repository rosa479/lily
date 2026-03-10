/// The target platform for code generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Platform {
    Linux,
    #[value(name = "osx")]
    OsX,
}

/// Which stage of the compiler to stop after.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Lex,
    Parse,
    Codegen,
    Assembly,
    Executable,
}

/// Detect the current platform by running `uname`.
pub fn detect_platform() -> Platform {
    let output = std::process::Command::new("uname")
        .output()
        .expect("failed to run uname");
    let uname = String::from_utf8_lossy(&output.stdout).to_lowercase();
    if uname.starts_with("darwin") {
        Platform::OsX
    } else {
        Platform::Linux
    }
}
