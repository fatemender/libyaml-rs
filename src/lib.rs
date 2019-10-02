//! High-level bindings for the [LibYAML] library.
//!
//! [LibYAML]: https://github.com/yaml/libyaml
//!
//! # Reading YAML
//! 
//! To read a YAML stream, use [`Parser`].  This example counts the number of
//! alias events in a stream.
//!
//! ```
//! # use std::io;
//! # use libyaml::*;
//! #
//! # fn doctest<R: io::Read>(reader: R) -> Result<(), ParserError> {
//! let alias_count = Parser::new(reader)?.into_iter().filter(|e| {
//!     if let Ok(Event::Alias { .. }) = e { true } else { false }
//! }).count();
//! # Ok(())
//! # }
//! ```
//!
//! [`Parser`]: struct.Parser.html
//!
//! # Writing YAML
//!
//! To write a YAML stream, use [`Emitter`].  This example writes a stream with
//! a single document consisting of a single scalar.
//!
//! ```
//! # use std::io;
//! # use libyaml::*;
//! #
//! # fn doctest<W: io::Write>(writer: W) -> Result<(), EmitterError> {
//! let mut emitter = Emitter::new(writer)?;
//!
//! emitter.emit(Event::StreamStart { encoding: None })?;
//! emitter.emit(Event::DocumentStart { implicit: true })?;
//! emitter.emit(Event::Scalar {
//!     anchor: None,
//!     tag: Some(tag::INT.to_string()),
//!     value: "42".to_string(),
//!     plain_implicit: false,
//!     quoted_implicit: false,
//!     style: None,
//! })?;
//! emitter.emit(Event::DocumentEnd { implicit: true })?;
//! emitter.emit(Event::StreamEnd)?;
//! # Ok(())
//! # }
//! ```
//!
//! [`Emitter`]: struct.Emitter.html

pub mod tag;

pub use self::emitter::Emitter;
pub use self::emitter_builder::EmitterBuilder;
pub use self::emitter_error::EmitterError;
pub use self::encoding::Encoding;
pub use self::event::Event;
pub use self::event_error::EventError;
pub use self::lib_version::lib_version;
pub use self::lib_version_string::lib_version_string;
pub use self::line_break::LineBreak;
pub use self::mapping_style::MappingStyle;
pub use self::parser::Parser;
pub use self::parser_builder::ParserBuilder;
pub use self::parser_error::ParserError;
pub use self::parser_iter::ParserIter;
pub use self::scalar_style::ScalarStyle;
pub use self::sequence_style::SequenceStyle;
pub use self::version_directive::VersionDirective;

mod emitter;
mod emitter_builder;
mod emitter_error;
mod encoding;
mod event;
mod event_error;
mod lib_version;
mod lib_version_string;
mod line_break;
mod mapping_style;
mod parser;
mod parser_builder;
mod parser_error;
mod parser_iter;
mod scalar_style;
mod sequence_style;
mod version_directive;
