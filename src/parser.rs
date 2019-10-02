use std::io;
use std::mem;
use std::os::raw;
use std::slice;

use libyaml_sys as sys;

use crate::{Event, ParserError, ParserIter};

/// Parser.
///
/// # Iteration
///
/// `Box<Parser>` implements `IntoIterator<Item = Result<Event, ParserError>>`,
/// and thus can be used in `for` loops.  The iterator is fused, and produces
/// `None` forever after the end of stream or after a first encountered error.
pub struct Parser<'a> {
    inner: sys::yaml_parser_t,
    reader: Box<dyn io::Read + 'a>,
    reader_error: Option<io::Error>,
}

impl<'a> Parser<'a> {
    /// Create a parser with default configuration.
    ///
    /// If a custom configuration is needed, use [`ParserBuilder`] to build a
    /// `Parser` instance.
    ///
    /// [`ParserBuilder`]: struct.ParserBuilder.html
    pub fn new<R: io::Read + 'a>(reader: R) -> Result<Box<Self>, ParserError> {
        let mut inner = unsafe { mem::MaybeUninit::zeroed().assume_init() };

        if unsafe { sys::yaml_parser_initialize(&mut inner) } == 1 {
            let mut parser = Box::new(Self {
                inner,
                reader: Box::new(reader),
                reader_error: None,
            });

            unsafe {
                sys::yaml_parser_set_input(
                    &mut parser.inner,
                    Some(read_handler),
                    parser.as_mut() as *mut _ as *mut _,
                );
            }

            Ok(parser)
        } else {
            Err(ParserError::LibYamlError)
        }
    }

    /// Parse an event.
    pub fn parse(&mut self) -> Result<Event, ParserError> {
        let mut event = unsafe { mem::MaybeUninit::zeroed().assume_init() };

        if unsafe { sys::yaml_parser_parse(&mut self.inner, &mut event) } == 1 {
            debug_assert!(self.reader_error.is_none());
            Ok(Event::from_raw(event)?)
        } else {
            match mem::replace(&mut self.reader_error, None) {
                Some(e) => Err(ParserError::IoError(e)),
                None => Err(ParserError::LibYamlError),
            }
        }
    }

    /// Return raw pointer to the underlying `yaml_parser_t`.
    pub fn as_raw_ptr(&mut self) -> *mut sys::yaml_parser_t {
        &mut self.inner
    }
}

impl Drop for Parser<'_> {
    fn drop(&mut self) {
        unsafe {
            sys::yaml_parser_delete(&mut self.inner);
        }
    }
}

impl<'a> IntoIterator for Box<Parser<'a>> {
    type Item = <ParserIter<'a> as Iterator>::Item;
    type IntoIter = ParserIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ParserIter::new(self)
    }
}

unsafe extern fn read_handler(
    data: *mut raw::c_void,
    buffer: *mut raw::c_uchar,
    size: usize,
    size_read: *mut usize,
) -> raw::c_int {
    let parser = &mut *(data as *mut Parser);

    match parser.reader.read(slice::from_raw_parts_mut(buffer, size)) {
        Ok(n) => {
            *size_read = n;
            parser.reader_error = None;
            1
        },
        Err(e) => {
            *size_read = 0;
            parser.reader_error = Some(e);
            0
        },
    }
}
