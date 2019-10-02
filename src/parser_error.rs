use std::io;

use crate::EventError;

/// Error returned from [`Parser`] methods.
///
/// [`Parser`]: struct.Parser.html
#[derive(Debug)]
pub enum ParserError {
    /// I/O error.
    IoError(io::Error),

    /// LibYAML error.
    LibYamlError,
}

impl From<EventError> for ParserError {
    fn from(_: EventError) -> Self {
        Self::LibYamlError
    }
}
