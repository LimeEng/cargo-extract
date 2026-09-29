const MANIFEST: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"));

#[test]
fn local_cargo_manifest() {
    let manifest = toml::from_str(MANIFEST).expect("Failed to parse Cargo.toml manifest");

    macro_rules! test {
        ($pattern:expr, $target:expr) => {
            let extracted = cargo_extract::extract($pattern, &manifest).unwrap();
            assert_eq!(extracted, env!($target));
        };
    }

    test!("package.name", "CARGO_PKG_NAME");
    test!("package.version", "CARGO_PKG_VERSION");
    test!("package.description", "CARGO_PKG_DESCRIPTION");
    test!("package.repository", "CARGO_PKG_REPOSITORY");
}
