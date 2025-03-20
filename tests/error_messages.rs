use cargo_extract::extract;
use indoc::indoc;

macro_rules! test {
    ($test_case:expr) => {
        let (pattern, output) = $test_case.split_once("\n").expect("Should not fail");
        let extracted = extract(pattern, &cargo_toml()).expect_err("Should fail");
        assert_eq!(extracted, output.trim());
    };
}

#[test]
fn tables() {
    test!(indoc! {"
        nonexistent
        nonexistent
        ^ No such property [nonexistent]
    "});

    test!(indoc! {"
        nonexistent.something
        nonexistent.something
        ^ No such property [nonexistent]
    "});

    test!(indoc! {"
        basket.nonexistent
        basket.nonexistent
               ^ No such property [nonexistent]
    "});

    test!(indoc! {"
        basket.nonexistent.something
        basket.nonexistent.something
               ^ No such property [nonexistent]
    "});

    test!(indoc! {"
        basket.fruit.nope
        basket.fruit.nope
                     ^ No such property [nope]
    "});

    test!(indoc! {"
        basket.fruit.nope.also_no
        basket.fruit.nope.also_no
                     ^ No such property [nope]
    "});
}

#[test]
fn arrays() {
    test!(indoc! {"
        math.fibonacci.-1
        math.fibonacci.-1
                       ^ Not an array index [-1]
    "});
    test!(indoc! {"
        math.fibonacci.10
        math.fibonacci.10
                       ^ Array index out of bounds [10]
    "});
    test!(indoc! {"
        math.collatz.0.-1
        math.collatz.0.-1
                       ^ Not an array index [-1]
    "});
    test!(indoc! {"
        math.collatz.0.3
        math.collatz.0.3
                       ^ Array index out of bounds [3]
    "});
    test!(indoc! {"
        math.collatz.5.0
        math.collatz.5.0
                     ^ Array index out of bounds [5]
    "});
    test!(indoc! {"
        math.collatz.nonexistent.0
        math.collatz.nonexistent.0
                     ^ Not an array index [nonexistent]
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
