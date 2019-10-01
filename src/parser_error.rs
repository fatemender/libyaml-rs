use std::io;

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
