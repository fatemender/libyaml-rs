use std::ffi;
use std::mem;
use std::os::raw;
use std::ptr;

use libyaml_sys as sys;

use crate::{Encoding, EventError, MappingStyle, ScalarStyle, SequenceStyle};
use crate::{VersionDirective};

/// Emitter or parser event.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Event {
    /// A *STREAM-START* event.
    StreamStart {
        /// Stream encoding; if `None`, LibYAML will choose an encoding.
        encoding: Option<Encoding>,
    },

    /// A *STREAM-END* event.
    StreamEnd,

    /// A *DOCUMENT-START* event.
    DocumentStart {
        /// Optional version directive.
        version: Option<VersionDirective>,

        /// If true, no document start marker will be emitted.
        implicit: bool,
    },

    /// A *DOCUMENT-END* event.
    DocumentEnd {
        /// If true, no document end marker will be emitted.
        implicit: bool,
    },

    /// An *ALIAS* event.
    Alias {
        /// Target anchor name.
        anchor: String,
    },

    /// A *SCALAR* event.
    Scalar {
        /// Optional anchor name.
        anchor: Option<String>,

        /// Optional tag name.
        tag: Option<String>,

        /// Scalar value.
        value: String,

        /// If true, no tag will be emitted for the plain style.
        plain_implicit: bool,

        /// If true, no tag will be emitted for the non-plain styles.
        quoted_implicit: bool,

        /// Scalar style; if `None`, LibYAML will choose a style.
        style: Option<ScalarStyle>,
    },

    /// A *SEQUENCE-START* event.
    SequenceStart {
        /// Optional anchor name.
        anchor: Option<String>,

        /// Optional tag name.
        tag: Option<String>,

        /// If true, no tag will be emitted.
        implicit: bool,

        /// Sequence style; if `None`, LibYAML will choose a style.
        style: Option<SequenceStyle>,
    },

    /// A *SEQUENCE-END* event.
    SequenceEnd,

    /// A *MAPPING-START* event.
    MappingStart {
        /// Optional anchor name.
        anchor: Option<String>,

        /// Optional tag name.
        tag: Option<String>,

        /// If true, no tag will be emitted.
        implicit: bool,

        /// Mapping style; if `None`, LibYAML will choose a style.
        style: Option<MappingStyle>,
    },

    /// A *MAPPING-END* event.
    MappingEnd,
}

impl Event {
    /// Take ownership of a raw `yaml_event_t`.  This method frees the allocated
    /// memory, even if the conversion fails.
    pub fn from_raw(mut raw: sys::yaml_event_t) -> Result<Self, EventError> {
        fn from_raw_cstr(ptr: *const raw::c_char) -> Option<String> {
            if ptr.is_null() {
                None
            } else {
                Some(from_raw_cstr_non_null(ptr))
            }
        }

        fn from_raw_cstr_non_null(ptr: *const raw::c_char) -> String {
            unsafe { ffi::CStr::from_ptr(ptr) }.to_string_lossy().into_owned()
        }

        let ret = unsafe {
            match raw.type_ {
                sys::YAML_STREAM_START_EVENT => {
                    Ok(Self::StreamStart {
                        encoding: Encoding::from_raw(raw.data.stream_start.as_ref().encoding),
                    })
                },
                sys::YAML_STREAM_END_EVENT => {
                    Ok(Self::StreamEnd)
                },
                sys::YAML_DOCUMENT_START_EVENT => {
                    let version_ptr = raw.data.document_start.as_ref().version_directive;

                    Ok(Self::DocumentStart {
                        version: if version_ptr.is_null() {
                            None
                        } else {
                            Some(VersionDirective::from_raw(*version_ptr))
                        },
                        implicit: raw.data.document_start.as_ref().implicit != 0,
                    })

                    // TODO: handle the remaining attributes.
                },
                sys::YAML_DOCUMENT_END_EVENT => {
                    Ok(Self::DocumentEnd {
                        implicit: raw.data.document_end.as_ref().implicit != 0,
                    })
                },
                sys::YAML_ALIAS_EVENT => {
                    Ok(Self::Alias {
                        anchor: from_raw_cstr_non_null(raw.data.alias.as_ref().anchor as *const _),
                    })
                },
                sys::YAML_SCALAR_EVENT => {
                    Ok(Self::Scalar {
                        anchor: from_raw_cstr(raw.data.scalar.as_ref().anchor as *const _),
                        tag: from_raw_cstr(raw.data.scalar.as_ref().tag as *const _),
                        value: from_raw_cstr_non_null(raw.data.scalar.as_ref().value as *const _),
                        plain_implicit: raw.data.scalar.as_ref().plain_implicit != 0,
                        quoted_implicit: raw.data.scalar.as_ref().quoted_implicit != 0,
                        style: ScalarStyle::from_raw(raw.data.scalar.as_ref().style),
                    })
                },
                sys::YAML_SEQUENCE_START_EVENT => {
                    Ok(Self::SequenceStart {
                        anchor: from_raw_cstr(raw.data.sequence_start.as_ref().anchor as *const _),
                        tag: from_raw_cstr(raw.data.sequence_start.as_ref().tag as *const _),
                        implicit: raw.data.sequence_start.as_ref().implicit != 0,
                        style: SequenceStyle::from_raw(raw.data.sequence_start.as_ref().style),
                    })
                },
                sys::YAML_SEQUENCE_END_EVENT => {
                    Ok(Self::SequenceEnd)
                },
                sys::YAML_MAPPING_START_EVENT => {
                    Ok(Self::MappingStart {
                        anchor: from_raw_cstr(raw.data.mapping_start.as_ref().anchor as *const _),
                        tag: from_raw_cstr(raw.data.mapping_start.as_ref().tag as *const _),
                        implicit: raw.data.mapping_start.as_ref().implicit != 0,
                        style: MappingStyle::from_raw(raw.data.mapping_start.as_ref().style),
                    })
                },
                sys::YAML_MAPPING_END_EVENT => {
                    Ok(Self::MappingEnd)
                },
                _ => {
                    Err(EventError)
                },
            }
        };

        unsafe {
            sys::yaml_event_delete(&mut raw);
        }

        ret
    }

