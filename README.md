# Benchmark between different crates that provide password hashing in Rust using Argon2

The crates being here benchmarked are:

- [argon2](https://crates.io/crates/argon2) - Rust Implementation
- [rust-argon2](https://crates.io/crates/rust-argon2) - Rust Implementation
- [argon2-kdf](https://crates.io/crates/argon2-kdf) - Bindings to the C implementation

They all have the same configuration, using the same parameters for the Argon2 algorithm. See below:

```rust
    let params = argon2::ParamsBuilder::new()
        .m_cost(19456)
        .t_cost(2)
        .p_cost(1)
        .output_len(32)
        .build()
```

The benchmark is done using the [criterion](https://crates.io/crates/criterion) crate.  

## Results

`argon2-kdf` seems to be the fastest.  
These are the results of the benchmark on my machine (Intel Core i3-1005G1 @ 3.40GHz):

```bash
argon2 hash password    time:   [36.586 ms 37.018 ms 37.511 ms]
Found 17 outliers among 100 measurements (17.00%)
  3 (3.00%) low severe
  1 (1.00%) low mild
  5 (5.00%) high mild
  8 (8.00%) high severe

argon2 verify password  time:   [31.532 ms 31.953 ms 32.457 ms]
Found 13 outliers among 100 measurements (13.00%)
  4 (4.00%) high mild
  9 (9.00%) high severe

rust_argon2 hash password
                        time:   [34.020 ms 34.166 ms 34.336 ms]
Found 16 outliers among 100 measurements (16.00%)
  4 (4.00%) high mild
  12 (12.00%) high severe

rust_argon2 verify password
                        time:   [33.730 ms 33.903 ms 34.110 ms]
Found 13 outliers among 100 measurements (13.00%)
  2 (2.00%) high mild
  11 (11.00%) high severe

argon2_kdf hash password
                        time:   [12.987 ms 13.049 ms 13.125 ms]
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe

argon2_kdf verify password
                        time:   [12.949 ms 13.043 ms 13.159 ms]
Found 12 outliers among 100 measurements (12.00%)
  2 (2.00%) high mild
  10 (10.00%) high severe
```

## Running the benchmark

Just run `cargo bench` in the root of the project.
