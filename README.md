# SAS

[![SAS crate](https://img.shields.io/crates/v/sas.svg)](https://crates.io/crates/sas)
[![SAS documentation](https://docs.rs/sas/badge.svg)](https://docs.rs/sas)

SAS (Salty-And-Sweet) is an one-line Rust runtime optimization library.

## Install

### Simple one-shot mode

```rust
use sas::init_once;

fn main() {
    ::sas::init().expect("SAS not inited");

    // ...
}
```

### Advanced mode

TBD

## Benchmarks

This sections describes the benchmark results. It uses default package features (zero-touch).

Note that the benchmark metrics are sensitive to process, so please benchmark **each metric separately!**

The lower elapsed time is the better.

| Machine      | OS           | Metric    | Elapsed time (ON)                 | Elapsed time (OFF)           | Performance Boost |
| ------------ | ------------ | --------- | --------------------------------- | ---------------------------- | ----------------- |
| NVIDIA DGX-2 | Ubuntu 22.04 | rayon_sum | 5,175,824 ns/iter (+/- 2,767,386) | 250,119 ns/iter (+/- 21,970) | 20.69x            |

## License

SAS is distributed under the terms of both the MIT license and the
Apache License (Version 2.0). See [LICENSE-APACHE](LICENSE-APACHE) and
[LICENSE-MIT](LICENSE-MIT) for details. Opening a pull request is
assumed to signal agreement with these licensing terms.
