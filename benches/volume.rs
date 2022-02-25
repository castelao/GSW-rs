use criterion::{black_box, criterion_group, criterion_main, Criterion};

use gsw::volume::{enthalpy_diff, specvol};

fn bench(c: &mut Criterion) {
    c.bench_function("specvol", |b| {
        b.iter(|| specvol(black_box(10.0), black_box(10.0), black_box(10.0)).unwrap())
    });

    c.bench_function("enthalpy_diff", |b| {
        b.iter(|| {
            enthalpy_diff(
                black_box(33.0),
                black_box(10.0),
                black_box(0.0),
                black_box(100.0),
            )
            .unwrap()
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
