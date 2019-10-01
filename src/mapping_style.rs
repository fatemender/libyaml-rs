use libyaml_sys as sys;

/// Mapping style.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum MappingStyle {
    /// Block sequence style.
    Block = sys::YAML_BLOCK_MAPPING_STYLE as _,

    /// Flow sequence style.
    Flow = sys::YAML_FLOW_MAPPING_STYLE as _,
}

impl MappingStyle {
    /// Return the raw `yaml_mapping_style_t`.
    pub fn into_raw(self) -> sys::yaml_mapping_style_t {
        self as _
    }

    /// Return the raw `yaml_mapping_style_t` where `None` becomes
    /// `YAML_ANY_MAPPING_STYLE`.
    pub fn option_into_raw(value: Option<Self>) -> sys::yaml_mapping_style_t {
        value.map_or(sys::YAML_ANY_MAPPING_STYLE, Self::into_raw)
    }
}
