use std::ffi;

/// Error returned from [`Event`] methods.
///
/// [`Event`]: enum.Event.html
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EventError;

impl From<ffi::NulError> for EventError {
    fn from(_: ffi::NulError) -> Self {
        Self
    }
}
