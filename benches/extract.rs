use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn bench_extract(c: &mut Criterion) {
    let manifest = include_str!("../Cargo.toml")
        .parse::<toml::Value>()
        .expect("Failed to parse Cargo.toml manifest");

    macro_rules! extract {
        ($pattern:expr) => {
            black_box(cargo_extract::extract($pattern, &manifest).unwrap());
        };
    }

    c.bench_function("extract", |b| {
        b.iter(|| {
            extract!("package.name");
            extract!("package.version");
            extract!("package.authors");
            extract!("package.description");
            extract!("package.repository");
            extract!("package.categories");
        });
    });
}

criterion_group!(benches, bench_extract);
criterion_main!(benches);
