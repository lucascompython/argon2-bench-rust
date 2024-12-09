use criterion::{black_box, criterion_group, Criterion};
use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Params, Scrypt,
};

fn hash_password(password: &[u8]) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let params = Params::new(16, 8, 1, 32).unwrap();
    let password_hash = Scrypt
        .hash_password_customized(password, None, None, params, &salt)
        .unwrap()
        .to_string();
    password_hash
}

fn verify_password(hash: &str, password: &[u8]) -> bool {
    let parsed_hash = PasswordHash::new(hash).unwrap();
    Scrypt.verify_password(password, &parsed_hash).is_ok()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("scrypt hash password", |b| {
        b.iter(|| hash_password(black_box(b"password")))
    });
    let hash = hash_password(b"password");
    c.bench_function("scrpyt verify password", |b| {
        b.iter(|| verify_password(black_box(&hash), black_box(b"password")))
    });
}

criterion_group!(benches, criterion_benchmark);
