use libyaml_sys as sys;

/// Document version directive.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct VersionDirective(pub u8, pub u8);

impl VersionDirective {
    /// Convert from `yaml_version_directive_t`.
    pub fn from_raw(raw: sys::yaml_version_directive_t) -> Self {
        Self(raw.major as _, raw.minor as _)
    }

    /// Convert to `yaml_version_directive_t`.
    pub fn into_raw(self) -> sys::yaml_version_directive_t {
        sys::yaml_version_directive_t {
            major: self.0 as _,
            minor: self.1 as _,
        }
    }
}
