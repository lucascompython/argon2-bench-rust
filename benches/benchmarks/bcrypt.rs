use std::hint::black_box;

use bcrypt::{hash, verify};
use criterion::{Criterion, criterion_group};

fn hash_password(password: &str) -> String {
    hash(password, 10).unwrap()
}

fn verify_password(hash: &str, password: &str) -> bool {
    verify(password, hash).unwrap()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bcrypt hash password", |b| {
        b.iter(|| hash_password(black_box("password")))
    });
    let hash = hash_password("password");
    c.bench_function("bcrypt verify password", |b| {
        b.iter(|| verify_password(black_box(&hash), black_box("password")))
    });
}

criterion_group!(benches, criterion_benchmark);
