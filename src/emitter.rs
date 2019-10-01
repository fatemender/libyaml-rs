use std::io;
use std::mem;
use std::os::raw;
use std::slice;

use libyaml_sys as sys;

use crate::{EmitterError, Event};

/// Emitter.
pub struct Emitter<'a> {
    inner: sys::yaml_emitter_t,
    must_free: bool,
    writer: Box<dyn io::Write + 'a>,
    writer_error: Option<io::Error>,
}

impl<'a> Emitter<'a> {
    /// Create an emitter with default configuration.
    ///
    /// If a custom configuration is needed, use [`EmitterBuilder`] to build an
    /// `Emitter` instance.
    ///
    /// [`EmitterBuilder`]: struct.EmitterBuilder.html
    pub fn new<W: io::Write + 'a>(writer: W) -> Result<Box<Self>, EmitterError> {
        let mut inner = unsafe { mem::MaybeUninit::zeroed().assume_init() };

        if unsafe { sys::yaml_emitter_initialize(&mut inner) } == 1 {
            let mut emitter = Box::new(Self {
                inner,
                must_free: true,
                writer: Box::new(writer),
                writer_error: None,
            });

            unsafe {
                sys::yaml_emitter_set_output(
                    &mut emitter.inner,
                    Some(write_handler),
                    emitter.as_mut() as *mut _ as *mut _,
                );
            }

            Ok(emitter)
        } else {
            Err(EmitterError::LibYamlError)
        }
    }

    /// Emit an event.
    pub fn emit(&mut self, event: Event) -> Result<(), EmitterError> {
        let ret = unsafe {
            sys::yaml_emitter_emit(
                &mut self.inner,
                event.into_raw(),
            )
        };

        if ret == 1 {
            assert!(self.writer_error.is_none());
            Ok(())
        } else {
            match mem::replace(&mut self.writer_error, None) {
                Some(e) => Err(EmitterError::IoError(e)),
                None => Err(EmitterError::LibYamlError),
            }
        }
    }

    /// Flush the emitter buffer to writer.
    pub fn flush(&mut self) -> Result<(), EmitterError> {
        if unsafe { sys::yaml_emitter_flush(&mut self.inner) } == 1 {
            Ok(())
        } else {
            Err(EmitterError::LibYamlError)
        }
    }

    /// Return raw pointer to the underlying `yaml_emitter_t`.
    pub fn as_raw_ptr(&mut self) -> *mut sys::yaml_emitter_t {
        &mut self.inner
    }
}

impl Drop for Emitter<'_> {
    fn drop(&mut self) {
        if self.must_free {
            unsafe {
                sys::yaml_emitter_delete(&mut self.inner)
            }
        }
    }
}

unsafe extern fn write_handler(
    data: *mut raw::c_void,
    buffer: *mut raw::c_uchar,
    size: usize,
) -> raw::c_int {
    let emitter = &mut *(data as *mut Emitter);

    emitter.writer_error = emitter.writer
        .write_all(slice::from_raw_parts(buffer, size))
        .err();

    if emitter.writer_error.is_none() { 1 } else { 0 }
}
