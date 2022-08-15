[![crates.io](https://img.shields.io/crates/v/libyaml.svg)](https://crates.io/crates/libyaml)
[![docs.rs](https://docs.rs/libyaml/badge.svg)](https://docs.rs/libyaml)

# libyaml

This Rust crate provides high-level bindings for the [LibYAML] library via the
[`unsafe-libyaml`] crate.  It has the following limitations:

* the token and document APIs are currently not implemented.

[LibYAML]: https://github.com/yaml/libyaml
[`unsafe-libyaml`]: https://github.com/dtolnay/unsafe-libyaml

## Installation

Just add this crate to your `Cargo.toml`:

```toml
[dependencies]
libyaml = "0.2"
```

You do not need to install the LibYAML library on the target system.  Instead,
`unsafe-libyaml` provides a transpiled version.

## License

This crate is licensed under the [MIT license].

[MIT license]: LICENSE
