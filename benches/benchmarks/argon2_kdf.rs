use std::str::FromStr;

use argon2_kdf::Hasher;
use criterion::{black_box, criterion_group, Criterion};

fn hash_password(password: &[u8]) -> String {
    Hasher::new()
        .algorithm(argon2_kdf::Algorithm::Argon2id)
        .salt_length(16)
        .iterations(4)
        .memory_cost_kib(65536)
        .hash_length(32)
        .threads(4)
        .hash(password)
        .unwrap()
        .to_string()
}

fn verify_password(hash: &str, password: &[u8]) -> bool {
    let hash = argon2_kdf::Hash::from_str(hash).unwrap();
    hash.verify(password)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("argon2_kdf hash password", |b| {
        b.iter(|| hash_password(black_box(b"password")))
    });
    let hash = hash_password(b"password");
    c.bench_function("argon2_kdf verify password", |b| {
        b.iter(|| verify_password(black_box(&hash), black_box(b"password")))
    });
}

criterion_group!(benches, criterion_benchmark);
