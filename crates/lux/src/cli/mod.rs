use std::{env::args_os, process::ExitCode};

use anyhow::Result;
use clap::{Parser, Subcommand};

pub(crate) mod build;
pub(crate) mod check;
pub(crate) mod list;
pub(crate) mod repl;
pub(crate) mod run;
pub(crate) mod setup;
pub(crate) mod utils;

pub use self::{
    build::BuildCommand, check::CheckCommand, list::ListCommand, repl::ReplCommand, run::RunCommand,
    setup::SetupCommand,
};

#[derive(Debug, Clone, Subcommand)]
pub enum CliSubcommand {
    Run(RunCommand),
    Check(CheckCommand),
    List(ListCommand),
    Setup(SetupCommand),
    Build(BuildCommand),
    Repl(ReplCommand),
}

impl Default for CliSubcommand {
    fn default() -> Self {
        Self::Repl(ReplCommand::default())
    }
}

/// Lux - The Micro-Kernel Luau Runtime with Native FFI
#[derive(Parser, Debug, Default, Clone)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[clap(short = 'e', long = "eval")]
    pub eval: Option<String>,

    #[clap(subcommand)]
    subcommand: Option<CliSubcommand>,
}

impl Cli {
    pub fn new() -> Self {
        // TODO: Figure out if there is a better way to do this using clap ... ?
        // https://github.com/lux-runtime/lux/issues/253
        if args_os()
            .nth(1)
            .is_some_and(|arg| arg.eq_ignore_ascii_case("run"))
        {
            let Some(script_path) = args_os()
                .nth(2)
                .and_then(|arg| arg.to_str().map(String::from))
            else {
                return Self::parse(); // Will fail and return the help message
            };

            let script_args = args_os()
                .skip(3)
                .filter_map(|arg| arg.to_str().map(String::from))
                .collect::<Vec<_>>();

            Self {
                eval: None,
                subcommand: Some(CliSubcommand::Run(RunCommand {
                    script_path,
                    script_args,
                })),
            }
        } else {
            Self::parse()
        }
    }

    pub async fn run(self) -> Result<ExitCode> {
        if let Some(code) = self.eval {
            let mut rt = lux::Runtime::new()?;
            let result = rt.run_custom("eval", code.as_bytes()).await?;
            return Ok(ExitCode::from(result.status()));
        }

        match self.subcommand.unwrap_or_default() {
            CliSubcommand::Run(cmd) => cmd.run().await,
            CliSubcommand::Check(cmd) => cmd.run().await,
            CliSubcommand::List(cmd) => cmd.run().await,
            CliSubcommand::Setup(cmd) => cmd.run().await,
            CliSubcommand::Build(cmd) => cmd.run().await,
            CliSubcommand::Repl(cmd) => cmd.run().await,
        }
    }
}
