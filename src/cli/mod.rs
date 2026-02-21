use clap::{Command, crate_version};

mod extract;

pub fn run() {
    let app = app();
    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("extract", matches)) => extract::execute(matches),
        _ => unreachable!(),
    }
}

fn app() -> Command {
    Command::new("cargo")
        .bin_name("cargo")
        .version(crate_version!())
        .long_version(crate_version!())
        .about("Extracts information found in Cargo.toml")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(extract::command())
}
