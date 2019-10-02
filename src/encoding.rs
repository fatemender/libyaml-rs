use libyaml_sys as sys;

/// Stream encoding.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Encoding {
    /// UTF-8.
    Utf8 = sys::YAML_UTF8_ENCODING as _,

    /// UTF-16-LE with BOM.
    Utf16Le = sys::YAML_UTF16LE_ENCODING as _,

    /// UTF-16-BE with BOM.
    Utf16Be = sys::YAML_UTF16BE_ENCODING as _,
}

impl Encoding {
    /// Convert from `yaml_encoding_t`; `YAML_ANY_ENCODING` becomes `None`.
    pub fn from_raw(raw: sys::yaml_encoding_t) -> Option<Self> {
        match raw {
            sys::YAML_UTF8_ENCODING => Some(Self::Utf8),
            sys::YAML_UTF16LE_ENCODING => Some(Self::Utf16Le),
            sys::YAML_UTF16BE_ENCODING => Some(Self::Utf16Be),
            _ => None,
        }
    }

    /// Convert to `yaml_encoding_t`.
    pub fn into_raw(self) -> sys::yaml_encoding_t {
        self as _
    }

    /// Convert to `yaml_encoding_t`; `None` becomes `YAML_ANY_ENCODING`.
    pub fn option_into_raw(value: Option<Self>) -> sys::yaml_encoding_t {
        value.map_or(sys::YAML_ANY_ENCODING, Self::into_raw)
    }
}
