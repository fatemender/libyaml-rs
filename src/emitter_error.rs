use std::error;
use std::fmt;
use std::io;

use crate::EventError;

/// Error returned from [`Emitter`] methods.
///
/// [`Emitter`]: struct.Emitter.html
#[derive(Debug)]
pub enum EmitterError {
    /// I/O error.
    IoError(io::Error),

    /// LibYAML error.
    LibYamlError,
}

impl From<EventError> for EmitterError {
    fn from(_: EventError) -> Self {
        Self::LibYamlError
    }
}

impl fmt::Display for EmitterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "YAML emitter error")
    }
}

impl error::Error for EmitterError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::IoError(io_error) => Some(io_error),
            Self::LibYamlError => None,
        }
    }
}
