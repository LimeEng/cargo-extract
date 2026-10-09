use cargo_extract::{ExtractResult, extract};
use clap::{Arg, ArgMatches, Command};
use std::{fs, process};

const ARG_ACCESS_PATTERN: &str = "access_pattern";
const ARG_FROM: &str = "from";
const DEFAULT_MANIFEST_PATH: &str = "Cargo.toml";

pub fn command() -> Command {
    Command::new("extract")
        .about("Extract a value from Cargo.toml")
        .arg(
            Arg::new(ARG_FROM)
                .long("from")
                .value_name("PATH")
                .help("Path of the Cargo.toml file to extract from"),
        )
        .arg(
            Arg::new(ARG_ACCESS_PATTERN)
                .help("Period-separated path of TOML keys and zero-based array indices")
                .required(true),
        )
}

/// Execute the extraction request selected by the parsed arguments.
pub fn execute(matches: &ArgMatches) {
    let pattern = matches
        .get_one::<String>(ARG_ACCESS_PATTERN)
        .expect("Access pattern is required");
    let manifest_path = matches
        .get_one::<String>(ARG_FROM)
        .map(String::as_str)
        .unwrap_or(DEFAULT_MANIFEST_PATH);

    match extract_from_manifest(pattern, manifest_path) {
        Ok(extracted) => println!("{extracted}"),
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    }
}

/// Load and parse the manifest at the given path, then extract the requested value.
fn extract_from_manifest(pattern: &str, manifest_path: &str) -> ExtractResult<String> {
    let manifest = fs::read_to_string(manifest_path)
        .map_err(|err| format!("Failed to open {manifest_path}: {err}"))?;
    let manifest = toml::from_str(&manifest)
        .map_err(|err| format!("Failed to parse {manifest_path}: {err}"))?;
    extract(pattern, &manifest)
}
