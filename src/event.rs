use std::ffi;
use std::mem;
use std::ptr;

use libyaml_sys as sys;

use crate::{Encoding, EventError, EventType, MappingStyle, ScalarStyle, SequenceStyle};

/// Emitter or parser event.
pub struct Event {
    inner: Box<sys::yaml_event_t>,
}

impl Event {
    /// Create an empty event.
    pub fn new() -> Self {
        Self {
            inner: Box::new(unsafe { mem::MaybeUninit::zeroed().assume_init() }),
        }
    }

    /// Create a *STREAM-START* event.
    ///
    /// * If `encoding` is `None`, LibYAML will choose an encoding.
    pub fn new_stream_start(encoding: Option<Encoding>) -> Result<Self, EventError> {
        let mut event = Self::new();

        let ret = unsafe {
            sys::yaml_stream_start_event_initialize(
                event.inner.as_mut(),
                Encoding::option_into_raw(encoding),
            )
        };

        if ret == 1 { Ok(event) } else { Err(EventError) }
    }

    /// Create a *STREAM-END* event.
    pub fn new_stream_end() -> Result<Self, EventError> {
        let mut event = Self::new();

        let ret = unsafe {
            sys::yaml_stream_end_event_initialize(
                event.inner.as_mut(),
            )
        };

        if ret == 1 { Ok(event) } else { Err(EventError) }
    }

    /// Create a *DOCUMENT-START* event.
    ///
    /// * If `implicit` is `true`, no document start mark is to be emitted.
    pub fn new_document_start(implicit: bool) -> Result<Self, EventError> {
        let mut event = Self::new();

        let ret = unsafe {
            sys::yaml_document_start_event_initialize(
                event.inner.as_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                implicit as _,
            )

            // TODO: all other arguments.
        };

        if ret == 1 { Ok(event) } else { Err(EventError) }
    }

    /// Create a *DOCUMENT-END* event.
    ///
    /// * If `implicit` is `true`, no document end mark is to be emitted.
    pub fn new_document_end(implicit: bool) -> Result<Self, EventError> {
        let mut event = Self::new();

        let ret = unsafe {
            sys::yaml_document_end_event_initialize(
                event.inner.as_mut(),
                implicit as _,
            )
        };

        if ret == 1 { Ok(event) } else { Err(EventError) }
    }

    /// Create an *ALIAS* event.
    pub fn new_alias(anchor: &str) -> Result<Self, EventError> {
        let mut event = Self::new();

        let anchor_cs = ffi::CString::new(anchor)?;

        let ret = unsafe {
            sys::yaml_alias_event_initialize(
                event.inner.as_mut(),
                anchor_cs.as_ptr() as *mut _,
            )
        };

        if ret == 1 { Ok(event) } else { Err(EventError) }
    }

    /// Create a *SCALAR* event.
    ///
    /// * If `plain_implicit` is `true`, the tag is optional for the plain
    ///   style.
    /// * If `quoted_implicit` is `true`, the tag is optional for non-plain
    ///   styles.
    /// * If `style` is `None`, LibYAML will choose a style.
    pub fn new_scalar(
        anchor: Option<&str>,
        tag: Option<&str>,
        value: &str,
        plain_implicit: bool,
        quoted_implicit: bool,
        style: Option<ScalarStyle>,
    ) -> Result<Self, EventError> {
        let mut event = Self::new();

        let anchor_cs = anchor.map(ffi::CString::new).transpose()?;
        let tag_cs = tag.map(ffi::CString::new).transpose()?;
        let value_cs = ffi::CString::new(value)?;

        let ret = unsafe {
            sys::yaml_scalar_event_initialize(
                event.inner.as_mut(),
                anchor_cs.as_ref().map_or(ptr::null(), |cs| cs.as_ptr()) as *mut _,
                tag_cs.as_ref().map_or(ptr::null(), |cs| cs.as_ptr()) as *mut _,
                value_cs.as_ptr() as *mut _,
                value_cs.as_bytes().len() as _,
                plain_implicit as _,
                quoted_implicit as _,
                ScalarStyle::option_into_raw(style),
            )
        };

        if ret == 1 { Ok(event) } else { Err(EventError) }
    }

