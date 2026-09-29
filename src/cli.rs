//! Command-line interface for the `cargo extract` subcommand.

use clap::Command;

mod extract;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Parse command-line arguments and dispatch to the selected subcommand.
pub fn run() {
    let app = app();
    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("extract", matches)) => extract::execute(matches),
        _ => unreachable!(),
    }
}

/// Define the top-level Cargo subcommand and its available commands.
fn app() -> Command {
    Command::new("cargo")
        .bin_name("cargo")
        .version(VERSION)
        .long_version(VERSION)
        .about("Extracts information found in Cargo.toml")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(extract::command())
}
