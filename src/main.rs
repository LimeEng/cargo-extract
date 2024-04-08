use cargo_extract::ExtractResult;
use clap::{crate_version, Arg, Command};
use std::{fs, process};

fn main() {
    let access_pattern_arg = "access_pattern";
    let matches = Command::new("cargo")
        .bin_name("cargo")
        .version(crate_version!())
        .long_version(crate_version!())
        .about("Extracts information found in Cargo.toml")
        .subcommand_required(true)
        .subcommand(Command::new("extract").arg(Arg::new(access_pattern_arg).required(true)))
        .get_matches();

    let Some(("extract", matches)) = matches.subcommand() else {
        unreachable!()
    };

    let pattern = matches
        .get_one::<String>(access_pattern_arg)
        .expect("Argument required");

    let manifest = read_cargo_toml().expect("Failed to find Cargo.toml");
    let manifest = manifest
        .parse::<toml::Value>()
        .expect("Failed to parse Cargo.toml manifest");

    match cargo_extract::extract(pattern, manifest) {
        Ok(extracted) => println!("{extracted}"),
        Err(err) => {
            println!("{err}");
            process::exit(1);
        }
    }
}

pub fn read_cargo_toml() -> ExtractResult<String> {
    fs::read_to_string("Cargo.toml").map_err(|_| "Failed to open Cargo.toml".to_string())
}
