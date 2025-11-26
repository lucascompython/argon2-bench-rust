use std::hint::black_box;

use criterion::{Criterion, criterion_group};
use rand::TryRngCore;

fn hash_password(password: &[u8]) -> String {
    let salt = generate_salt();
    let config = rust_argon2::Config {
        variant: rust_argon2::Variant::Argon2id,
        version: rust_argon2::Version::Version13,
        mem_cost: 65536,
        time_cost: 4,
        lanes: 4,
        secret: &[],
        ad: &[],
        hash_length: 32,
        thread_mode: rust_argon2::ThreadMode::Parallel,
    };
    rust_argon2::hash_encoded(password, &salt, &config).unwrap()
}

fn verify_password(hash: &str, password: &[u8]) -> bool {
    rust_argon2::verify_encoded(hash, password).unwrap()
}

fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    let mut rng = rand::rngs::OsRng;
    // rng.fill_bytes(&mut salt);
    rng.try_fill_bytes(&mut salt).unwrap();
    salt
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("rust_argon2 hash password", |b| {
        b.iter(|| hash_password(black_box(b"password")))
    });
    let hash = hash_password(b"password");
    c.bench_function("rust_argon2 verify password", |b| {
        b.iter(|| verify_password(black_box(&hash), black_box(b"password")))
    });
}

criterion_group!(benches, criterion_benchmark);
