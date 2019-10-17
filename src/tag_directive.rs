/// Document tag directive.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TagDirective {
    /// Handle.
    pub handle: String,

    /// Prefix.
    pub prefix: String,
}