    /// Create a *SEQUENCE-START* event.
    ///
    /// * If `implicit` is `true`, the tag is optional.
    /// * If `style` is `None`, LibYAML will choose a style.
    pub fn new_sequence_start(
        anchor: Option<&str>,
        tag: Option<&str>,
        implicit: bool,
        style: Option<SequenceStyle>,
    ) -> Result<Self, EventError> {
        let mut event = Self::new();

        let anchor_cs = anchor.map(ffi::CString::new).transpose()?;
        let tag_cs = tag.map(ffi::CString::new).transpose()?;

        let ret = unsafe {
            sys::yaml_sequence_start_event_initialize(
                event.inner.as_mut(),
                anchor_cs.as_ref().map_or(ptr::null(), |cs| cs.as_ptr()) as *mut _,
                tag_cs.as_ref().map_or(ptr::null(), |cs| cs.as_ptr()) as *mut _,
                implicit as _,
                SequenceStyle::option_into_raw(style),
            )
        };

        if ret == 1 { Ok(event) } else { Err(EventError) }
    }

    /// Create a *SEQUENCE-END* event.
    pub fn new_sequence_end() -> Result<Self, EventError> {
        let mut event = Self::new();

        let ret = unsafe {
            sys::yaml_sequence_end_event_initialize(
                event.inner.as_mut(),
            )
        };

        if ret == 1 { Ok(event) } else { Err(EventError) }
    }

    /// Create a *MAPPING-START* event.
    ///
    /// * If `implicit` is `true`, the tag is optional.
    /// * If `style` is `None`, LibYAML will choose a style.
    pub fn new_mapping_start(
        anchor: Option<&str>,
        tag: Option<&str>,
        implicit: bool,
        style: Option<MappingStyle>,
    ) -> Result<Self, EventError> {
        let mut event = Self::new();

        let anchor_cs = anchor.map(ffi::CString::new).transpose()?;
        let tag_cs = tag.map(ffi::CString::new).transpose()?;

        let ret = unsafe {
            sys::yaml_mapping_start_event_initialize(
                event.inner.as_mut(),
                anchor_cs.as_ref().map_or(ptr::null(), |cs| cs.as_ptr()) as *mut _,
                tag_cs.as_ref().map_or(ptr::null(), |cs| cs.as_ptr()) as *mut _,
                implicit as _,
                MappingStyle::option_into_raw(style),
            )
        };

        if ret == 1 { Ok(event) } else { Err(EventError) }
    }

    /// Create a *MAPPING-END* event.
    pub fn new_mapping_end() -> Result<Self, EventError> {
        let mut event = Self::new();

        let ret = unsafe {
            sys::yaml_mapping_end_event_initialize(
                event.inner.as_mut(),
            )
        };

        if ret == 1 { Ok(event) } else { Err(EventError) }
    }

    /// Return the event type.
    pub fn type_(&self) -> Option<EventType> {
        EventType::from_raw(self.inner.type_)
    }

    /// Return raw pointer to the underlying `yaml_event_t`, consuming this
    /// structure.
    pub fn into_raw(mut self) -> *mut sys::yaml_event_t {
        Box::into_raw(mem::replace(
            &mut self.inner,
            Box::new(unsafe { mem::MaybeUninit::zeroed().assume_init() }),
        ))
    }

    /// Return raw pointer to the underlying `yaml_event_t`.
    pub fn as_raw_ptr(&mut self) -> *mut sys::yaml_event_t {
        &mut *self.inner
    }
}

impl Default for Event {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        unsafe {
            sys::yaml_event_delete(self.inner.as_mut());
        }
    }
}
