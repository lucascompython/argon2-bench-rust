use criterion::criterion_main;

mod benchmarks;

criterion_main!(
    benchmarks::argon2_bench::benches,
    benchmarks::rust_argon2_bench::benches,
    benchmarks::argon2_kdf_bench::benches
);
