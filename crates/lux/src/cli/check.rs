use std::{io::stdin, process::ExitCode};

use anyhow::{Context, Result};
use clap::Parser;

use lux::Runtime;

use super::utils::files::discover_script_path_including_lux_dirs;

/// Check a script for syntax errors
#[derive(Debug, Clone, Parser)]
pub struct CheckCommand {
    /// Script name or full path to the file to check
    pub(super) script_path: String,
}

impl CheckCommand {
    pub async fn run(self) -> Result<ExitCode> {
        // Create a new Lux runtime
        let rt = Runtime::new()?;

        let mut contents = Vec::new();
        let name;

        if self.script_path == "-" {
            name = "stdin".to_string();
            std::io::Read::read_to_end(&mut stdin(), &mut contents)
                .context("Failed to read script contents from stdin")?;
        } else {
            let file_path = discover_script_path_including_lux_dirs(&self.script_path)?;
            name = format!("@{}", file_path.display());
            contents = async_fs::read(&file_path)
                .await
                .with_context(|| format!("Failed to read file at path \"{}\"", file_path.display()))?;
        }

        // Strip shebang if present
        if contents.starts_with(b"#!") {
             if let Some(idx) = contents.iter().position(|x| *x == b'\n') {
                 contents.drain(..idx).for_each(drop);
             }
        }

        // Check syntax
        match rt.check(&name, contents) {
            Ok(_) => {
                println!("Syntax OK");
                Ok(ExitCode::SUCCESS)
            }
            Err(e) => {
                eprintln!("Syntax Error: {}", e);
                Ok(ExitCode::FAILURE)
            }
        }
    }
}
