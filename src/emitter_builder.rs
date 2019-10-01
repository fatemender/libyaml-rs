use std::io;

use libyaml_sys as sys;

use crate::{Emitter, EmitterError, Encoding};

/// Builder for emitters.
pub struct EmitterBuilder<'a> {
    emitter: Box<Emitter<'a>>,
}

impl<'a> EmitterBuilder<'a> {
    /// Start building an emitter.
    pub fn new<W: io::Write + 'a>(writer: W) -> Result<Self, EmitterError> {
        Emitter::new(writer).map(|emitter| Self { emitter })
    }

    /// Finish building an emitter.
    pub fn finish(self) -> Box<Emitter<'a>> {
        self.emitter
    }

    /// Enable or disable canonical output.
    pub fn canonical(mut self, enable: bool) -> Self {
        unsafe {
            sys::yaml_emitter_set_canonical(
                self.emitter.as_raw_ptr(),
                enable as _,
            )
        }

        self
    }

    /// Set encoding.
    pub fn encoding(mut self, encoding: Encoding) -> Self {
        unsafe {
            sys::yaml_emitter_set_encoding(
                self.emitter.as_raw_ptr(),
                encoding.into_raw(),
            )
        }

        self
    }

    /// Set indentation increment.
    pub fn indent(mut self, indent: usize) -> Self {
        unsafe {
            sys::yaml_emitter_set_indent(
                self.emitter.as_raw_ptr(),
                indent as _,
            )
        }

        self
    }

    /// Set preferred line width.
    pub fn line_width(mut self, width: usize) -> Self {
        unsafe {
            sys::yaml_emitter_set_width(
                self.emitter.as_raw_ptr(),
                width as _,
            )
        }

        self
    }

    /// Enable or disable unescaped non-ASCII characters in output.
    pub fn unicode(mut self, enable: bool) -> Self {
        unsafe {
            sys::yaml_emitter_set_unicode(
                self.emitter.as_raw_ptr(),
                enable as _,
            )
        }

        self
    }

    // TODO: line break.
}
