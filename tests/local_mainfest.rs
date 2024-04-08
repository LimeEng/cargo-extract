#[test]
fn local_cargo_manifest() {
    let manifest = include_str!("../Cargo.toml")
        .parse::<toml::Value>()
        .expect("Failed to parse Cargo.toml manifest");

    macro_rules! test {
        ($pattern:expr, $target:expr) => {
            let extracted = cargo_extract::extract($pattern, manifest.clone()).unwrap();
            assert_eq!(extracted, env!($target));
        };
    }

    test!("package.name", "CARGO_PKG_NAME");
    test!("package.version", "CARGO_PKG_VERSION");
    test!("package.authors", "CARGO_PKG_AUTHORS");
    test!("package.description", "CARGO_PKG_DESCRIPTION");
    test!("package.repository", "CARGO_PKG_REPOSITORY");
}
