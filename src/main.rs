
use sap_bot::cli::Cli;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    cli.run();
}
