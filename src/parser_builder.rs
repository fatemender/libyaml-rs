use std::io;

use libyaml_sys as sys;

use crate::{Encoding, Parser, ParserError};

/// Builder for parsers.
pub struct ParserBuilder<'a> {
    parser: Box<Parser<'a>>,
}

impl<'a> ParserBuilder<'a> {
    /// Start building a parser.
    pub fn new<R: io::Read + 'a>(reader: R) -> Result<Self, ParserError> {
        Parser::new(reader).map(|parser| Self { parser })
    }

    /// Finish building a parser.
    pub fn finish(self) -> Box<Parser<'a>> {
        self.parser
    }

    /// Set encoding.
    pub fn encoding(mut self, encoding: Encoding) -> Self {
        unsafe {
            sys::yaml_parser_set_encoding(
                self.parser.as_raw_ptr(),
                encoding.into_raw(),
            );
        }

        self
    }
}
