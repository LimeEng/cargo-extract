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
```

An access pattern is a sequence of TOML table keys separated by periods. Array elements are selected with zero-based integer indices. For example:

```sh
$ cargo extract package.name
cargo-extract

$ cargo extract package.version
0.3.4

$ cargo extract package.categories
command-line-utilities
development-tools::build-utils
development-tools::cargo-plugins

$ cargo extract package.categories.0
command-line-utilities

$ cargo extract --from ../other-project/Cargo.toml package.name
other-project
```
