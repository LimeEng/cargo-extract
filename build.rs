fn main() {
    let name = "TARGET_TRIPLE";
    let value = std::env::var("TARGET").unwrap();

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rustc-env={name}={value}");
}
