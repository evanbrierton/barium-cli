#![warn(clippy::pedantic)]

use anyhow::Ok;
use clap::Parser;
mod bars;
mod command;
mod execute;
mod files;
mod plates;

use crate::command::Command;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    execute::execute(args.command);
    Ok(())
}
