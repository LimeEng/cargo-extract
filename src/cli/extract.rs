use cargo_extract::{ExtractResult, extract};
use clap::{Arg, ArgAction, ArgGroup, ArgMatches, Command};
use std::{fs, process};

const TARGET_TRIPLE: &str = env!("TARGET_TRIPLE");
const ARG_ACCESS_PATTERN: &str = "access_pattern";
const ARG_ARCHITECTURE: &str = "architecture";
const ARG_FROM: &str = "from";
const DEFAULT_MANIFEST_PATH: &str = "Cargo.toml";

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
            Arg::new(ARG_FROM)
                .long("from")
                .value_name("PATH")
                .help("Path of the Cargo.toml file to extract from")
                .conflicts_with(ARG_ARCHITECTURE),
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
    let manifest_path = matches
        .get_one::<String>(ARG_FROM)
        .map(String::as_str)
        .unwrap_or(DEFAULT_MANIFEST_PATH);

    if let Some(pattern) = pattern {
        handle_pattern(pattern, manifest_path);
    } else if arch_flag.is_some() {
        handle_arch();
    } else {
        process::exit(1);
    }
}

/// Read the selected manifest, extract the requested value, and print the result.
fn handle_pattern(pattern: &str, manifest_path: &str) {
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

/// Print the target triple embedded when this executable was compiled.
fn handle_arch() {
    println!("{TARGET_TRIPLE}");
}
