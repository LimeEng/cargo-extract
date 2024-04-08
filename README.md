[![CI status](https://github.com/LimeEng/cargo-extract/actions/workflows/ci.yaml/badge.svg)](https://github.com/LimeEng/cargo-extract/actions/workflows/ci.yaml)
[![Latest version](https://img.shields.io/crates/v/cargo-extract.svg)](https://crates.io/crates/cargo-extract)

# cargo-extract

This cargo subcommand allows you to extract specific information from a `Cargo.toml` file.

## Installation

```
$ cargo install cargo-extract
```

## Examples

```
$ cargo extract package.name
cargo-extract

$ cargo extract package.version
0.2.0

$ cargo extract package.categories
command-line-utilities
development-tools::build-utils
development-tools::cargo-plugins

$ cargo extract package.categories.0
command-line-utilities
```
