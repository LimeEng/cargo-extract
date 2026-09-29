use crate::{ExtractResult, extract};
use clap::{Arg, ArgAction, ArgGroup, ArgMatches, Command};
use std::{fs, process};

const TARGET_TRIPLE: &str = env!("TARGET_TRIPLE");
const ARG_ACCESS_PATTERN: &str = "access_pattern";
const ARG_ARCHITECTURE: &str = "architecture";

pub fn command() -> Command {
    Command::new("extract")
        .about("Extract a value from Cargo.toml or print this executable's target triple")
        .arg(
            Arg::new(ARG_ARCHITECTURE)
                .long("arch")
                .help("Print the target triple this executable was built for")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(ARG_ACCESS_PATTERN)
                .help("Period-separated path of TOML keys and zero-based array indices"),
        )
        .group(
            ArgGroup::new("input")
                .required(true)
                .args([ARG_ACCESS_PATTERN, ARG_ARCHITECTURE]),
        )
}

/// Execute the extraction request selected by the parsed arguments.
pub fn execute(matches: &ArgMatches) {
    let pattern = matches.get_one::<String>(ARG_ACCESS_PATTERN);
    let arch_flag = matches.get_one::<bool>(ARG_ARCHITECTURE);

    if let Some(pattern) = pattern {
        handle_pattern(pattern);
    } else if arch_flag.is_some() {
        handle_arch();
    } else {
        process::exit(1);
    }
}

/// Read the current manifest, extract the requested value, and print the result.
fn handle_pattern(pattern: &str) {
    let manifest = read_cargo_toml().expect("Failed to find Cargo.toml");
    let manifest = toml::from_str(&manifest).expect("Failed to parse Cargo.toml manifest");
    match extract(pattern, &manifest) {
        Ok(extracted) => println!("{extracted}"),
        Err(err) => {
            println!("{err}");
            process::exit(1);
        }
    }
}

/// Print the target triple embedded when this executable was compiled.
fn handle_arch() {
    println!("{TARGET_TRIPLE}");
}

/// Read `Cargo.toml` from the process's current working directory.
fn read_cargo_toml() -> ExtractResult<String> {
    fs::read_to_string("Cargo.toml").map_err(|_| "Failed to open Cargo.toml".to_string())
}
