
//! Command Line Interface for binaries

use clap::{Parser, Subcommand, command};
use crate::processes::Test;

/// Command Line Interface arguments
#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    cmd: Option<Command>
}

/// Command enum, mostly for cli
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Remove Matlcons operations from orders
    #[command(name="matlcons")]
    RemoveMatlcons,

    /// Testing value
    Test
}

impl Command {
    /// run subcommand
    pub fn exec(&self) {
        match self {
            Self::RemoveMatlcons => (),
            Self::Test => Test::exec()
        }
    }
}

impl Cli {
    /// run cli
    pub fn run(&self) {
        self.cmd
            .as_ref()
            .unwrap_or(&Command::Test)
            .exec()
    }
}