    /// Return the raw `yaml_event_t` for this event.  The caller is responsible
    /// for freeing memory.
    pub fn into_raw(self) -> Result<sys::yaml_event_t, EventError> {
        unsafe {
            let mut event = mem::MaybeUninit::zeroed().assume_init();

            let ret = match self {
                Self::StreamStart { encoding } => {
                    sys::yaml_stream_start_event_initialize(
                        &mut event,
                        Encoding::option_into_raw(encoding),
                    )
                },
                Self::StreamEnd => {
                    sys::yaml_stream_end_event_initialize(
                        &mut event,
                    )
                },
                Self::DocumentStart { version, implicit } => {
                    let version = version.map(VersionDirective::into_raw);
                    let version_ptr = version.as_ref().map_or(ptr::null(), |v| v);

                    sys::yaml_document_start_event_initialize(
                        &mut event,
                        version_ptr as *mut _,
                        ptr::null_mut(),
                        ptr::null_mut(),
                        implicit as _,
                    )

                    // TODO: handle the remaining attributes.
                },
                Self::DocumentEnd { implicit } => {
                    sys::yaml_document_end_event_initialize(
                        &mut event,
                        implicit as _,
                    )
                },
                Self::Alias { anchor } => {
                    let anchor = ffi::CString::new(anchor)?;

                    sys::yaml_alias_event_initialize(
                        &mut event,
                        anchor.as_ptr() as *mut _,
                    )
                },
                Self::Scalar { anchor, tag, value, plain_implicit, quoted_implicit, style } => {
                    let anchor = anchor.map(ffi::CString::new).transpose()?;
                    let tag = tag.map(ffi::CString::new).transpose()?;
                    let value = ffi::CString::new(value)?;

                    sys::yaml_scalar_event_initialize(
                        &mut event,
                        anchor.as_ref().map_or(ptr::null(), |cs| cs.as_ptr()) as *mut _,
                        tag.as_ref().map_or(ptr::null(), |cs| cs.as_ptr()) as *mut _,
                        value.as_ptr() as *mut _,
                        value.as_bytes().len() as _,
                        plain_implicit as _,
                        quoted_implicit as _,
                        ScalarStyle::option_into_raw(style),
                    )
                },
                Self::SequenceStart { anchor, tag, implicit, style } => {
                    let anchor = anchor.map(ffi::CString::new).transpose()?;
                    let tag = tag.map(ffi::CString::new).transpose()?;

                    sys::yaml_sequence_start_event_initialize(
                        &mut event,
                        anchor.as_ref().map_or(ptr::null(), |cs| cs.as_ptr()) as *mut _,
                        tag.as_ref().map_or(ptr::null(), |cs| cs.as_ptr()) as *mut _,
                        implicit as _,
                        SequenceStyle::option_into_raw(style),
                    )
                },
                Self::SequenceEnd => {
                    sys::yaml_sequence_end_event_initialize(
                        &mut event,
                    )
                },
                Self::MappingStart { anchor, tag, implicit, style } => {
                    let anchor = anchor.map(ffi::CString::new).transpose()?;
                    let tag = tag.map(ffi::CString::new).transpose()?;

                    sys::yaml_mapping_start_event_initialize(
                        &mut event,
                        anchor.as_ref().map_or(ptr::null(), |cs| cs.as_ptr()) as *mut _,
                        tag.as_ref().map_or(ptr::null(), |cs| cs.as_ptr()) as *mut _,
                        implicit as _,
                        MappingStyle::option_into_raw(style),
                    )
                },
                Self::MappingEnd => {
                    sys::yaml_mapping_end_event_initialize(
                        &mut event,
                    )
                },
            };

            if ret == 1 { Ok(event) } else { Err(EventError) }
        }
    }
}
