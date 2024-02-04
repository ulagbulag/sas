# SAS

[![SAS logo](https://raw.githubusercontent.com/ulagbulag/sas/master/assets/logo.webp)](https://crates.io/crates/sas)

[![SAS crate](https://img.shields.io/crates/v/sas.svg)](https://crates.io/crates/sas)
[![SAS documentation](https://docs.rs/sas/badge.svg)](https://docs.rs/sas)

SAS (Salty-And-Sweet) is an one-line Rust runtime optimization library.

## Features

- NUMA-aware [`rayon`](https://docs.rs/rayon): `numa` feature should be enabled
  - If you have 1 NUMA nodes, you can experience about 20% performance improvements if the tasks are completed at approximately equal times.
  - If you have 2+ NUMA nodes, you can experience extreme performance improvements.

## Install

### Simple one-shot mode

```rust
fn main() {
    // That's end!
    sas::init();

    // ... your heavy works
}
```

### Advanced mode

TBD

## Benchmarks

This sections describes the benchmark results. It uses default package features (zero-touch).

Note that the benchmark metrics are sensitive to process, so please benchmark **each metric separately!**

The lower elapsed time is the better.

| Machine             | OS (in Docker) | Kernel                       | Metric    | Elapsed time (OFF)                | Elapsed time (ON)           | Performance Boost |
| ------------------- | -------------- | ---------------------------- | --------- | --------------------------------- | --------------------------- | ----------------- |
| HP Z440 Workstation | Arch Linux     | 6.7.0-arch3-1                | rayon_sum | 150,826 ns/iter (+/- 61,270)      | 80,341 ns/iter (+/- 6,926)  | 1.88x             |
| NVIDIA DGX-2        | Ubuntu 22.04   | 5.14.0-284.11.1.el9_2.x86_64 | rayon_sum | 5,175,824 ns/iter (+/- 2,767,386) | 247,236 ns/iter (+/- 6,151) | 20.93x            |

## License

SAS is distributed under the terms of both the MIT license and the
Apache License (Version 2.0). See [LICENSE-APACHE](LICENSE-APACHE) and
[LICENSE-MIT](LICENSE-MIT) for details. Opening a pull request is
assumed to signal agreement with these licensing terms.
