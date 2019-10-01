# libyaml

This Rust crate provides high-level bindings for the [LibYAML] library,
version 0.2.2.  It has the following limitations:

* the `yaml` library must be available on the system, no attempt is made to
  build it from source.

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
