# Benchmark between different crates that provide password hashing in Rust

The crates being here benchmarked are:

- [argon2](https://crates.io/crates/argon2) - Rust Implementation
- [rust-argon2](https://crates.io/crates/rust-argon2) - Rust Implementation
- [argon2-kdf](https://crates.io/crates/argon2-kdf) - Bindings to the C implementation
- [scrpyt](https://crates.io/crates/scrypt) - Rust Implementation
- [bcrypt](https://crates.io/crates/bcrypt) - Rust Implementation

Argon2 parameters used in the benchmark:

- `m=65536`
- `t=4`
- `p=4`
- `password="password"`
- `salt=randomly generated with rand::rngs::OsRng`
- `salt_length=16`
- `hash_length=32`
- `version=13`
- `variant=Argon2id`

Scrypt parameters used in the benchmark:

- `log_n=16`
- `r=8`
- `p=1`

Bcrypt parameters used in the benchmark:

- `cost=10`

The benchmark is done using the [criterion](https://crates.io/crates/criterion) crate.  

## Results

`argon2-kdf` seems to be the fastest closely followed by `argon2`, you shold probably use the `argon2` crate, since it's the most supported one.  
These are the results of the benchmark on my machine AMD Ryzen 9 7950X (32) @ 5.88 GHz:

Rust Results:

```bash
argon2 hash password    time:   [25.174 ms 26.122 ms 27.159 ms]
argon2 verify password  time:   [24.874 ms 25.482 ms 26.178 ms]

rust_argon2 hash password  time:   [43.375 ms 43.566 ms 43.777 ms]
rust_argon2 verify password  time:   [43.815 ms 43.951 ms 44.087 ms]

argon2_kdf hash password  time:   [24.274 ms 24.397 ms 24.525 ms]
argon2_kdf verify password  time:   [24.592 ms 24.880 ms 25.214 ms]

scrypt hash password    time:   [102.15 ms 103.21 ms 104.33 ms]
scrpyt verify password  time:   [99.658 ms 99.804 ms 99.972 ms]

bcrypt hash password    time:   [36.898 ms 36.909 ms 36.922 ms]
bcrypt verify password  time:   [36.969 ms 36.998 ms 37.031 ms]
```

PHP Results for comparison:

```bash
Bcrypt hash elapsed time: 34.893989562988 ms
Bcrypt verify elapsed time: 34.30700302124 ms
Argon2id hash elapsed time: 36.846876144409 ms
Argon2id verify elapsed time: 37.317037582397 ms
```

## Running the benchmark

Just run `cargo bench` in the root of the project.
