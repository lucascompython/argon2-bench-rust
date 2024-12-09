use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use criterion::{black_box, criterion_group, Criterion};

fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let params = argon2::ParamsBuilder::new()
        .m_cost(65536)
        .t_cost(4)
        .p_cost(1)
        .output_len(32)
        .build()
        .unwrap();
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
    let hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();
    hash.to_string()
}

fn verify_password(hash: &str, password: &str) -> bool {
    let argon2 = Argon2::default();
    let hash = PasswordHash::new(&hash).unwrap();
    argon2.verify_password(password.as_bytes(), &hash).is_ok()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("argon2 hash password", |b| {
        b.iter(|| hash_password(black_box("password123")))
    });
    let hash = hash_password("password123");
    c.bench_function("argon2 verify password", |b| {
        b.iter(|| verify_password(black_box(&hash), black_box("password123")))
    });
}

criterion_group!(benches, criterion_benchmark);
