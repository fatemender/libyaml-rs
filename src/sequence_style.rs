use libyaml_sys as sys;

/// Sequence style.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum SequenceStyle {
    /// Block sequence style.
    Block = sys::YAML_BLOCK_SEQUENCE_STYLE as _,

    /// Flow sequence style.
    Flow = sys::YAML_FLOW_SEQUENCE_STYLE as _,
}

impl SequenceStyle {
    /// Convert from `yaml_sequence_style_t`; `YAML_ANY_SEQUENCE_STYLE` becomes
    /// `None`.
    pub fn from_raw(raw: sys::yaml_sequence_style_t) -> Option<Self> {
        match raw {
            sys::YAML_BLOCK_SEQUENCE_STYLE => Some(Self::Block),
            sys::YAML_FLOW_SEQUENCE_STYLE => Some(Self::Flow),
            _ => None,
        }
    }

    /// Convert to `yaml_sequence_style_t`.
    pub fn into_raw(self) -> sys::yaml_sequence_style_t {
        self as _
    }

    /// Convert to `yaml_sequence_style_t`; `None` becomes
    /// `YAML_ANY_SEQUENCE_STYLE`.
    pub fn option_into_raw(value: Option<Self>) -> sys::yaml_sequence_style_t {
        value.map_or(sys::YAML_ANY_SEQUENCE_STYLE, Self::into_raw)
    }
}
