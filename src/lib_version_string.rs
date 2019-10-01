use std::ffi::CStr;

use libyaml_sys as sys;

/// Return LibYAML version as a string.
pub fn lib_version_string() -> String {
    unsafe { CStr::from_ptr(sys::yaml_get_version_string()) }
        .to_string_lossy()
        .into_owned()
}
