use std::error;
use std::fmt;
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

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "YAML parser error")
    }
}

impl error::Error for ParserError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::IoError(io_error) => Some(io_error),
            Self::LibYamlError => None,
        }
    }
}
