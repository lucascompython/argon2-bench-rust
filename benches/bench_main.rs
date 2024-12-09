use criterion::criterion_main;

mod benchmarks;

criterion_main!(
    benchmarks::argon2::benches,
    benchmarks::rust_argon2::benches,
    benchmarks::argon2_kdf::benches
);
