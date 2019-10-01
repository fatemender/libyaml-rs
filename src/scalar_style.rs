use libyaml_sys as sys;

/// Scalar style.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum ScalarStyle {
    /// Plain scalar style.
    Plain = sys::YAML_PLAIN_SCALAR_STYLE as _,

    /// Single-quoted scalar style.
    SingleQuoted = sys::YAML_SINGLE_QUOTED_SCALAR_STYLE as _,

    /// Double-quoted scalar style.
    DoubleQuoted = sys::YAML_DOUBLE_QUOTED_SCALAR_STYLE as _,

    /// Literal scalar style.
    Literal = sys::YAML_LITERAL_SCALAR_STYLE as _,

    /// Folded scalar style.
    Folded = sys::YAML_FOLDED_SCALAR_STYLE as _,
}

impl ScalarStyle {
    /// Return the raw `yaml_scalar_style_t`.
    pub fn into_raw(self) -> sys::yaml_scalar_style_t {
        self as _
    }

    /// Return the raw `yaml_scalar_style_t` where `None` becomes
    /// `YAML_ANY_SCALAR_STYLE`.
    pub fn option_into_raw(value: Option<Self>) -> sys::yaml_scalar_style_t {
        value.map_or(sys::YAML_ANY_SCALAR_STYLE, Self::into_raw)
    }
}
