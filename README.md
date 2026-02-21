[![CI status](https://github.com/LimeEng/cargo-extract/actions/workflows/ci.yaml/badge.svg)](https://github.com/LimeEng/cargo-extract/actions/workflows/ci.yaml)
[![Latest version](https://img.shields.io/crates/v/cargo-extract.svg)](https://crates.io/crates/cargo-extract)

# cargo-extract

This cargo subcommand allows you to extract specific information from a `Cargo.toml` file.

## Installation

```sh
cargo install cargo-extract
```

## Examples

```sh
$ cargo extract package.name
cargo-extract

$ cargo extract package.version
0.3.2

$ cargo extract package.categories
command-line-utilities
development-tools::build-utils
development-tools::cargo-plugins

$ cargo extract package.categories.0
command-line-utilities

$ cargo extract --arch
x86_64-pc-windows-msvc
```
