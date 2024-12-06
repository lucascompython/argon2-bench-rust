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
argon2 hash password    time:   [31.585 ms 31.696 ms 31.824 ms]
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe

argon2 verify password  time:   [36.624 ms 36.869 ms 37.192 ms]
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe

rust_argon2 hash password
                        time:   [40.742 ms 40.888 ms 41.050 ms]
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe

rust_argon2 verify password
                        time:   [33.546 ms 33.791 ms 34.083 ms]
Found 14 outliers among 100 measurements (14.00%)
  4 (4.00%) high mild
  10 (10.00%) high severe

argon2_kdf hash password
                        time:   [9.5027 ms 9.5815 ms 9.6753 ms]
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) high mild
  12 (12.00%) high severe

argon2_kdf verify password
                        time:   [9.5352 ms 9.6227 ms 9.7253 ms]
Found 16 outliers among 100 measurements (16.00%)
  3 (3.00%) high mild
  13 (13.00%) high severe
```
