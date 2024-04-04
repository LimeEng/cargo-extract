use toml::Table;

#[test]
fn test_local_cargo_manifest() {
    let manifest = include_str!("../Cargo.toml");
    let manifest = manifest
        .parse::<Table>()
        .expect("Failed to parse Cargo.toml manifest");

    macro_rules! test {
        ($pattern:expr, $target:expr) => {
            let target = env!($target);
            let extracted = cargo_extract::extract($pattern, manifest.clone()).unwrap();
            assert_eq!(extracted, target);
        };
    }

    test!("package.name", "CARGO_PKG_NAME");
    test!("package.version", "CARGO_PKG_VERSION");
    test!("package.authors", "CARGO_PKG_AUTHORS");
    test!("package.description", "CARGO_PKG_DESCRIPTION");
    test!("package.repository", "CARGO_PKG_REPOSITORY");
}
