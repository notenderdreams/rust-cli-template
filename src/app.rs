use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::{cmds::cmd_hello, error::CliError};

#[derive(Debug, Parser)]
#[command(name = "{{project-name}}", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Hello {
        #[arg(short, long)]
        name: Option<String>,
    },
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hello { name } => cmd_hello::run(name).map_err(CliError::from)?,
    }

    Ok(())
}