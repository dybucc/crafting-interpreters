use std::{
    io::{self, BufReader, Cursor, Read},
    ops::Not,
};

use crate::{
    Error, ToError,
    tokenizer::{IoBound, Location, SyntaxError, SyntaxErrorContainer, Token},
};

#[derive(Debug)]
pub(crate) struct Scanner<'a> {
    pub(crate) buf: BufReader<Cursor<&'a [u8]>>,
    pub(crate) line: usize,
    pub(crate) col: usize,
}

impl<'a> Scanner<'a> {
    pub(crate) fn new(buf: &'a [u8]) -> Self {
        Self {
            buf: BufReader::new(Cursor::new(buf)),
            col: 0,
            line: 0,
        }
    }
}

impl Scanner<'_> {
    pub(crate) fn advance(&mut self, mut buf: [u8; 1]) -> Result<(), Error> {
        let Self {
            buf: source, line, ..
        } = self;

        if let Err(err) = source.read_exact(&mut buf)
            && matches!(err.kind(), io::ErrorKind::UnexpectedEof).not()
        {
            return Err(IoBound {
                inner: err.into(),
                line: *line,
            }
            .convert(None));
        }

        Ok(())
    }

    #[expect(unused, reason = "WIP.")]
    pub(crate) fn scan(&mut self) -> Result<Vec<Token>, Error> {
        let mut buf = [0; 1];
        let mut out = Vec::new();
        let mut errors = Vec::new();

        // NOTE: we don't care for panics here because it's all being read from an
        // in-memory buffer so no I/O-bound operations are taking place.
        loop {
            self.advance(buf)?;

            match &buf {
                b @ (b"(" | b")" | b"{" | b"}" | b"," | b"." | b"-" | b"+" | b";" | b"*") => {
                    out.push(Token::new(b, Location::new(self.line, self.col, 1)));
                    self.col += 1;
                }
                b"/" => todo!(),
                b"!" => todo!(),
                b"=" => todo!(),
                b">" => todo!(),
                b"<" => todo!(),
                _ => {
                    if errors.is_empty().not() {
                        let SyntaxError { repr } = errors.pop().unwrap();
                        // TODO: converge errors into one if their spans are
                        // similar. Repeat in a loop, extracting the last one
                        // from errors until noone remain or the spans don't
                        // overlap.
                    } else {
                        errors.push(SyntaxError {
                            repr: {
                                vec![Location {
                                    line: self.line,
                                    col: self.col,
                                    len: 1,
                                }]
                            },
                        });
                    }
                }
            }

            break;
        }

        Ok(out)
    }
}
