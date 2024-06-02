# embedded-utils

`embedded-utils` provides some features in Rust(no_std) embedded systems.

## Crate Feature Flags

The following crate [feature flags](https://doc.rust-lang.org/cargo/reference/features.html#the-features-section) are
available:

- `time`: enable processing time.

## Usage

1. Add the following line to your `Cargo.toml`:

```toml
embedded-utils = { version = "0.1.0" }
```

2. Use `time` for processing time.:

```rust
use embedded_utils::time::{DateTime, TimeZone};

let datetime = DateTime::from_unix_millis(1704067199998, TimeZone::AsiaShanghai);
info!("datetime year is {}", datetime.year);
```

## License

Licensed under either of

- Apache License, Version 2.0. [LICENSE-APACHE](LICENSE-APACHE)
  or [Apache-2.0 license](http://apache.org/licenses/LICENSE-2.0)
- MIT license. [LICENSE-MIT](LICENSE-MIT) or [MIT license](http://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
