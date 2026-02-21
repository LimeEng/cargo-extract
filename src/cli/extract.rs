use crate::ExtractResult;
use clap::{Arg, ArgAction, ArgGroup, ArgMatches, Command};
use std::{fs, process};

const ARG_ACCESS_PATTERN: &str = "access_pattern";
const ARG_ARCHITECTURE: &str = "architecture";

pub fn command() -> Command {
    Command::new("extract")
        .arg(
            Arg::new(ARG_ARCHITECTURE)
                .long("arch")
                .action(ArgAction::SetTrue),
        )
        .arg(Arg::new(ARG_ACCESS_PATTERN))
        .group(
            ArgGroup::new("input")
                .required(true)
                .args([ARG_ACCESS_PATTERN, ARG_ARCHITECTURE]),
        )
}

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

fn handle_pattern(pattern: &str) {
    let manifest = read_cargo_toml().expect("Failed to find Cargo.toml");
    let manifest = toml::from_str(&manifest).expect("Failed to parse Cargo.toml manifest");
    match crate::extract(pattern, &manifest) {
        Ok(extracted) => println!("{extracted}"),
        Err(err) => {
            println!("{err}");
            process::exit(1);
        }
    }
}

fn handle_arch() {
    println!("{}", env!("TARGET_TRIPLE"));
}

fn read_cargo_toml() -> ExtractResult<String> {
    fs::read_to_string("Cargo.toml").map_err(|_| "Failed to open Cargo.toml".to_string())
}
