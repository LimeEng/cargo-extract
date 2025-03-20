use cargo_extract::extract;
use indoc::indoc;

macro_rules! test {
    ($test_case:expr) => {
        let (pattern, output) = $test_case.split_once("\n").expect("Should not fail");
        let extracted = extract(pattern, cargo_toml()).expect("Should not fail");
        assert_eq!(extracted, output.trim());
    };
}

#[test]
fn values() {
    test!(indoc! {"
        basket.fruit
        lime
    "});
    test!(indoc! {"
        basket.flower
        rose
    "});
    test!(indoc! {"
        math.fibonacci.0
        0
    "});
    test!(indoc! {"
        math.fibonacci.9
        34
    "});
    test!(indoc! {"
        math.collatz.1.0
        1
    "});
    test!(indoc! {"
        math.collatz.2.6
        1
    "});
}

#[test]
fn tables() {
    test!(indoc! {"
        basket
        fruit = lime
        flower = rose
    "});
}

#[test]
fn arrays() {
    test!(indoc! {"
        math.fibonacci
        0
        1
        1
        2
        3
        5
        8
        13
        21
        34
    "});
    test!(indoc! {"
        math.power_of_two
        1
        2
        4
        8
        16
        32
        64
        128
        256
        512
    "});
    test!(indoc! {"
        math.collatz.0
        4
        2
        1
    "});
    test!(indoc! {"
        math.collatz.1
        1
    "});
}

fn cargo_toml() -> toml::Value {
    indoc! {r#"
        [basket]
        fruit = "lime"
        flower = "rose"

        [math]
        fibonacci = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
        power_of_two = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512]
        collatz = [[4, 2, 1], [1], [10, 5, 16, 8, 4, 2, 1], [2, 1], [16, 8, 4, 2, 1]]
    "#}
    .parse()
    .unwrap()
}
