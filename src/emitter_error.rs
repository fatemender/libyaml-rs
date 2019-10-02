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
