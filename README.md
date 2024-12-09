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

`argon2-kdf` seems to be the fastest.  
These are the results of the benchmark on my machine AMD Ryzen 9 7950X (32) @ 5.88 GHz:

Rust Results:

```bash
argon2 hash password    time:   [71.344 ms 71.489 ms 71.638 ms]

argon2 verify password  time:   [70.104 ms 70.234 ms 70.368 ms]

rust_argon2 hash password    time:   [128.73 ms 128.91 ms 129.14 ms]

rust_argon2 verify password    time:   [133.18 ms 133.37 ms 133.57 ms]

argon2_kdf hash password    time:   [23.596 ms 23.644 ms 23.693 ms]

argon2_kdf verify password    time:   [23.525 ms 23.594 ms 23.667 ms]

scrypt hash password    time:   [89.648 ms 89.763 ms 89.896 ms]

scrpyt verify password  time:   [90.840 ms 91.009 ms 91.185 ms]

bcrypt hash password    time:   [37.686 ms 37.711 ms 37.739 ms]

bcrypt verify password  time:   [37.660 ms 37.692 ms 37.733 ms]
```

PHP Results:

```bash
Bcrypt hash elapsed time: 34.893989562988 ms
Bcrypt verify elapsed time: 34.30700302124 ms
Argon2id hash elapsed time: 36.846876144409 ms
Argon2id verify elapsed time: 37.317037582397 ms
```

## Running the benchmark

Just run `cargo bench` in the root of the project.
