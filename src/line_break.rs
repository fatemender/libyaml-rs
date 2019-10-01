use libyaml_sys as sys;

/// Line break encoding.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum LineBreak {
    /// CR character (classic Mac).
    Cr = sys::YAML_CR_BREAK as _,

    /// LN character (UNIX).
    Ln = sys::YAML_LN_BREAK as _,

    /// CR LN characters (DOS).
    CrLn = sys::YAML_CRLN_BREAK as _,
}

impl LineBreak {
    /// Return the raw `yaml_break_t`.
    pub fn into_raw(self) -> sys::yaml_break_t {
        self as _
    }

    /// Return the raw `yaml_break_t` where `None` becomes `YAML_ANY_BREAK`.
    pub fn option_into_raw(value: Option<Self>) -> sys::yaml_break_t {
        value.map_or(sys::YAML_ANY_BREAK, Self::into_raw)
    }
}
