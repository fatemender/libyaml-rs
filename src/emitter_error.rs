use std::io;

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
