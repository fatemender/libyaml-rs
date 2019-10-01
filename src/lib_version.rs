use std::os::raw::c_int;

use libyaml_sys as sys;

/// Return LibYAML version as a `(major, minor, patch)` tuple.
pub fn lib_version() -> (u64, u64, u64) {
    let mut major: c_int = 0;
    let mut minor: c_int = 0;
    let mut patch: c_int = 0;

    unsafe {
        sys::yaml_get_version(&mut major, &mut minor, &mut patch);
    }

    (major as _, minor as _, patch as _)
}
