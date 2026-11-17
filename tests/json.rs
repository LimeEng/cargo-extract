use cargo_extract::{OutputFormat, extract};
use indoc::indoc;
use serde_json::json;

#[test]
fn formats_extracted_values() {
    let manifest = cargo_toml();
    assert_eq!(
        extract("basket", &manifest, OutputFormat::Plain).unwrap(),
        "fruit = lime\nflower = rose"
    );
    let json: serde_json::Value =
        serde_json::from_str(&extract("basket", &manifest, OutputFormat::Json).unwrap()).unwrap();
    assert_eq!(json, json!({"fruit": "lime", "flower": "rose"}));
}

#[test]
fn json_values() {
    let manifest = cargo_toml();
    for (path, expected) in [
        ("string", json!("line\n\"quoted\"")),
        ("integer", json!(42)),
        ("float", json!(1.5)),
        ("boolean", json!(true)),
        ("date", json!("1979-05-27T07:32:00Z")),
        ("array", json!(["one", 2, {"enabled": true}])),
        ("array.2", json!({"enabled": true})),
    ] {
        let output = extract(path, &manifest, OutputFormat::Json).expect("Should not fail");
        let json: serde_json::Value = serde_json::from_str(&output).expect("Should be valid JSON");
        assert_eq!(json, expected, "{path}");
    }
}

#[test]
fn json_rejects_non_finite_floats_at_any_depth() {
    let manifest = cargo_toml();
    let paths = [
        "values.non_finite.0",
        "values.non_finite.1",
        "values.non_finite.2",
        "values.non_finite.3",
        "values.non_finite",
        "values",
    ];
    for path in paths {
        let error = extract(path, &manifest, OutputFormat::Json).expect_err("Should fail");
        assert!(error.contains("non-finite float"), "{path}: {error}");
    }
}

fn cargo_toml() -> toml::Value {
    let manifest = indoc! {r#"
        string = "line\n\"quoted\""
        integer = 42
        float = 1.5
        boolean = true
        date = 1979-05-27T07:32:00Z
        array = ["one", 2, { enabled = true }]

        [basket]
        fruit = "lime"
        flower = "rose"

        [values]
        non_finite = [inf, -inf, +inf, nan]
    "#};

    toml::from_str(manifest).unwrap()
}
