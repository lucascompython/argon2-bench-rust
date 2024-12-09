use criterion::{black_box, criterion_group, Criterion};

fn hash_password(password: &[u8], salt: &[u8]) -> String {
    let config = rust_argon2::Config {
        variant: rust_argon2::Variant::Argon2id,
        version: rust_argon2::Version::Version13,
        mem_cost: 19456,
        time_cost: 2,
        lanes: 1,
        secret: &[],
        ad: &[],
        hash_length: 32,
    };
    rust_argon2::hash_encoded(password, salt, &config).unwrap()
}

fn verify_password(hash: &str, password: &[u8]) -> bool {
    rust_argon2::verify_encoded(hash, password).unwrap()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("rust_argon2 hash password", |b| {
        b.iter(|| hash_password(black_box(b"password"), black_box(b"randomsalt")))
    });
    let hash = hash_password(b"password", b"randomsalt");
    c.bench_function("rust_argon2 verify password", |b| {
        b.iter(|| verify_password(black_box(&hash), black_box(b"password")))
    });
}

criterion_group!(benches, criterion_benchmark);
