# `semver-macro`

[![crates.io](https://img.shields.io/crates/v/semver-macro.svg)](https://crates.io/crates/semver-macro)
[![Documentation](https://docs.rs/semver-macro/badge.svg)](https://docs.rs/semver-macro)
[![MIT License](https://img.shields.io/crates/l/semver-macro.svg)](./LICENSE)

This crate contains a macros to parse [`semver::Version`][1]s at compile-time.
Currently only versions without pre-release and build metadata identifier
are supported.

## Example

```rust
use semver::Version;
use semver_macro::version;

const MY_VERSION: Version = version!("0.1.0");
```

[1]: https://docs.rs/semver/latest/semver/struct.Version.html
