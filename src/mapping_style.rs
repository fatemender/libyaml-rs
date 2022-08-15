use crate::sys;

/// Mapping style.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum MappingStyle {
    /// Block sequence style.
    Block = sys::YAML_BLOCK_MAPPING_STYLE as _,

    /// Flow sequence style.
    Flow = sys::YAML_FLOW_MAPPING_STYLE as _,
}

impl MappingStyle {
    /// Convert from `yaml_mapping_style_t`; `YAML_ANY_MAPPING_STYLE` becomes
    /// `None`.
    pub fn from_raw(raw: sys::yaml_mapping_style_t) -> Option<Self> {
        match raw {
            sys::YAML_BLOCK_MAPPING_STYLE => Some(Self::Block),
            sys::YAML_FLOW_MAPPING_STYLE => Some(Self::Flow),
            _ => None,
        }
    }

    /// Convert to `yaml_mapping_style_t`.
    pub fn into_raw(self) -> sys::yaml_mapping_style_t {
        match self {
            Self::Block => sys::YAML_BLOCK_MAPPING_STYLE,
            Self::Flow => sys::YAML_FLOW_MAPPING_STYLE,
        }
    }

    /// Convert to `yaml_mapping_style_t`; `None` becomes
    /// `YAML_ANY_MAPPING_STYLE`.
    pub fn option_into_raw(value: Option<Self>) -> sys::yaml_mapping_style_t {
        value.map_or(sys::YAML_ANY_MAPPING_STYLE, Self::into_raw)
    }
}
