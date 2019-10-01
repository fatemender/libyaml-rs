//! High-level bindings for the [LibYAML] library.
//!
//! [LibYAML]: https://github.com/yaml/libyaml

pub mod tag;

pub use self::emitter::Emitter;
pub use self::emitter_builder::EmitterBuilder;
pub use self::emitter_error::EmitterError;
pub use self::encoding::Encoding;
pub use self::event::Event;
pub use self::event_error::EventError;
pub use self::lib_version::lib_version;
pub use self::lib_version_string::lib_version_string;
pub use self::mapping_style::MappingStyle;
pub use self::scalar_style::ScalarStyle;
pub use self::sequence_style::SequenceStyle;

mod emitter;
mod emitter_builder;
mod emitter_error;
mod encoding;
mod event;
mod event_error;
mod lib_version;
mod lib_version_string;
mod mapping_style;
mod scalar_style;
mod sequence_style;
