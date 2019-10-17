[![crates.io](https://img.shields.io/crates/v/libyaml.svg)](https://crates.io/crates/libyaml)
[![docs.rs](https://docs.rs/libyaml/badge.svg)](https://docs.rs/libyaml)

# libyaml

This Rust crate provides high-level bindings for the [LibYAML] library,
version 0.2.2.  It has the following limitations:

* the `yaml` library must be available on the system, no attempt is made to
  build it from source;
* the token and document APIs are currently not implemented.

[LibYAML]: https://github.com/yaml/libyaml

## Installation

First, compile LibYAML as a shared or static library and install it.  Then add
this crate to your `Cargo.toml`:

```toml
[dependencies]
libyaml = "0.1.0"
```

## License

This crate is licensed under the [MIT license].

[MIT license]: LICENSE
