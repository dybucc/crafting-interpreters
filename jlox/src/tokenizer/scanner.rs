use std::io::{self, BufReader, Read};

use crate::{
    Error, ToError,
    tokenizer::{IoBound, Location, SyntaxError, Token},
};

#[derive(Debug)]
pub(crate) struct Scanner<'a> {
    pub(crate) buf: BufReader<&'a [u8]>,
    pub(crate) line: usize,
    pub(crate) col: usize,
}

impl<'a> Scanner<'a> {
    pub(crate) fn new(buf: &'a [u8]) -> Self {
        Self {
            buf: BufReader::new(buf),
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

        match source.read_exact(&mut buf) {
            Err(err) if !matches!(err.kind(), io::ErrorKind::UnexpectedEof) => {
                return Err(IoBound {
                    inner: err.into(),
                    line: *line,
                }
                .convert(None));
            }
            _ => (),
        }

        Ok(())
    }

    pub(crate) fn scan(&mut self) -> Result<Vec<Token>, Error> {
        let Self {
            buf: source,
            line,
            col,
        } = self;

        let mut buf = [0; 1];
        let mut out = Vec::new();

        // NOTE: we don't care for panics here because it's all being read from an
        // in-memory buffer so no I/O-bound operations are taking place.
        loop {
            match source.read_exact(&mut buf) {
                Err(err) if matches!(err.kind(), io::ErrorKind::UnexpectedEof) => break,
                Err(err) => {
                    return Err(IoBound {
                        inner: err.into(),
                        line: *line,
                    }
                    .convert(None));
                }
                Ok(()) => (),
            }
        }

        for byte in source.bytes().map(Result::unwrap) {
            match byte {
                b @ (b'(' | b')' | b'{' | b'}' | b',' | b'.' | b'-' | b'+' | b';' | b'*') => {
                    out.push(Token::new(&[b], Location::new(*line, *col, 1)));
                    *col += 1;
                }
                b @ b'/' => {}
                b'!' => todo!(),
                b'=' => todo!(),
                b'>' => todo!(),
                b'<' => todo!(),
                _ => {
                    return Err(SyntaxError {
                        line: *line,
                        col: *col,
                        expect: None,
                    }
                    .convert(None));
                }
            }
        }

        Ok(out)
    }
}
