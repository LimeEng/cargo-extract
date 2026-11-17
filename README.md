[![CI status](https://github.com/LimeEng/cargo-extract/actions/workflows/ci.yaml/badge.svg)](https://github.com/LimeEng/cargo-extract/actions/workflows/ci.yaml)
[![Latest version](https://img.shields.io/crates/v/cargo-extract.svg)](https://crates.io/crates/cargo-extract)

# cargo-extract

This cargo subcommand allows you to extract specific information from a `Cargo.toml` file.

## Installation

```sh
cargo install cargo-extract
```

## Usage

```sh
cargo extract <ACCESS_PATTERN>
cargo extract <ACCESS_PATTERN> --from <PATH>
cargo extract <ACCESS_PATTERN> --format <FORMAT>
```

An access pattern is a sequence of TOML table keys separated by periods. Array elements are selected with zero-based integer indices. For example:

```sh
$ cargo extract package.name
cargo-extract

$ cargo extract package.version
0.4.0

$ cargo extract package.categories
command-line-utilities
development-tools::build-utils
development-tools::cargo-plugins

$ cargo extract package.categories.0
command-line-utilities

$ cargo extract --from ../other-project/Cargo.toml package.name
other-project
```

The default format is `plain`, which prints strings without quotes and arrays
as newline-separated values. Use `--format json` for structured output:

```sh
$ cargo extract --format json package.version
"0.4.0"

$ cargo extract --format json package.categories
["command-line-utilities","development-tools::build-utils","development-tools::cargo-plugins"]
```

JSON preserves arrays, tables, strings, integers, finite floats, and booleans.
TOML dates and times become JSON strings in TOML's textual representation.
Non-finite floats (`inf`, `-inf`, and `nan`) cannot be represented in JSON and
produce an error, including when nested in an array or table.
