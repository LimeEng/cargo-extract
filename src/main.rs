use anyhow::Context;
use clap::{crate_version, Arg, Command};
use std::fs;
use toml::Table;

fn main() -> anyhow::Result<()> {
    let mut args: Vec<_> = std::env::args().collect();

    if let Some("extract") = args.get(1).map(String::as_ref) {
        args.remove(1);
    }
    let access_pattern_arg = "access_pattern";
    let matches = Command::new("cargo-extract")
        .version(crate_version!())
        .long_version(crate_version!())
        .about("Extracts information found in Cargo.toml")
        .arg(Arg::new(access_pattern_arg).required(true))
        .get_matches_from(args);

    let pattern = matches
        .get_one::<String>(access_pattern_arg)
        .expect("Argument required");

    let manifest = read_cargo_toml().expect("Failed to find Cargo.toml");
    let manifest = manifest
        .parse::<Table>()
        .expect("Failed to parse Cargo.toml manifest");
    match cargo_extract::extract(pattern, manifest) {
        Ok(extracted) => {
            println!("{extracted}");
            Ok(())
        }
        Err(err) => Err(err),
    }
}

pub fn read_cargo_toml() -> anyhow::Result<String> {
    fs::read_to_string("Cargo.toml").with_context(|| "Failed to open Cargo.toml".to_string())
}
