use cargo_extract::ExtractResult;
use clap::{Arg, ArgAction, ArgGroup, Command, crate_version};
use std::{fs, process};

fn main() {
    let access_pattern_arg = "access_pattern";
    let architecture_arg = "architecture";
    let matches = Command::new("cargo")
        .bin_name("cargo")
        .version(crate_version!())
        .long_version(crate_version!())
        .about("Extracts information found in Cargo.toml")
        .subcommand_required(true)
        .subcommand(
            Command::new("extract")
                .arg(
                    Arg::new(architecture_arg)
                        .long("arch")
                        .action(ArgAction::SetTrue),
                )
                .arg(Arg::new(access_pattern_arg))
                .group(
                    ArgGroup::new("input")
                        .required(true)
                        .args([access_pattern_arg, architecture_arg]),
                ),
        )
        .get_matches();

    let Some(("extract", matches)) = matches.subcommand() else {
        unreachable!()
    };

    let pattern = matches.get_one::<String>(access_pattern_arg);
    let arch_flag = matches.get_one::<bool>(architecture_arg);

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

fn handle_arch() {
    println!("{}", env!("TARGET_TRIPLE"));
}

pub fn read_cargo_toml() -> ExtractResult<String> {
    fs::read_to_string("Cargo.toml").map_err(|_| "Failed to open Cargo.toml".to_string())
}
