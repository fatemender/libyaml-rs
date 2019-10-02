use std::iter::FusedIterator;

use crate::{Event, Parser, ParserError};

/// Iterator adapter for [`Parser`].
///
/// [`Parser`]: struct.Parser.html
pub struct ParserIter<'a> {
    parser: Box<Parser<'a>>,
    fuse_burnt: bool,
}

impl<'a> ParserIter<'a> {
    /// Convert a parser into an iterator.
    pub fn new(parser: Box<Parser<'a>>) -> Self {
        Self { parser, fuse_burnt: false }
    }
}

impl Iterator for ParserIter<'_> {
    type Item = Result<Event, ParserError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.fuse_burnt {
            None
        } else {
            match self.parser.parse() {
                ret @ Ok(Event::StreamEnd) | ret @ Err(_) => {
                    self.fuse_burnt = true;
                    Some(ret)
                },
                ret => {
                    Some(ret)
                },
            }
        }
    }
}

impl FusedIterator for ParserIter<'_> {
}
